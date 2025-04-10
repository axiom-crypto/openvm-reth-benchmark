use std::collections::HashSet;

use axum::{extract::Query, response::IntoResponse};
use pprof::protos::Profile;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProfileParams {
    seconds: Option<u32>,
}

pub async fn start_profile_server(pprof_data: Vec<u8>) -> eyre::Result<()> {
    let app = axum::Router::new().route(
        "/debug/pprof/profile",
        axum::routing::get(|query: Query<ProfileParams>| async move {
            let _ = query.seconds.unwrap_or(10);
            let bytes = axum::body::Bytes::from(pprof_data);
            bytes.into_response()
        }),
    );

    println!("Starting profile server on port 3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/// Substrings that should be filtered out from the profile.
const UNWANTED_SUBSTRINGS: [&str; 6] = ["rayon", "std", "call_once", "tokio", "core::", "alloc::"];

fn is_allowed(function_name: &str) -> bool {
    UNWANTED_SUBSTRINGS.iter().all(|&sub| !function_name.contains(sub))
}

fn get_function_name<'a>(profile: &'a Profile, f: &pprof::protos::Function) -> &'a str {
    profile.string_table.get(f.name as usize).map(|s| s.as_str()).unwrap_or("")
}

/// Filter the profile so that only functions (and associated locations/samples)
/// whose names do not contain the unwanted substrings remain.
pub fn filter_profile(mut profile: Profile) -> Profile {
    // 1. Build a set of allowed function IDs by looking up the actual function name.
    let allowed_function_ids: HashSet<u64> = profile
        .function
        .iter()
        .filter(|f| is_allowed(get_function_name(&profile, f)))
        .map(|f| f.id)
        .collect();

    // 2. Retain only those Function records that are allowed.
    profile.function.retain(|f| allowed_function_ids.contains(&f.id));

    // 3. Filter locations:
    // A location is kept if any of its line entries reference an allowed function.
    profile
        .location
        .retain(|loc| loc.line.iter().any(|line| allowed_function_ids.contains(&line.function_id)));

    // 4. Build a set of allowed location IDs from the remaining locations.
    let allowed_location_ids: HashSet<u64> = profile.location.iter().map(|loc| loc.id).collect();

    // 5. For each sample in the profile, filter its list of location IDs so that
    // only allowed location IDs remain.
    for sample in &mut profile.sample {
        sample.location_id.retain(|loc_id| allowed_location_ids.contains(loc_id));
    }

    // 6. Remove any samples that no longer reference any locations.
    profile.sample.retain(|sample| !sample.location_id.is_empty());

    // 7. Remap function_ids and location_ids to be sequential
    remap_ids(&mut profile);

    profile
}

/// Remap function_ids and location_ids to be sequential after filtering
fn remap_ids(profile: &mut Profile) {
    // Create mappings from old IDs to new sequential IDs
    let mut function_id_map = std::collections::HashMap::new();
    let mut location_id_map = std::collections::HashMap::new();

    // Assign new sequential IDs to functions
    for (new_id, function) in profile.function.iter_mut().enumerate() {
        let old_id = function.id;
        let new_id = new_id as u64 + 1; // Start from 1 to match protobuf convention
        function_id_map.insert(old_id, new_id);
        function.id = new_id;
    }

    // Assign new sequential IDs to locations and update function_ids in lines
    for (new_id, location) in profile.location.iter_mut().enumerate() {
        let old_id = location.id;
        let new_id = new_id as u64 + 1; // Start from 1
        location_id_map.insert(old_id, new_id);
        location.id = new_id;

        // Update function_ids in line entries
        for line in &mut location.line {
            if let Some(&new_function_id) = function_id_map.get(&line.function_id) {
                line.function_id = new_function_id;
            }
        }
    }

    // Update location_ids in samples
    for sample in &mut profile.sample {
        sample.location_id = sample
            .location_id
            .iter()
            .filter_map(|&old_id| location_id_map.get(&old_id).copied())
            .collect();
    }
}
