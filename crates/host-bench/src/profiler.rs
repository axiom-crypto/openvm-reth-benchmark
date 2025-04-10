use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct ProfileParams {
    seconds: Option<u32>,
}

pub async fn start_profile_server(profile_data: Vec<u8>) -> eyre::Result<()> {
    let app = axum::Router::new().route(
        "/debug/pprof/profile",
        axum::routing::get(|query: Query<ProfileParams>| async move {
            let _ = query.seconds.unwrap_or(30);
            let bytes = axum::body::Bytes::from(profile_data.clone());
            bytes.into_response()
        }),
    );

    println!("Starting profile server on port 3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
