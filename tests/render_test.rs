use ayvu::balloon::{render_balloon_svg, Point};
use ayvu::model::BalloonType;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::render_comic_strip_to_svg;

#[test]
fn test_render_speech_balloon_svg() {
    let text = "Hello from Comic Chat!";
    let bubble_origin = Point { x: 20.0, y: 20.0 };
    let target = Point { x: 80.0, y: 150.0 };

    let svg = render_balloon_svg(text, BalloonType::Speech, bubble_origin, target, 200.0);
    assert!(svg.contains("<path") || svg.contains("<rect"));
    assert!(svg.contains("Hello from Comic Chat!"));
    assert!(svg.contains("stroke=\"#000000\""));
}

#[test]
fn test_render_thought_balloon_svg() {
    let text = "I wonder...";
    let bubble_origin = Point { x: 20.0, y: 20.0 };
    let target = Point { x: 80.0, y: 150.0 };

    let svg = render_balloon_svg(text, BalloonType::Thought, bubble_origin, target, 200.0);
    assert!(svg.contains("<circle"));
    assert!(svg.contains("I wonder..."));
}

#[test]
fn test_render_shout_balloon_svg() {
    let text = "WATCH OUT!";
    let bubble_origin = Point { x: 20.0, y: 20.0 };
    let target = Point { x: 80.0, y: 150.0 };

    let svg = render_balloon_svg(text, BalloonType::Shout, bubble_origin, target, 200.0);
    assert!(svg.contains("polygon") || svg.contains("path"));
    assert!(svg.contains("WATCH OUT!"));
}

#[test]
fn test_render_complete_comic_strip_svg() {
    let script = r#"
        Alice (happy): Good morning!
        Bob (thinking): Is it morning already?
        ---
        Bob (laughing): Coffee is ready!
    "#;

    let strip =
        parse_dialogue_script(script, Some("Daily Routine")).expect("failed to parse script");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>"));
    assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
    assert!(svg.contains("viewBox="));
    assert!(svg.contains("Daily Routine"));
    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));
    assert!(svg.contains("Good morning!"));
    assert!(svg.contains("Coffee is ready!"));
}
