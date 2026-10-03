use ayvu::model::{BalloonType, ComicStrip, Emotion};
use ayvu::parser::parse_dialogue_script;

#[test]
fn test_parse_explicit_emotion_and_dialogue() {
    let script = "Alice (happy): Hello world!";
    let strip: ComicStrip = parse_dialogue_script(script, None).expect("failed to parse script");

    assert_eq!(strip.panels.len(), 1);
    let panel = &strip.panels[0];
    assert_eq!(panel.dialogues.len(), 1);

    let d = &panel.dialogues[0];
    assert_eq!(d.speaker, "Alice");
    assert_eq!(d.emotion, Emotion::Happy);
    assert_eq!(d.balloon_type, BalloonType::Speech);
    assert_eq!(d.text, "Hello world!");
}

#[test]
fn test_parse_explicit_thought() {
    let script = "Bob (thinking): Is this real?";
    let strip = parse_dialogue_script(script, None).expect("failed to parse script");

    assert_eq!(strip.panels.len(), 1);
    let d = &strip.panels[0].dialogues[0];
    assert_eq!(d.speaker, "Bob");
    assert_eq!(d.emotion, Emotion::Thinking);
    assert_eq!(d.balloon_type, BalloonType::Thought);
    assert_eq!(d.text, "Is this real?");
}

#[test]
fn test_parse_heuristic_thought_prefix() {
    let script = "Bob: oO What was that?";
    let strip = parse_dialogue_script(script, None).expect("failed to parse script");

    let d = &strip.panels[0].dialogues[0];
    assert_eq!(d.speaker, "Bob");
    assert_eq!(d.emotion, Emotion::Thinking);
    assert_eq!(d.balloon_type, BalloonType::Thought);
    assert_eq!(d.text, "What was that?");
}

#[test]
fn test_parse_heuristic_shout() {
    let script = "Alice: STOP RIGHT NOW!";
    let strip = parse_dialogue_script(script, None).expect("failed to parse script");

    let d = &strip.panels[0].dialogues[0];
    assert_eq!(d.speaker, "Alice");
    assert_eq!(d.emotion, Emotion::Shouting);
    assert_eq!(d.balloon_type, BalloonType::Shout);
}

#[test]
fn test_parse_heuristic_emoticons() {
    let script_laugh = "Carol: That was so funny :D";
    let strip_laugh = parse_dialogue_script(script_laugh, None).unwrap();
    assert_eq!(
        strip_laugh.panels[0].dialogues[0].emotion,
        Emotion::Laughing
    );

    let script_sad = "Dave: Nothing is working :(";
    let strip_sad = parse_dialogue_script(script_sad, None).unwrap();
    assert_eq!(strip_sad.panels[0].dialogues[0].emotion, Emotion::Sad);
}

#[test]
fn test_parse_explicit_panel_separator() {
    let script = "Alice: First panel.\n---\nBob: Second panel.";
    let strip = parse_dialogue_script(script, Some("Two Panels")).expect("failed to parse");

    assert_eq!(strip.title.as_deref(), Some("Two Panels"));
    assert_eq!(strip.panels.len(), 2);
    assert_eq!(strip.panels[0].dialogues[0].text, "First panel.");
    assert_eq!(strip.panels[1].dialogues[0].text, "Second panel.");
}

#[test]
fn test_parse_auto_grouping_panels() {
    let script = r#"
        Alice: Line 1
        Bob: Line 2
        Alice: Line 3
        Bob: Line 4
    "#;
    let strip = parse_dialogue_script(script, None).expect("failed to parse");

    // 4 lines auto-grouped into 2 lines per panel = 2 panels
    assert_eq!(strip.panels.len(), 2);
    assert_eq!(strip.panels[0].dialogues.len(), 2);
    assert_eq!(strip.panels[1].dialogues.len(), 2);
}
