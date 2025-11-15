use crate::fs::fs::get_dir_entries;
use axum::body::Body;
use axum::extract::Query;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum_extra::headers::Range;
use axum_extra::TypedHeader;
use axum_range::{KnownSize, Ranged};
use std::collections::HashMap;
use tokio::fs::{self, File};
use tokio_util::io::ReaderStream;

pub async fn get_entries() -> impl IntoResponse {
    let files = get_dir_entries();

    let json = serde_json::to_string(&files).unwrap();

    return json;
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn download2(
    range: Option<TypedHeader<Range>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let file = match File::open("C:\\Users\\Rikelmy\\Coding\\testee.zip").await {
        Ok(f) => f,
        Err(err) => {
            return (StatusCode::NOT_FOUND, format!("File not found: {}", err)).into_response()
        }
    };
    let body = KnownSize::file(file).await.unwrap();

    for v in params.values() {
        println!("{}", v)
    }

    let range = range.map(|TypedHeader(range)| range);
    let ranged = Ranged::new(range, body);

    let content_disposition = format!("attachment; filename={}", "\"teste.zip\"");

    (
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            ),
            (
                header::CONTENT_DISPOSITION,
                HeaderValue::from_str(&content_disposition).unwrap(),
            ),
        ],
        ranged,
    )
        .into_response()
}

pub async fn download() -> impl IntoResponse {
    let path = "C:\\Users\\SI30\\Coding\\SIGP_INT.jar";

    let metadata = match fs::metadata(path).await {
        Ok(m) => m,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("Metadata error: {}", err))),
    };

    let file_size = metadata.len().to_string();

    let file = match File::open(path).await {
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
