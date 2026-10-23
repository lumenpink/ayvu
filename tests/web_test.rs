use ayvu::server::run_server_on_random_port;
use std::fs;
use std::path::Path;

#[test]
fn test_wrangler_config_and_site_files() {
    let wrangler = Path::new("wrangler.jsonc");
    assert!(wrangler.is_file(), "wrangler.jsonc should exist");

    let wrangler_content = fs::read_to_string(wrangler).expect("wrangler.jsonc readable");
    assert!(wrangler_content.contains("ayvu.2lp.in"));
    assert!(wrangler_content.contains("./site"));

    let index_html = Path::new("site/index.html");
    assert!(index_html.is_file(), "site/index.html should exist");

    let html_content = fs::read_to_string(index_html).expect("site/index.html readable");
    assert!(html_content.contains("Ayvu"));
    assert!(html_content.contains("Playground"));
    assert!(html_content.contains("id=\"editor\""));
    assert!(html_content.contains("id=\"preview\""));
    assert!(html_content.contains("POW!"));
    assert!(html_content.contains("footer-canonical"));
    assert!(html_content.contains("https://travesti.2lp.in/pt/"));
    assert!(html_content.contains("https://lumen.pink"));

    let headers = Path::new("site/_headers");
    assert!(headers.is_file(), "site/_headers should exist");
}

#[test]
fn test_server_serves_web_playground_on_root() {
    let handle = run_server_on_random_port().expect("server should start");
    let port = handle.port();

    let url = format!("http://127.0.0.1:{}/", port);
    let resp = ureq::get(&url)
        .call()
        .expect("GET / request should succeed");

    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.header("content-type"),
        Some("text/html; charset=utf-8")
    );

    let body = resp.into_string().expect("response body");
    assert!(body.contains("Ayvu"));
    assert!(body.contains("Playground"));
    assert!(body.contains("id=\"editor\""));

    handle.stop();
}

#[test]
fn test_character_inspector_page_and_server_endpoints() {
    let character_html = Path::new("site/character.html");
    assert!(character_html.is_file(), "site/character.html should exist");

    let html_content = fs::read_to_string(character_html).expect("site/character.html readable");
    assert!(html_content.contains("Inspetor de Personagens"));
    assert!(html_content.contains("fixed-ruler"));
    assert!(html_content.contains("stage-canvas"));
    assert!(html_content.contains("poses-grid"));
    assert!(html_content.contains("groundline") || html_content.contains("GROUNDLINE"));

    // Test server serving /character.html and /character API
    let handle = run_server_on_random_port().expect("server should start");
    let port = handle.port();

    let url_html = format!("http://127.0.0.1:{}/character.html", port);
    let resp_html = ureq::get(&url_html)
        .call()
        .expect("GET /character.html request should succeed");
    assert_eq!(resp_html.status(), 200);
    assert_eq!(
        resp_html.header("content-type"),
        Some("text/html; charset=utf-8")
    );

    let url_api = format!("http://127.0.0.1:{}/character?id=arti", port);
    let resp_api = ureq::get(&url_api)
        .call()
        .expect("GET /character?id=arti request should succeed");
    assert_eq!(resp_api.status(), 200);
    let body_api = resp_api.into_string().expect("json response body");
    assert!(body_api.contains("\"id\":\"arti\""));
    assert!(body_api.contains("\"width\":157.0"));
    assert!(body_api.contains("\"height\":396.0"));
    assert!(body_api.contains("\"poses\""));

    handle.stop();
}
