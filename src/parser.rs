use crate::model::{BalloonType, ComicStrip, DialogueLine, Emotion, Panel};

/// Parse plain text dialogue script into a structured ComicStrip
pub fn parse_dialogue_script(script: &str, title: Option<&str>) -> Result<ComicStrip, String> {
    let mut panels: Vec<Panel> = Vec::new();
    let mut current_dialogues: Vec<DialogueLine> = Vec::new();
    let mut explicit_panels = false;

    for raw_line in script.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        // Panel boundaries: '---' or '[panel...]'
        if line == "---" || line.starts_with("[panel") {
            explicit_panels = true;
            if !current_dialogues.is_empty() {
                panels.push(Panel {
                    id: panels.len() + 1,
                    dialogues: std::mem::take(&mut current_dialogues),
                });
            }
            continue;
        }

        // Parse speaker and speech: "Speaker (emotion): speech" or "Speaker: speech"
        if let Some((speaker_part, speech_part)) = line.split_once(':') {
            let speaker_raw = speaker_part.trim();
            let mut speech = speech_part.trim().to_string();

            if speaker_raw.is_empty() || speech.is_empty() {
                continue;
            }

            let (speaker, explicit_emotion) = if let Some(open_idx) = speaker_raw.find('(') {
                if let Some(close_idx) = speaker_raw[open_idx..].find(')') {
                    let sp = speaker_raw[..open_idx].trim().to_string();
                    let em = speaker_raw[open_idx + 1..open_idx + close_idx]
                        .trim()
                        .to_string();
                    (sp, Some(em))
                } else {
                    (speaker_raw.to_string(), None)
                }
            } else {
                (speaker_raw.to_string(), None)
            };

            let (emotion, balloon_type) = if let Some(em_str) = explicit_emotion {
                let parsed_em = Emotion::from_str_loose(&em_str);
                let b_type = match parsed_em {
                    Emotion::Thinking => BalloonType::Thought,
                    Emotion::Shouting => BalloonType::Shout,
                    _ => BalloonType::Speech,
                };
                (parsed_em, b_type)
            } else {
                // Heuristic detection
                detect_emotion_and_balloon(&mut speech)
            };

            current_dialogues.push(DialogueLine {
                speaker,
                emotion,
                balloon_type,
                text: speech,
            });
        }
    }

    if !current_dialogues.is_empty() {
        panels.push(Panel {
            id: panels.len() + 1,
            dialogues: current_dialogues,
        });
    }

    // If no explicit panel boundaries were specified, auto-group into panels of up to 2 dialogues
    let final_panels = if !explicit_panels && panels.len() == 1 {
        let all_dialogues = panels.remove(0).dialogues;
        let mut grouped = Vec::new();
        let mut chunk = Vec::new();

        for d in all_dialogues {
            chunk.push(d);
            if chunk.len() >= 2 {
                grouped.push(Panel {
                    id: grouped.len() + 1,
                    dialogues: std::mem::take(&mut chunk),
                });
            }
        }
        if !chunk.is_empty() {
            grouped.push(Panel {
                id: grouped.len() + 1,
                dialogues: chunk,
            });
        }
        grouped
    } else {
        panels
    };

    Ok(ComicStrip {
        title: title.map(|t| t.to_string()),
        panels: final_panels,
    })
}

fn detect_emotion_and_balloon(text: &mut String) -> (Emotion, BalloonType) {
    let trimmed = text.trim();

    // 1. Thought prefix: oO or o.O
    if trimmed.starts_with("oO") || trimmed.starts_with("o.O") {
        let clean = trimmed
            .trim_start_matches("oO")
            .trim_start_matches("o.O")
            .trim();
        *text = clean.to_string();
        return (Emotion::Thinking, BalloonType::Thought);
    }

    // 2. Shout: ALL CAPS + '!' or '!!!'
    let letters: String = trimmed.chars().filter(|c| c.is_alphabetic()).collect();
    let is_caps = letters.len() >= 3 && letters.chars().all(|c| c.is_uppercase());
    if (is_caps && trimmed.contains('!')) || trimmed.ends_with("!!!") {
        return (Emotion::Shouting, BalloonType::Shout);
    }

    // 3. Emoticons & Keywords
    if trimmed.contains(":D")
        || trimmed.contains("xD")
        || trimmed.contains("haha")
        || trimmed.contains("kkk")
    {
        return (Emotion::Laughing, BalloonType::Speech);
    }

    if trimmed.contains(":'(") {
        return (Emotion::Crying, BalloonType::Speech);
    }

    if trimmed.contains(":(") || trimmed.contains(":-(") {
        return (Emotion::Sad, BalloonType::Speech);
    }

    if trimmed.contains(">:") || trimmed.contains("x(") {
        return (Emotion::Angry, BalloonType::Speech);
    }

    if trimmed.contains(":)") || trimmed.contains(":-)") || trimmed.contains(r"\o/") {
        return (Emotion::Happy, BalloonType::Speech);
    }

    if trimmed.contains("???") {
        return (Emotion::Surprised, BalloonType::Speech);
    }

    if trimmed.starts_with("...") || trimmed.contains("hmm") {
        return (Emotion::Thinking, BalloonType::Thought);
    }

    (Emotion::Neutral, BalloonType::Speech)
}
