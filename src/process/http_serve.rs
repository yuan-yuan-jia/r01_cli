use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    info!("Serving {:?} on {}", path, addr);
    let state = HttpServeState { path: path.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><url>...</url></body></html>
        if p.is_dir() {
            let mut content = String::from("<html><body>");
            for entry in fs::read_dir(p).expect("read dir failed") {
                match entry {
                    Ok(e) => content.push_str(&format!(
                        "<li><a href=\"{:?}\">{:?}</a></li>",
                        e.path().as_path(),
                        e.file_name()
                    )),
                    Err(e) => {
                        warn!("read file failed {}", e)
                    }
                }
            }
            content.push_str("</body></html>");
            (StatusCode::OK, content)
        } else if p.is_file() {
            match tokio::fs::read_to_string(p).await {
                Ok(content) => {
                    info!("Read {} bytes", content.len());
                    (StatusCode::OK, content)
                }
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            }
        } else {
            warn!("No file found");
            (StatusCode::NOT_FOUND, "No file found".to_owned())
        }
    }
}
