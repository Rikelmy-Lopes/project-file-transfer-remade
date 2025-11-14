use crate::fs::fs::get_dir_entries;
use axum::body::Body;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::{AppendHeaders, IntoResponse};
/* use crate::state::state::APP_STATE; */
use axum::routing::get;
use axum::Router;
use local_ip_address::local_ip;
use once_cell::sync::Lazy;
use std::env::home_dir;
use std::sync::Mutex;
use tokio::fs;
use tokio::sync::oneshot;
use tokio_util::io::ReaderStream;

static STOP_TX: Lazy<Mutex<Option<oneshot::Sender<()>>>> = Lazy::new(|| Mutex::new(None));

async fn get_entries() -> impl IntoResponse {
    let files = get_dir_entries();

    let json = serde_json::to_string(&files).unwrap();

    return json;
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn download() -> impl IntoResponse {
    let path = "C:\\Users\\SI30\\Coding\\SIGP_INT.jar";

    let metadata = match fs::metadata(path).await {
        Ok(m) => m,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("Metadata error: {}", err))),
    };

    let file_size = metadata.len().to_string();

    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);

    let body = Body::from_stream(stream);

    let mut response = Response::new(body);
    let headers = response.headers_mut();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("attachment; filename=\"SIGP_INT.jar\""),
    );

    headers.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&file_size.to_string()).unwrap(),
    );

    Ok(response)
}

#[tauri::command]
pub async fn start_server(port: u16) -> () {
    let ip = get_current_ip();
    let (tx, rx) = oneshot::channel::<()>();

    *STOP_TX.lock().unwrap() = Some(tx);

    let app = Router::new()
        .route("/", get(root))
        .route("/download", get(download))
        .route("/entries", get(get_entries));

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
        let _ = tx.send(());
    }
}

#[tauri::command]
pub fn get_current_ip() -> String {
    let ip = local_ip().unwrap().to_string();
    ip
}
