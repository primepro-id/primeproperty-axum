mod agents;
// mod banks;
mod db;
// mod developers;
mod envs;
// mod leads;
mod mail;
mod middleware;
// mod properties;
mod s3;
mod schema;
mod supertokens;

use crate::db::build_db_pool;
use crate::envs::Envs;
use axum::http::{HeaderValue, Method};
use axum::Router;
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let sentry_url = Envs::sentry_url();
    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    let app_env = Envs::app_env();
    let origins = [
        "https://primeproindonesia.com"
            .parse::<HeaderValue>()
            .unwrap(),
        "https://agent.primeproindonesia.com"
            .parse::<HeaderValue>()
            .unwrap(),
    ];
    let cors = match app_env.as_str() {
        "production" => CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
            ]),
        _ => CorsLayer::permissive(),
    };

    let tracing_filter = tracing_subscriber::EnvFilter::new(
        "tower_http::trace::make_span=debug,tower_http::trace::on_response=debug,tower_http::trace::on_request=debug",
    );
    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .with_max_level(tracing::Level::ERROR)
        .init();

    // build our application with a route
    let pool = build_db_pool();
    let app = Router::new()
        .nest("/agents", agents::routes())
        // .nest("/banks", banks::banks_routes(pool.clone()))
        // .nest("/developers", developers::developers_routes(pool.clone()))
        // .nest("/leads", leads::lead_routes())
        // .nest("/properties", properties::property_routes())
        .nest("/s3", s3::routes())
        .with_state(pool)
        // .layer(from_fn(middleware::Session::middleware))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::new().enable_transaction());

    // run our app with hyper, listening globally on env port
    let host_addr = Envs::host_address();
    let listener = match tokio::net::TcpListener::bind(&host_addr).await {
        Ok(listen) => listen,
        Err(err) => panic!("Failed to bind address: {}", err),
    };

    println!("Server started at {}", &host_addr);
    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(err) => panic!("Failed to start server: {}", err),
    }
}
