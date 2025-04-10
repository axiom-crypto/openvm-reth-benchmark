use std::sync::{Arc, Mutex};

use axum::{extract::Query, response::IntoResponse};
use pprof::{protos::Message, ProfilerGuard};
use serde::Deserialize;

#[derive(Deserialize)]
struct ProfileParams {
    seconds: Option<u32>,
}

pub async fn start_profile_server(guard: Arc<Mutex<ProfilerGuard<'static>>>) -> eyre::Result<()> {
    let app = axum::Router::new().route(
        "/debug/pprof/profile",
        axum::routing::get(|query: Query<ProfileParams>| async move {
            let _ = query.seconds.unwrap_or(10);

            // Lock the mutex to ensure exclusive access to the profiler guard
            let guard = guard.lock().unwrap();

            let report = guard.report().build().expect("Failed to build report");
            let pprof_data = report.pprof().expect("Failed to create pprof");
            let bytes = axum::body::Bytes::from(
                pprof_data.write_to_bytes().expect("Failed to write pprof data"),
            );
            bytes.into_response()
        }),
    );

    println!("Starting profile server on port 3000");

    // Spawn a task to run the server in the background
    tokio::spawn(async move {
        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    Ok(())
}
