use crate::balloon::{escape_xml, render_balloon_svg, Point};
use crate::character::CharacterRegistry;
use crate::model::{ComicStrip, Emotion, Panel};

pub const PANEL_WIDTH: f64 = 280.0;
pub const PANEL_HEIGHT: f64 = 280.0;
pub const PANEL_MARGIN: f64 = 16.0;

/// Render an entire ComicStrip to a standalone SVG string using the default embedded character registry
pub fn render_comic_strip_to_svg(strip: &ComicStrip) -> String {
    let registry = CharacterRegistry::default();
    render_comic_strip_to_svg_with_registry(strip, &registry)
}

/// Render an entire ComicStrip to a standalone SVG string using a provided CharacterRegistry
pub fn render_comic_strip_to_svg_with_registry(
    strip: &ComicStrip,
    registry: &CharacterRegistry,
) -> String {
    let mut registry_clone = registry.clone();
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

        render_single_panel(&mut out, panel, px, py, idx + 1, &mut registry_clone);
    }

    out.push_str("</svg>");
    out
}

struct CharacterSlot {
    speaker: String,
    char_id: String,
    char_x: f64,
    char_y: f64,
    mouth: Point,
    pose_svg: String,
    emotion: Emotion,
}

fn render_single_panel(
    out: &mut String,
    panel: &Panel,
    px: f64,
    py: f64,
    panel_num: usize,
    registry: &mut CharacterRegistry,
) {
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

    // 3. Resolve characters and positioning
    let mut slots: Vec<CharacterSlot> = Vec::new();
    let num_speakers = speakers.len();

    for (idx, speaker) in speakers.iter().enumerate() {
        let pack = registry.resolve_speaker(speaker);
        let char_id = pack.id.clone();
        let anchor = pack.balloon_anchor;

        let (char_x, char_y) = if num_speakers == 1 {
            (px + 95.0, py + 145.0)
        } else if idx == 0 {
            (px + 15.0, py + 145.0)
        } else {
            (px + 175.0, py + 145.0)
        };

        let mouth = Point {
            x: char_x + (anchor.x * 90.0),
            y: char_y + (anchor.y * 108.0),
        };

        let emotion = panel
            .dialogues
            .iter()
            .rev()
            .find(|d| &d.speaker == speaker)
            .map(|d| d.emotion.clone())
            .unwrap_or(Emotion::Neutral);

        let pose_svg = pack.get_pose_svg(&emotion).to_string();

        slots.push(CharacterSlot {
            speaker: speaker.clone(),
            char_id,
            char_x,
            char_y,
            mouth,
            pose_svg,
            emotion,
        });
    }

    // 4. Render Character Avatars
    for slot in &slots {
        out.push_str(&format!(
            r##"<g class="character-pose" id="char-{}" transform="translate({:.1}, {:.1}) scale(0.9)">"##,
            escape_xml(&slot.char_id),
            slot.char_x,
            slot.char_y
        ));
        out.push_str(&slot.pose_svg);
        out.push_str("</g>");

        // Speaker label below avatar
        out.push_str(&format!(
            r##"<text x="{:.1}" y="{:.1}" class="speaker-label">{}</text>"##,
            slot.char_x + 45.0,
            slot.char_y + 116.0,
            escape_xml(&slot.speaker)
        ));
        out.push_str(&format!(
            r##"<text x="{:.1}" y="{:.1}" class="emotion-badge">({})</text>"##,
            slot.char_x + 45.0,
            slot.char_y + 127.0,
            escape_xml(slot.emotion.as_str())
        ));
    }

    // 5. Render Balloons
    let mut balloon_y = py + 16.0;
    for (i, d) in panel.dialogues.iter().enumerate() {
        let slot = slots.iter().find(|s| s.speaker == d.speaker);
        let mouth_target = if let Some(s) = slot {
            s.mouth
        } else {
            Point {
                x: px + 65.0,
                y: py + 180.0,
            }
        };

        let origin = if mouth_target.x < (px + PANEL_WIDTH / 2.0) {
            Point {
                x: px + 12.0,
                y: balloon_y,
            }
        } else {
            Point {
                x: px + 35.0,
                y: balloon_y,
            }
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
