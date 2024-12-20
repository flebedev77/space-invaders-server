use std::sync::atomic::AtomicUsize;

use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, Extension, SocketRef, State},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Serialize, Deserialize)]
struct MoveData {
    left: bool,
    right: bool,
    shoot: bool 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting server");

    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/", |s: SocketRef| {
        info!("Socket connected with id {}", s.id);
        s.on(
            "move",
            |s: SocketRef, Data::<MoveData>(msg)| {
                let msg = &MoveData {
                    left: msg.left,
                    right: msg.right,
                    shoot: msg.shoot
                };
                s.broadcast().emit("move", msg).ok();
            },
        );

        s.on_disconnect(
            |s: SocketRef | {
                //s.broadcast().emit("user left", res).ok();
                info!("Socket with id {} left", s.id)
            },
        );
    });

    let app = axum::Router::new()
        .nest_service("/", ServeDir::new("client"))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // Enable CORS policy
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}