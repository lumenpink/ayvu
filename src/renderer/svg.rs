use crate::balloon::{escape_xml, render_balloon_svg, Point};
use crate::model::{ComicStrip, Emotion, Panel};

pub const PANEL_WIDTH: f64 = 280.0;
pub const PANEL_HEIGHT: f64 = 280.0;
pub const PANEL_MARGIN: f64 = 16.0;

/// Render an entire ComicStrip to a standalone SVG string
pub fn render_comic_strip_to_svg(strip: &ComicStrip) -> String {
    let panel_count = strip.panels.len().max(1);
    let title_height = if strip.title.is_some() { 48.0 } else { 0.0 };

    let total_width =
        (PANEL_WIDTH * panel_count as f64) + (PANEL_MARGIN * (panel_count as f64 + 1.0));
    let total_height = PANEL_HEIGHT + (PANEL_MARGIN * 2.0) + title_height;

    let mut out = String::new();

    out.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {total_width:.1} {total_height:.1}" width="{total_width:.1}" height="{total_height:.1}" style="background-color: #fdfbf7;">"##,
        total_width = total_width,
        total_height = total_height
    ));

    out.push_str(r##"<defs><style>
        .strip-title { font-family: 'Comic Sans MS', 'Comic Relief', 'Impact', sans-serif; font-size: 18px; font-weight: bold; text-anchor: middle; text-transform: uppercase; fill: #111111; letter-spacing: 1.5px; }
        .panel-bg { fill: #ffffff; stroke: #111111; stroke-width: 2.5; rx: 4px; }
        .speaker-label { font-family: 'Comic Sans MS', sans-serif; font-size: 11px; font-weight: bold; fill: #222222; text-anchor: middle; }
        .emotion-badge { font-family: sans-serif; font-size: 9px; fill: #666666; font-style: italic; text-anchor: middle; }
    </style></defs>"##);

    // Render Title
    if let Some(ref title) = strip.title {
        let title_x = total_width / 2.0;
        let title_y = 30.0;
        out.push_str(&format!(
            r##"<text x="{title_x:.1}" y="{title_y:.1}" class="strip-title">★ {} ★</text>"##,
            escape_xml(title)
        ));
        out.push_str(&format!(
            r##"<line x1="{}" y1="40" x2="{}" y2="40" stroke="#111111" stroke-width="1.5" stroke-dasharray="4,4" />"##,
            PANEL_MARGIN,
            total_width - PANEL_MARGIN
        ));
    }

    // Render Panels
    for (idx, panel) in strip.panels.iter().enumerate() {
        let px = PANEL_MARGIN + (idx as f64 * (PANEL_WIDTH + PANEL_MARGIN));
        let py = PANEL_MARGIN + title_height;

        render_single_panel(&mut out, panel, px, py, idx + 1);
    }

    out.push_str("</svg>");
    out
}

fn render_single_panel(out: &mut String, panel: &Panel, px: f64, py: f64, panel_num: usize) {
    // 1. Panel frame
    out.push_str(&format!(r##"<g id="panel-{panel_num}">"##));
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{PANEL_WIDTH:.1}" height="{PANEL_HEIGHT:.1}" class="panel-bg" />"##
    ));

    // Panel number in top-right corner
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" font-size="10" fill="#999999" font-family="sans-serif ">{panel_num}</text>"##,
        px + PANEL_WIDTH - 14.0,
        py + 14.0
    ));

    // 2. Identify characters present in this panel
    let mut speakers: Vec<String> = Vec::new();
    for d in &panel.dialogues {
        if !speakers.contains(&d.speaker) {
            speakers.push(d.speaker.clone());
        }
    }

    // Positions for up to 2 characters (left and right)
    let char_left_pos = Point {
        x: px + 65.0,
        y: py + 220.0,
    };
    let char_right_pos = Point {
        x: px + PANEL_WIDTH - 65.0,
        y: py + 220.0,
    };

    let get_char_anchor = |speaker: &str| -> Point {
        if speakers.len() > 1 && speaker == speakers[1] {
            char_right_pos
        } else {
            char_left_pos
        }
    };

    // 3. Render Characters
    for speaker in &speakers {
        let anchor = get_char_anchor(speaker);
        let emotion = panel
            .dialogues
            .iter()
            .rev()
            .find(|d| &d.speaker == speaker)
            .map(|d| &d.emotion)
            .unwrap_or(&Emotion::Neutral);

        render_character_avatar(out, speaker, emotion, anchor);
    }

    // 4. Render Balloons
    let mut balloon_y = py + 16.0;
    for (i, d) in panel.dialogues.iter().enumerate() {
        let anchor = get_char_anchor(&d.speaker);
        let origin = if anchor.x < (px + PANEL_WIDTH / 2.0) {
            Point {
                x: px + 12.0,
                y: balloon_y,
            }
        } else {
            Point {
                x: px + 40.0,
                y: balloon_y,
            }
        };

        let mouth_target = Point {
            x: anchor.x,
            y: anchor.y - 35.0,
        };

        let balloon_svg = render_balloon_svg(
            &d.text,
            d.balloon_type,
            origin,
            mouth_target,
            PANEL_WIDTH - 40.0,
        );

        out.push_str(&balloon_svg);
        balloon_y += 75.0;
        if i >= 1 {
            break;
        }
    }

    out.push_str("</g>");
}

fn render_character_avatar(out: &mut String, name: &str, emotion: &Emotion, pos: Point) {
    let cx = pos.x;
    let cy = pos.y;

    out.push_str(&format!(
        r##"<g class="character" id="char-{}">"##,
        escape_xml(name)
    ));

    // Head
    out.push_str(&format!(
        r##"<circle cx="{cx:.1}" cy="{:.1}" r="22" fill="#ffd1b3" stroke="#111111" stroke-width="2" />"##,
        cy - 20.0
    ));

    // Simple expressive facial features based on emotion
    match emotion {
        Emotion::Happy | Emotion::Laughing => {
            // Smile
            out.push_str(&format!(
                r##"<path d="M {:.1} {:.1} Q {cx:.1} {:.1} {:.1} {:.1}" stroke="#111111" stroke-width="2" fill="none" />"##,
                cx - 8.0, cy - 16.0,
                cy - 8.0,
                cx + 8.0, cy - 16.0
            ));
            // Eyes
            out.push_str(&format!(
                r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" stroke="#111111" stroke-width="2" fill="none" />"##,
                cx - 10.0, cy - 23.0, cx - 7.0, cy - 27.0, cx - 4.0, cy - 23.0
            ));
            out.push_str(&format!(
                r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" stroke="#111111" stroke-width="2" fill="none" />"##,
                cx + 4.0, cy - 23.0, cx + 7.0, cy - 27.0, cx + 10.0, cy - 23.0
            ));
        }
        Emotion::Sad | Emotion::Crying => {
            // Frown
            out.push_str(&format!(
                r##"<path d="M {:.1} {:.1} Q {cx:.1} {:.1} {:.1} {:.1}" stroke="#111111" stroke-width="2" fill="none" />"##,
                cx - 7.0, cy - 12.0,
                cy - 18.0,
                cx + 7.0, cy - 12.0
            ));
            // Eyes
            out.push_str(&format!(
                r##"<circle cx="{:.1}" cy="{:.1}" r="2" fill="#111111" /><circle cx="{:.1}" cy="{:.1}" r="2" fill="#111111" />"##,
                cx - 7.0, cy - 23.0, cx + 7.0, cy - 23.0
            ));
        }
        Emotion::Angry | Emotion::Shouting => {
            // Open shout mouth
            out.push_str(&format!(
                r##"<ellipse cx="{cx:.1}" cy="{:.1}" rx="6" ry="5" fill="#aa0000" stroke="#111111" stroke-width="1.5" />"##,
                cy - 13.0
            ));
            // Slanted angry eyebrows
            out.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2" />"##,
                cx - 10.0, cy - 28.0, cx - 3.0, cy - 24.0
            ));
            out.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2" />"##,
                cx + 10.0, cy - 28.0, cx + 3.0, cy - 24.0
            ));
        }
        _ => {
            // Neutral face
            out.push_str(&format!(
                r##"<circle cx="{:.1}" cy="{:.1}" r="2.5" fill="#111111" /><circle cx="{:.1}" cy="{:.1}" r="2.5" fill="#111111" />"##,
                cx - 7.0, cy - 23.0, cx + 7.0, cy - 23.0
            ));
            out.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2" />"##,
                cx - 6.0, cy - 13.0, cx + 6.0, cy - 13.0
            ));
        }
    }

    // Torso / Shoulders
    out.push_str(&format!(
        r##"<path d="M {:.1} {cy:.1} C {:.1} {:.1}, {:.1} {:.1}, {:.1} {cy:.1} Z" fill="#4466aa" stroke="#111111" stroke-width="2" />"##,
        cx - 24.0,
        cx - 18.0, cy + 2.0,
        cx + 18.0, cy + 2.0,
        cx + 24.0
    ));

    // Name Badge
    out.push_str(&format!(
        r##"<text x="{cx:.1}" y="{:.1}" class="speaker-label">{}</text>"##,
        cy + 18.0,
        escape_xml(name)
    ));
    out.push_str(&format!(
        r##"<text x="{cx:.1}" y="{:.1}" class="emotion-badge">({})</text>"##,
        cy + 29.0,
        escape_xml(emotion.as_str())
    ));

    out.push_str("</g>");
}
