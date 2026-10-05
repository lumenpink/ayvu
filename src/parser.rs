use crate::model::{
    BalloonType, ComicStrip, DialogueLine, Emotion, Panel, PanelWidth, StickerType, StripLayout,
    VisualEffect,
};

/// Parse plain text dialogue script into a structured ComicStrip
pub fn parse_dialogue_script(script: &str, title: Option<&str>) -> Result<ComicStrip, String> {
    let mut panels: Vec<Panel> = Vec::new();
    let mut current_dialogues: Vec<DialogueLine> = Vec::new();
    let mut current_effects: Vec<VisualEffect> = Vec::new();
    let mut explicit_panels = false;
    let mut strip_layout = StripLayout::Horizontal;
    let mut strip_backdrop: Option<String> = None;
    let mut current_backdrop: Option<String> = None;
    let mut current_characters: Vec<String> = Vec::new();
    let mut global_characters: Vec<String> = Vec::new();
    let mut current_width: Option<PanelWidth> = None;

    for raw_line in script.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        // Layout directive: [layout: grid] or [layout: vertical] or [layout: horizontal]
        if line.to_lowercase().starts_with("[layout:") && line.ends_with(']') {
            let inner = &line[8..line.len() - 1].trim();
            strip_layout = StripLayout::from_str_loose(inner);
            continue;
        }

        // Backdrop directive: [backdrop: pastoral] or [backdrop: sala_de_aula]
        if line.to_lowercase().starts_with("[backdrop:") && line.ends_with(']') {
            let inner = line[10..line.len() - 1].trim().to_lowercase();
            if explicit_panels || !panels.is_empty() {
                current_backdrop = Some(inner);
            } else {
                strip_backdrop = Some(inner.clone());
                current_backdrop = Some(inner);
            }
            continue;
        }

        // Characters / Cast directive: [characters: Tiki, Jordan, Armando] or [cast: ...]
        let line_lower = line.to_lowercase();
        if (line_lower.starts_with("[characters:") || line_lower.starts_with("[cast:"))
            && line.ends_with(']')
        {
            let colon_idx = line.find(':').unwrap();
            let inner = line[colon_idx + 1..line.len() - 1].trim();
            let parsed_chars: Vec<String> = inner
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if explicit_panels || !panels.is_empty() {
                current_characters = parsed_chars;
            } else {
                global_characters = parsed_chars;
            }
            continue;
        }

        // Panel boundaries: '---' or '[panel...]'
        if line == "---" || line.starts_with("[panel") {
            explicit_panels = true;
            let explicit_w = if line.starts_with("[panel") && line.ends_with(']') {
                let inner = line[6..line.len() - 1].trim();
                let opt = if let Some((_, o)) = inner.split_once(':') {
                    o.trim()
                } else if let Some((_, o)) = inner.split_once(' ') {
                    o.trim()
                } else {
                    inner
                };
                PanelWidth::from_str_loose(opt)
            } else {
                None
            };

            if !current_dialogues.is_empty()
                || !current_effects.is_empty()
                || !current_characters.is_empty()
                || current_backdrop.is_some()
            {
                let chars = if !current_characters.is_empty() {
                    std::mem::take(&mut current_characters)
                } else {
                    global_characters.clone()
                };
                let bg = current_backdrop.take().or_else(|| strip_backdrop.clone());
                let w = current_width.take();
                panels.push(Panel {
                    id: panels.len() + 1,
                    dialogues: std::mem::take(&mut current_dialogues),
                    effects: std::mem::take(&mut current_effects),
                    characters: chars,
                    backdrop: bg,
                    width: w,
                });
            }
            current_width = explicit_w;
            continue;
        }

        // Directives inside panel: [wide], [narrow], [standard], [panoramic], [width: ...]
        if line.starts_with('[') && line.ends_with(']') {
            let inner = &line[1..line.len() - 1].trim();
            if let Some(pw) = PanelWidth::from_str_loose(inner) {
                current_width = Some(pw);
                continue;
            }
            if let Some((prefix, val)) = inner.split_once(':') {
                let p = prefix.trim().to_lowercase();
                if p == "width"
                    || p == "largura"
                    || p == "size"
                    || p == "tamanho"
                    || p == "panel_width"
                {
                    if let Some(pw) = PanelWidth::from_str_loose(val) {
                        current_width = Some(pw);
                        continue;
                    }
                }
            }
            if inner.ends_with('!') {
                current_effects.push(VisualEffect::Onomatopoeia(inner.to_string()));
                continue;
            }
            if let Some(sticker) = StickerType::from_str_loose(inner) {
                current_effects.push(VisualEffect::Sticker(sticker));
                continue;
            }
        }

        // Parse speaker and speech: "Speaker (emotion): speech" or "Speaker: speech"
        if let Some((speaker_part, speech_part)) = line.split_once(':') {
            let speaker_raw = speaker_part.trim();
            let mut speech = speech_part.trim().to_string();

            if speaker_raw.is_empty() || speech.is_empty() {
                continue;
            }

            // Extract inline onomatopoeia or stickers from speech
            extract_inline_effects(&mut speech, &mut current_effects);

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

    if !current_dialogues.is_empty()
        || !current_effects.is_empty()
        || !current_characters.is_empty()
        || current_backdrop.is_some()
    {
        let chars = if !current_characters.is_empty() {
            std::mem::take(&mut current_characters)
        } else {
            global_characters.clone()
        };
        let bg = current_backdrop.take().or_else(|| strip_backdrop.clone());
        let w = current_width.take();
        panels.push(Panel {
            id: panels.len() + 1,
            dialogues: current_dialogues,
            effects: current_effects,
            characters: chars,
            backdrop: bg,
            width: w,
        });
    }

    // If no explicit panel boundaries were specified, auto-group into panels
    let final_panels = if !explicit_panels && panels.len() == 1 {
        let first = panels.remove(0);
        let all_dialogues = first.dialogues;
        let mut effects = first.effects;
        let p_chars = first.characters;
        let p_bg = first.backdrop;
        let mut grouped = Vec::new();
        let mut chunk = Vec::new();

        let max_per_panel = if p_chars.len() >= 3 { 3 } else { 2 };

        for d in all_dialogues {
            chunk.push(d);
            if chunk.len() >= max_per_panel {
                grouped.push(Panel {
                    id: grouped.len() + 1,
                    dialogues: std::mem::take(&mut chunk),
                    effects: std::mem::take(&mut effects),
                    characters: p_chars.clone(),
                    backdrop: p_bg.clone(),
                    width: None,
                });
            }
        }
        if !chunk.is_empty() || !effects.is_empty() {
            grouped.push(Panel {
                id: grouped.len() + 1,
                dialogues: chunk,
                effects,
                characters: p_chars,
                backdrop: p_bg,
                width: None,
            });
        }
        grouped
    } else {
        panels
    };

    Ok(ComicStrip {
        title: title.map(|t| t.to_string()),
        layout: strip_layout,
        backdrop: strip_backdrop,
        panels: final_panels,
    })
}

fn extract_inline_effects(speech: &mut String, effects: &mut Vec<VisualEffect>) {
    let mut modified = speech.clone();
    let mut found_stickers = Vec::new();
    let mut found_sfx = Vec::new();

    let words: Vec<&str> = speech.split_whitespace().collect();
    for word in words {
        if word.starts_with('[') && word.ends_with(']') {
            let inner = &word[1..word.len() - 1].trim();
            if inner.ends_with('!') {
                found_sfx.push((word.to_string(), inner.to_string()));
            } else if let Some(sticker) = StickerType::from_str_loose(inner) {
                found_stickers.push((word.to_string(), sticker));
            }
        }
    }

    for (token, sfx) in found_sfx {
        modified = modified.replace(&token, "").trim().to_string();
        effects.push(VisualEffect::Onomatopoeia(sfx));
    }

    for (token, sticker) in found_stickers {
        modified = modified.replace(&token, "").trim().to_string();
        effects.push(VisualEffect::Sticker(sticker));
    }

    *speech = modified;
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

    // 2. Shout prefix or suffix: multiple exclamation marks or ALL CAPS with !
    if trimmed.ends_with("!!!")
        || trimmed.ends_with("!!")
        || (trimmed.ends_with('!')
            && trimmed
                .chars()
                .all(|c| !c.is_alphabetic() || c.is_uppercase()))
    {
        return (Emotion::Shouting, BalloonType::Shout);
    }

    // 3. Emoticons heuristics
    if trimmed.contains(":D") || trimmed.contains("xD") || trimmed.contains("XD") {
        return (Emotion::Laughing, BalloonType::Speech);
    }
    if trimmed.contains(":) ") || trimmed.ends_with(":)") || trimmed.contains(":-)") {
        return (Emotion::Happy, BalloonType::Speech);
    }
    if trimmed.contains(":( ") || trimmed.ends_with(":(") || trimmed.contains(":-(") {
        return (Emotion::Sad, BalloonType::Speech);
    }
    if trimmed.contains(":'(") || trimmed.contains("T_T") || trimmed.contains(";\n") {
        return (Emotion::Crying, BalloonType::Speech);
    }
    if trimmed.contains(">:( ") || trimmed.ends_with(">:(") {
        return (Emotion::Angry, BalloonType::Speech);
    }
    if trimmed.contains(":o") || trimmed.contains(":O") || trimmed.contains("=O") {
        return (Emotion::Surprised, BalloonType::Speech);
    }

    (Emotion::Neutral, BalloonType::Speech)
}
