use crate::parser::parse_dialogue_script;
use crate::renderer::render_comic_strip_to_svg;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tiny_http::{Header, Method, Response, Server, StatusCode};

#[derive(Debug, Clone)]
pub struct ServerHandle {
    shutdown: Arc<AtomicBool>,
    port: u16,
}

impl ServerHandle {
    pub fn stop(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        // Ping server to unblock recv() loop
        let _ = std::net::TcpStream::connect(format!("127.0.0.1:{}", self.port));
    }
}

#[derive(Deserialize)]
struct RenderRequest {
    text: String,
    #[serde(default)]
    title: Option<String>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Serialize)]
struct VersionResponse {
    version: &'static str,
}

/// Start HTTP server in a background thread for tests and embedded usage
pub fn start_server_in_background(host: &str, port: u16) -> Result<(u16, ServerHandle), String> {
    let server = Server::http(format!("{}:{}", host, port))
        .map_err(|e| format!("Failed to bind HTTP server to {}:{}: {}", host, port, e))?;

    let bound_port = server
        .server_addr()
        .to_ip()
        .map(|a| a.port())
        .unwrap_or(port);

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown);

    let handle = ServerHandle {
        shutdown,
        port: bound_port,
    };

    thread::spawn(move || {
        let server = server;
        while !shutdown_clone.load(Ordering::SeqCst) {
            match server.recv_timeout(Duration::from_millis(200)) {
                Ok(Some(request)) => {
                    handle_request(request);
                }
                Ok(None) => {}
                Err(_) => break,
            }
        }
    });

    Ok((bound_port, handle))
}

/// Run HTTP server synchronously (CLI daemon mode)
pub fn run_server(host: &str, port: u16) -> Result<(), String> {
    let server = Server::http(format!("{}:{}", host, port))
        .map_err(|e| format!("Failed to bind HTTP server to {}:{}: {}", host, port, e))?;

    println!(
        "Ayvu HTTP microservice listening on http://{}:{}",
        host, port
    );
    println!("Endpoints available: POST /render, GET /render?text=..., GET /health, GET /version");

    for request in server.incoming_requests() {
        handle_request(request);
    }

    Ok(())
}

fn handle_request(mut request: tiny_http::Request) {
    let cors_header = Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap();
    let cors_methods = Header::from_bytes(
        &b"Access-Control-Allow-Methods"[..],
        &b"GET, POST, OPTIONS"[..],
    )
    .unwrap();
    let cors_headers = Header::from_bytes(
        &b"Access-Control-Allow-Headers"[..],
        &b"Content-Type, Authorization"[..],
    )
    .unwrap();

    // Handle OPTIONS for CORS pre-flight
    if request.method() == &Method::Options {
        let res = Response::empty(StatusCode(204))
            .with_header(cors_header)
            .with_header(cors_methods)
            .with_header(cors_headers);
        let _ = request.respond(res);
        return;
    }

    let url = request.url().to_string();
    let (path, query) = match url.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (url.as_str(), None),
    };

    match (request.method(), path) {
        (&Method::Get, "/health") => {
            let json = serde_json::to_string(&HealthResponse {
                status: "ok",
                service: "ayvu",
            })
            .unwrap();

            let header =
                Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
            let res = Response::from_string(json)
                .with_header(header)
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        (&Method::Get, "/version") => {
            let json = serde_json::to_string(&VersionResponse {
                version: crate::VERSION,
            })
            .unwrap();

            let header =
                Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
            let res = Response::from_string(json)
                .with_header(header)
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        (&Method::Post, "/render") | (&Method::Post, "/comic") => {
            let mut body = String::new();
            if let Err(e) = request.as_reader().read_to_string(&mut body) {
                let res = Response::from_string(format!("Failed to read request body: {}", e))
                    .with_status_code(StatusCode(400));
                let _ = request.respond(res);
                return;
            }

            let (text, title) = if body.trim_start().starts_with('{') {
                match serde_json::from_str::<RenderRequest>(&body) {
                    Ok(req) => (req.text, req.title),
                    Err(e) => {
                        let res = Response::from_string(format!("Invalid JSON payload: {}", e))
                            .with_status_code(StatusCode(400));
                        let _ = request.respond(res);
                        return;
                    }
                }
            } else {
                (body, None)
            };

            respond_with_svg(request, &text, title.as_deref(), cors_header);
        }
        (&Method::Get, "/render") | (&Method::Get, "/comic") => {
            let mut text = String::new();
            let mut title = None;

            if let Some(q) = query {
                for pair in q.split('&') {
                    if let Some((k, v)) = pair.split_once('=') {
                        let decoded = decode_percent(v);
                        if k == "text" {
                            text = decoded;
                        } else if k == "title" {
                            title = Some(decoded);
                        }
                    }
                }
            }

            if text.trim().is_empty() {
                let res = Response::from_string("Missing required 'text' query parameter")
                    .with_status_code(StatusCode(400))
                    .with_header(cors_header);
                let _ = request.respond(res);
                return;
            }

            respond_with_svg(request, &text, title.as_deref(), cors_header);
        }
        _ => {
            let res = Response::from_string("Not Found")
                .with_status_code(StatusCode(404))
                .with_header(cors_header);
            let _ = request.respond(res);
        }
    }
}

fn respond_with_svg(
    request: tiny_http::Request,
    text: &str,
    title: Option<&str>,
    cors_header: Header,
) {
    match parse_dialogue_script(text, title) {
        Ok(strip) => {
            let svg = render_comic_strip_to_svg(&strip);
            let content_type =
                Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml; charset=utf-8"[..])
                    .unwrap();

            let res = Response::from_string(svg)
                .with_header(content_type)
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        Err(e) => {
            let res = Response::from_string(format!("Error parsing dialogue script: {}", e))
                .with_status_code(StatusCode(422))
                .with_header(cors_header);
            let _ = request.respond(res);
        }
    }
}

/// Simple percent-decoding helper for query parameters
fn decode_percent(s: &str) -> String {
    let replaced = s.replace('+', " ");
    let mut bytes = Vec::new();
    let mut chars = replaced.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(c1), Some(c2)) = (h1, h2) {
                let hex_str = format!("{}{}", c1, c2);
                if let Ok(b) = u8::from_str_radix(&hex_str, 16) {
                    bytes.push(b);
                    continue;
                }
            }
        }
        let mut buf = [0; 4];
        let encoded = c.encode_utf8(&mut buf);
        bytes.extend_from_slice(encoded.as_bytes());
    }

    String::from_utf8_lossy(&bytes).to_string()
}
