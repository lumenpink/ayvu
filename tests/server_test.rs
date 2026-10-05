use ayvu::server::start_server_in_background;
use std::thread;
use std::time::Duration;

#[test]
fn test_http_server_endpoints() {
    let (port, server_handle) =
        start_server_in_background("127.0.0.1", 0).expect("failed to start background server");

    // Wait a brief moment for socket to be ready
    thread::sleep(Duration::from_millis(50));
    let base_url = format!("http://127.0.0.1:{}", port);

    // 1. Test GET /health
    let health_res = ureq::get(&format!("{}/health", base_url))
        .call()
        .expect("GET /health failed");
    assert_eq!(health_res.status(), 200);
    let health_body = health_res.into_string().expect("failed to read body");
    let health_json: serde_json::Value = serde_json::from_str(&health_body).expect("invalid json");
    assert_eq!(health_json["status"], "ok");
    assert_eq!(health_json["service"], "ayvu");

    // 2. Test GET /version
    let version_res = ureq::get(&format!("{}/version", base_url))
        .call()
        .expect("GET /version failed");
    assert_eq!(version_res.status(), 200);
    let version_body = version_res.into_string().expect("failed to read body");
    let version_json: serde_json::Value =
        serde_json::from_str(&version_body).expect("invalid json");
    assert!(version_json["version"].is_string());

    // 3. Test POST /render with JSON
    let json_payload = serde_json::json!({
        "text": "Alice (happy): Test from HTTP JSON!\nBob: Works!",
        "title": "HTTP Strip"
    })
    .to_string();

    let post_json_res = ureq::post(&format!("{}/render", base_url))
        .set("Content-Type", "application/json")
        .send_string(&json_payload)
        .expect("POST /render json failed");

    assert_eq!(post_json_res.status(), 200);
    assert_eq!(
        post_json_res.header("Content-Type").unwrap(),
        "image/svg+xml; charset=utf-8"
    );
    assert_eq!(
        post_json_res.header("Access-Control-Allow-Origin").unwrap(),
        "*"
    );
    let svg_body = post_json_res.into_string().unwrap();
    assert!(svg_body.contains("<svg"));
    assert!(svg_body.contains("HTTP Strip"));
    assert!(svg_body.contains("Test from HTTP") && svg_body.contains("JSON!"));

    // 4. Test POST /render with raw text
    let post_raw_res = ureq::post(&format!("{}/render", base_url))
        .set("Content-Type", "text/plain")
        .send_string("Alice: Raw plain text dialogue!")
        .expect("POST /render raw text failed");

    assert_eq!(post_raw_res.status(), 200);
    let raw_svg = post_raw_res.into_string().unwrap();
    assert!(raw_svg.contains("<svg"));
    assert!(raw_svg.contains("Raw plain text") && raw_svg.contains("dialogue!"));

    // 5. Test GET /render?text=...
    let get_render_res = ureq::get(&format!(
        "{}/render?text=Alice%3A%20Hello%20GET&title=FromGET",
        base_url
    ))
    .call()
    .expect("GET /render query failed");

    assert_eq!(get_render_res.status(), 200);
    let get_svg = get_render_res.into_string().unwrap();
    assert!(get_svg.contains("<svg"));
    assert!(get_svg.contains("FromGET"));
    assert!(get_svg.contains("Hello GET"));

    // Stop background server
    server_handle.stop();
}
