use router::create_router;
use app_state::AppState;

pub mod app_state;
pub mod router;
pub mod routes;
pub mod queries;
pub mod utilities;


pub async fn run_server(state: AppState) {
    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server Running");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}