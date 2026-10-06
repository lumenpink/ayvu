use crate::character::CharacterRegistry;
use crate::model::StripLayout;
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
    pub fn port(&self) -> u16 {
        self.port
    }

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
    #[serde(default)]
    layout: Option<String>,
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

/// Helper to start HTTP server on an OS-assigned random port for tests or embedding
pub fn run_server_on_random_port() -> Result<ServerHandle, Box<dyn std::error::Error>> {
    let (_, handle) =
        start_server_in_background("127.0.0.1", 0).map_err(Box::<dyn std::error::Error>::from)?;
    thread::sleep(Duration::from_millis(50));
    Ok(handle)
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
    println!("Endpoints available: POST /render, GET /render?text=..., GET /characters, GET /health, GET /version");

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
        (&Method::Get, "/") | (&Method::Get, "/index.html") => {
            let html = include_str!("../site/index.html");
            let header =
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
            let res = Response::from_string(html)
                .with_header(header)
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        (&Method::Get, "/character.html") | (&Method::Get, "/characters.html") => {
            let html = include_str!("../site/character.html");
            let header =
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
            let res = Response::from_string(html)
                .with_header(header)
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        (&Method::Get, p) if p.starts_with("/pkg/") => {
            let candidate1 = format!("./site{}", p);
            let candidate2 = format!(".{}", p);

            let file_path = if std::path::Path::new(&candidate1).is_file() {
                candidate1
            } else {
                candidate2
            };

            if let Ok(bytes) = std::fs::read(&file_path) {
                let mime = if p.ends_with(".wasm") {
                    "application/wasm"
                } else if p.ends_with(".js") {
                    "application/javascript; charset=utf-8"
                } else {
                    "application/octet-stream"
                };

                let header = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();
                let res = Response::from_data(bytes)
                    .with_header(header)
                    .with_header(cors_header);
                let _ = request.respond(res);
                return;
            }

            let res = Response::from_string("Not Found")
                .with_status_code(StatusCode(404))
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        (&Method::Get, "/health") => {
            let json = serde_json::to_string(&HealthResponse {
                status: "ok",
                service: "ayvu",
            })
            .unwrap();

            let header = Header::from_bytes(
                &b"Content-Type"[..],
                &b"application/json; charset=utf-8"[..],
            )
            .unwrap();
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

            let header = Header::from_bytes(
                &b"Content-Type"[..],
                &b"application/json; charset=utf-8"[..],
            )
            .unwrap();
            let res = Response::from_string(json)
                .with_header(header)
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        (&Method::Get, "/characters") => {
            let registry = CharacterRegistry::default();
            let metadata = registry.list_metadata();
            let json = serde_json::to_string(&metadata).unwrap();

            let header = Header::from_bytes(
                &b"Content-Type"[..],
                &b"application/json; charset=utf-8"[..],
            )
            .unwrap();
            let res = Response::from_string(json)
                .with_header(header)
                .with_header(cors_header);
            let _ = request.respond(res);
        }
        (&Method::Get, "/character") | (&Method::Get, "/character.json") => {
            let registry = CharacterRegistry::default();
            let id = query
                .and_then(|q| {
                    q.split('&')
                        .find_map(|pair| pair.strip_prefix("id="))
                })
                .unwrap_or("arti");

            if let Some(detail) = registry.get_detail(id) {
                let json = serde_json::to_string(&detail).unwrap();
                let header = Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"application/json; charset=utf-8"[..],
                )
                .unwrap();
                let res = Response::from_string(json)
                    .with_header(header)
                    .with_header(cors_header);
                let _ = request.respond(res);
            } else {
                let res = Response::from_string("{\"error\":\"character not found\"}")
                    .with_status_code(StatusCode(404))
                    .with_header(cors_header);
                let _ = request.respond(res);
            }
        }
        (&Method::Post, "/render") | (&Method::Post, "/comic") => {
            let mut body = String::new();
            if let Err(e) = request.as_reader().read_to_string(&mut body) {
                let res = Response::from_string(format!("Failed to read request body: {}", e))
                    .with_status_code(StatusCode(400));
                let _ = request.respond(res);
                return;
            }

            let (text, title, layout) = if body.trim_start().starts_with('{') {
                match serde_json::from_str::<RenderRequest>(&body) {
                    Ok(req) => (req.text, req.title, req.layout),
                    Err(e) => {
                        let res = Response::from_string(format!("Invalid JSON payload: {}", e))
                            .with_status_code(StatusCode(400));
                        let _ = request.respond(res);
                        return;
                    }
                }
            } else {
                (body, None, None)
            };

            respond_with_svg(
                request,
                &text,
                title.as_deref(),
                layout.as_deref(),
                cors_header,
            );
        }
        (&Method::Get, "/render") | (&Method::Get, "/comic") => {
            let mut text = String::new();
            let mut title = None;
            let mut layout = None;

            if let Some(q) = query {
                for pair in q.split('&') {
                    if let Some((k, v)) = pair.split_once('=') {
                        let decoded = decode_percent(v);
                        if k == "text" {
                            text = decoded;
                        } else if k == "title" {
                            title = Some(decoded);
                        } else if k == "layout" {
                            layout = Some(decoded);
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

            respond_with_svg(
                request,
                &text,
                title.as_deref(),
                layout.as_deref(),
                cors_header,
            );
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
    layout_override: Option<&str>,
    cors_header: Header,
) {
    match parse_dialogue_script(text, title) {
        Ok(mut strip) => {
            if let Some(l) = layout_override {
                strip.layout = StripLayout::from_str_loose(l);
            }
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
            let res = Response::from_string(format!("Script parsing error: {}", e))
                .with_status_code(StatusCode(400))
                .with_header(cors_header);
            let _ = request.respond(res);
        }
    }
}

fn decode_percent(s: &str) -> String {
    let replaced = s.replace('+', " ");
    let mut bytes = Vec::new();
    let mut chars = replaced.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex_high = chars.next();
            let hex_low = chars.next();
            if let (Some(h1), Some(h2)) = (hex_high, hex_low) {
                let hex_str = format!("{}{}", h1, h2);
                if let Ok(b) = u8::from_str_radix(&hex_str, 16) {
                    bytes.push(b);
                    continue;
                }
            }
            bytes.push(b'%');
        } else {
            let mut buf = [0; 4];
            let encoded = c.encode_utf8(&mut buf);
            bytes.extend_from_slice(encoded.as_bytes());
        }
    }

    String::from_utf8_lossy(&bytes).to_string()
}
