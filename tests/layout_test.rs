use ayvu::model::StripLayout;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::render_comic_strip_to_svg;
use ayvu::server::run_server_on_random_port;

#[test]
fn test_parse_layout_directive_grid() {
    let script = r#"
[layout: grid]
Alice: First panel!
---
Bob: Second panel!
---
Alice: Third panel!
---
Bob: Fourth panel!
"#;
    let strip = parse_dialogue_script(script, Some("Grid Comic"))
        .expect("parsing script with layout should succeed");

    assert_eq!(strip.layout, StripLayout::Grid);
    assert_eq!(strip.panels.len(), 4);

    let svg = render_comic_strip_to_svg(&strip);
    // Grid with 2 columns should have width for 2 panels
    // 2 * 280 + 3 * 16 = 608.0
    assert!(svg.contains(r#"viewBox="0 0 608.0"#));
}

#[test]
fn test_parse_layout_directive_vertical() {
    let script = r#"
[layout: vertical]
Alice: Top panel!
---
Bob: Bottom panel!
"#;
    let strip = parse_dialogue_script(script, Some("Vertical Strip"))
        .expect("parsing script should succeed");

    assert_eq!(strip.layout, StripLayout::Vertical);
    assert_eq!(strip.panels.len(), 2);

    let svg = render_comic_strip_to_svg(&strip);
    // Vertical with 1 column should have width for 1 panel: 280 + 2 * 16 = 312.0
    assert!(svg.contains(r#"viewBox="0 0 312.0"#));
}

#[test]
fn test_parse_onomatopoeia_impact_badge() {
    let script = r#"
[POW!]
Alice (angry): Gotcha!
---
Bob (surprised): What hit me?!
[CRASH!]
"#;
    let strip = parse_dialogue_script(script, None).expect("parsing onomatopoeia should succeed");

    assert_eq!(strip.panels.len(), 2);
    assert!(!strip.panels[0].effects.is_empty());
    assert!(!strip.panels[1].effects.is_empty());

    let svg = render_comic_strip_to_svg(&strip);
    assert!(svg.contains("onomatopoeia-starburst"));
    assert!(svg.contains("POW!"));
    assert!(svg.contains("CRASH!"));
}

#[test]
fn test_parse_emotion_stickers() {
    let script = r#"
Alice (happy): I love this! [heart]
Bob (thinking): Let me ponder... [idea]
---
Alice (shock): Oh no! [shock]
Bob (sad): I'm nervous... [sweat]
"#;
    let strip = parse_dialogue_script(script, None).expect("parsing stickers should succeed");

    let svg = render_comic_strip_to_svg(&strip);
    assert!(svg.contains("sticker-heart"));
    assert!(svg.contains("sticker-idea"));
    assert!(svg.contains("sticker-shock"));
    assert!(svg.contains("sticker-sweat"));
}

#[test]
fn test_http_render_with_layout_param() {
    let handle = run_server_on_random_port().expect("server should start");
    let port = handle.port();

    // Test GET /render with layout=vertical
    let url = format!(
        "http://127.0.0.1:{}/render?text=Alice:+Hello!%0A---%0ABob:+Hi!&layout=vertical",
        port
    );
    let resp = ureq::get(&url).call().expect("GET /render should succeed");
    assert_eq!(resp.status(), 200);

    let body = resp.into_string().expect("response body");
    // Vertical layout width is 312.0
    assert!(body.contains(r#"viewBox="0 0 312.0"#));

    // Test POST /render with layout JSON field
    let post_url = format!("http://127.0.0.1:{}/render", port);
    let post_resp = ureq::post(&post_url)
        .set("Content-Type", "application/json")
        .send_string(
            r#"{
            "text": "Alice: P1\n---\nBob: P2\n---\nAlice: P3\n---\nBob: P4",
            "layout": "grid"
        }"#,
        )
        .expect("POST /render should succeed");

    let post_body = post_resp.into_string().expect("post body");
    // Grid layout width is 608.0
    assert!(post_body.contains(r#"viewBox="0 0 608.0"#));

    handle.stop();
}
