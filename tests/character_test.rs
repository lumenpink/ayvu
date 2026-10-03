use ayvu::character::{CharacterManifest, CharacterRegistry};
use ayvu::model::Emotion;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::render_comic_strip_to_svg_with_registry;
use ayvu::server::run_server_on_random_port;

#[test]
fn test_character_manifest_deserialization() {
    let raw = r#"{
        "id": "tuffy",
        "name": "Tuffy",
        "author": "Ayvu Project",
        "license": "GPL-3.0-or-later",
        "description": "Retro cartoon dog",
        "balloon_anchor": { "x": 0.5, "y": 0.3 },
        "poses": {
            "neutral": "<circle cx=\"50\" cy=\"50\" r=\"40\" />",
            "happy": "<path d=\"M20 50 Q50 90 80 50\" />"
        }
    }"#;

    let manifest: CharacterManifest = serde_json::from_str(raw).expect("manifest should parse");
    assert_eq!(manifest.id, "tuffy");
    assert_eq!(manifest.name, "Tuffy");
    assert_eq!(manifest.balloon_anchor.x, 0.5);
    assert_eq!(manifest.balloon_anchor.y, 0.3);
    assert!(manifest.poses.contains_key("neutral"));
    assert!(manifest.poses.contains_key("happy"));
}

#[test]
fn test_default_registry_contains_presets() {
    let registry = CharacterRegistry::default();
    assert!(registry.contains_id("tuffy"));
    assert!(registry.contains_id("jordan"));
    assert!(registry.contains_id("peep"));

    let list = registry.list_metadata();
    assert!(list.len() >= 3);
}

#[test]
fn test_registry_speaker_resolution() {
    let mut registry = CharacterRegistry::default();

    // Explicit character match
    let tuffy_pack = registry.resolve_speaker("Tuffy");
    assert_eq!(tuffy_pack.id, "tuffy");

    // Dynamic fallback for unknown speaker 1
    let speaker1_pack = registry.resolve_speaker("Alice");
    // Dynamic fallback for unknown speaker 2
    let speaker2_pack = registry.resolve_speaker("Bob");

    // Should assign different characters deterministically
    assert_ne!(speaker1_pack.id, speaker2_pack.id);

    // Same speaker repeatedly resolves to the same pack
    let speaker1_again = registry.resolve_speaker("Alice");
    assert_eq!(speaker1_pack.id, speaker1_again.id);
}

#[test]
fn test_pose_retrieval_and_fallback() {
    let registry = CharacterRegistry::default();
    let tuffy = registry.get("tuffy").expect("tuffy pack exists");

    let happy_pose = tuffy.get_pose_svg(&Emotion::Happy);
    assert!(!happy_pose.is_empty());

    let neutral_pose = tuffy.get_pose_svg(&Emotion::Neutral);
    assert!(!neutral_pose.is_empty());
}

#[test]
fn test_renderer_with_character_registry() {
    let script = r#"
Tuffy (happy): Hello world!
Bob (angry): Grrr!
"#;
    let strip = parse_dialogue_script(script, Some("Character Test Strip"))
        .expect("script parsing should succeed");

    let registry = CharacterRegistry::default();
    let svg = render_comic_strip_to_svg_with_registry(&strip, &registry);

    assert!(svg.contains("character-pose"));
    assert!(svg.contains("char-tuffy"));
    assert!(svg.contains("strip-title"));
}

#[test]
fn test_http_get_characters_endpoint() {
    let handle = run_server_on_random_port().expect("server should start");
    let port = handle.port();

    let url = format!("http://127.0.0.1:{}/characters", port);
    let resp = ureq::get(&url)
        .call()
        .expect("GET /characters request should succeed");

    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.header("content-type"),
        Some("application/json; charset=utf-8")
    );
    assert_eq!(resp.header("access-control-allow-origin"), Some("*"));

    let body = resp.into_string().expect("response body as string");
    assert!(body.contains("tuffy"));
    assert!(body.contains("jordan"));
    assert!(body.contains("peep"));

    handle.stop();
}
