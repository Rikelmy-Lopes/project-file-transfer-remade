use crate::http::routes::{download, download2, get_entries, root};
use crate::utils::os::get_current_ip;

use axum::http::StatusCode;
use axum::routing::{get, get_service};
use axum::Router;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tokio::sync::oneshot;
use tower_http::services::ServeDir;

static STOP_TX: Lazy<Mutex<Option<oneshot::Sender<()>>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub async fn start_server(port: u16) -> () {
    let ip = get_current_ip();
    let (tx, rx) = oneshot::channel::<()>();

    *STOP_TX.lock().unwrap() = Some(tx);

    let app = Router::new()
        .route("/download", get(download))
        .route("/entries", get(get_entries))
        .route("/file", get(download2))
        .fallback_service(
            get_service(ServeDir::new("../web-ui/webapp"))
                .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR }),
        );

    let url = format!("{}:{}", ip, port);

    println!("Server running on: http://{}", url);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();

    let server = axum::serve(listener, app).with_graceful_shutdown(async {
        rx.await.ok();
    });

    server.await.unwrap()
}

#[tauri::command]
pub fn stop_server() -> () {
    if let Some(tx) = STOP_TX.lock().unwrap().take() {
        println!("Stopping server!");
        let _ = tx.send(());
    }
}
