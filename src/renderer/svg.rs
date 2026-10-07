use crate::balloon::{escape_xml, measure_balloon, render_balloon_svg, Point};
use crate::character::CharacterRegistry;
use crate::model::{
    ComicStrip, DialogueLine, Emotion, Panel, StickerType, StripLayout, VisualEffect,
};

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
    let title_height = if strip.title.is_some() { 48.0 } else { 0.0 };

    // Compute (px, py, pw) for each panel according to strip layout
    let (panel_layouts, total_width, total_height) = match strip.layout {
        StripLayout::Horizontal => {
            let mut layouts = Vec::with_capacity(strip.panels.len());
            let mut cur_x = PANEL_MARGIN;
            for panel in &strip.panels {
                let pw = panel.resolve_width();
                let py = PANEL_MARGIN + title_height;
                layouts.push((cur_x, py, pw));
                cur_x += pw + PANEL_MARGIN;
            }
            let w = if strip.panels.is_empty() {
                PANEL_WIDTH + (PANEL_MARGIN * 2.0)
            } else {
                cur_x
            };
            let h = PANEL_HEIGHT + (PANEL_MARGIN * 2.0) + title_height;
            (layouts, w, h)
        }
        StripLayout::Vertical => {
            let max_w = strip
                .panels
                .iter()
                .map(|p| p.resolve_width())
                .fold(PANEL_WIDTH, f64::max);
            let w = max_w + (PANEL_MARGIN * 2.0);
            let mut layouts = Vec::with_capacity(strip.panels.len());
            let mut cur_y = PANEL_MARGIN + title_height;
            for panel in &strip.panels {
                let pw = panel.resolve_width();
                let px = PANEL_MARGIN + (max_w - pw) / 2.0;
                layouts.push((px, cur_y, pw));
                cur_y += PANEL_HEIGHT + PANEL_MARGIN;
            }
            let h = if strip.panels.is_empty() {
                PANEL_HEIGHT + (PANEL_MARGIN * 2.0) + title_height
            } else {
                cur_y
            };
            (layouts, w, h)
        }
        StripLayout::Grid => {
            let rows: Vec<&[Panel]> = strip.panels.chunks(2).collect();
            let mut max_row_w: f64 = PANEL_WIDTH * 2.0 + PANEL_MARGIN * 3.0;
            for row in &rows {
                let row_w: f64 = row.iter().map(|p| p.resolve_width()).sum::<f64>()
                    + (row.len() as f64 + 1.0) * PANEL_MARGIN;
                if row_w > max_row_w {
                    max_row_w = row_w;
                }
            }
            let total_w = max_row_w;
            let mut layouts = Vec::with_capacity(strip.panels.len());
            let mut cur_y = PANEL_MARGIN + title_height;
            for row in &rows {
                let row_panels_w: f64 = row.iter().map(|p| p.resolve_width()).sum();
                let extra_space = total_w - row_panels_w - (row.len() as f64 + 1.0) * PANEL_MARGIN;
                let mut cur_x = PANEL_MARGIN + (extra_space / 2.0).max(0.0);
                for panel in *row {
                    let pw = panel.resolve_width();
                    layouts.push((cur_x, cur_y, pw));
                    cur_x += pw + PANEL_MARGIN;
                }
                cur_y += PANEL_HEIGHT + PANEL_MARGIN;
            }
            let h = if strip.panels.is_empty() {
                PANEL_HEIGHT + (PANEL_MARGIN * 2.0) + title_height
            } else {
                cur_y
            };
            (layouts, total_w, h)
        }
    };

    let mut out = String::new();

    out.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {total_width:.1} {total_height:.1}" width="{total_width:.1}" height="{total_height:.1}" style="background-color: #fdfbf7;">"##,
        total_width = total_width,
        total_height = total_height
    ));

    out.push_str(r##"<defs><style>
        .strip-title { font-family: 'Comic Sans MS', 'Comic Relief', 'Impact', sans-serif; font-size: 18px; font-weight: bold; text-anchor: middle; text-transform: uppercase; fill: #111111; letter-spacing: 1.5px; }
        .panel-bg { fill: #ffffff; }
        .panel-floor { stroke: #444444; stroke-width: 1.8; stroke-linecap: round; }
        .panel-floor-wash { fill: #f8f6f0; opacity: 0.75; }
        .panel-frame-ink { stroke: #111111; stroke-width: 2.5; stroke-linecap: square; }
        .panel-corner-tick { stroke: #111111; stroke-width: 2.5; }
        .sfx-text { font-family: 'Impact', 'Arial Black', sans-serif; font-size: 24px; font-weight: 900; text-anchor: middle; fill: #d90429; stroke: #ffffff; stroke-width: 1.5; letter-spacing: 2px; }
        .comic-prop { pointer-events: none; }
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
    for (idx, (panel, &(px, py, pw))) in strip.panels.iter().zip(&panel_layouts).enumerate() {
        let effective_backdrop = panel.backdrop.as_deref().or(strip.backdrop.as_deref());

        render_single_panel(
            &mut out,
            panel,
            px,
            py,
            pw,
            idx + 1,
            effective_backdrop,
            &mut registry_clone,
        );
    }

    out.push_str("</svg>");
    out
}

struct CharacterSlot {
    speaker: String,
    char_id: String,
    char_x: f64,
    char_y: f64,
    scale: f64,
    scaled_w: f64,
    mouth: Point,
    pose_svg: String,
    flipped: bool,
}

fn render_single_panel(
    out: &mut String,
    panel: &Panel,
    px: f64,
    py: f64,
    pw: f64,
    panel_num: usize,
    backdrop: Option<&str>,
    registry: &mut CharacterRegistry,
) {
    // 1. Panel background and floor staging
    out.push_str(&format!(r##"<g id="panel-{panel_num}">"##));
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{pw:.1}" height="{PANEL_HEIGHT:.1}" class="panel-bg" />"##
    ));

    // Optional vector backdrop (UFSC academic / vector pastoral / room) or authentic Jim Woodring bitmap
    let has_backdrop = if let Some(bg_name) = backdrop {
        if let Some(vector_svg) =
            crate::backdrop::get_vector_backdrop_svg(bg_name, px, py, pw, PANEL_HEIGHT)
        {
            out.push_str(&vector_svg);
            true
        } else if let Some(b64) = crate::assets::get_backdrop_b64(bg_name) {
            out.push_str(&format!(
                r##"<image href="data:image/png;base64,{b64}" x="{px:.1}" y="{py:.1}" width="{pw:.1}" height="{PANEL_HEIGHT:.1}" preserveAspectRatio="xMidYMid slice" opacity="0.95" />"##
            ));
            true
        } else {
            false
        }
    } else {
        false
    };

    let floor_y = if has_backdrop {
        py + PANEL_HEIGHT - 16.0
    } else {
        py + PANEL_HEIGHT - 24.0
    };

    if !has_backdrop {
        // Floor area wash & floor line
        out.push_str(&format!(
            r##"<rect x="{px:.1}" y="{floor_y:.1}" width="{pw:.1}" height="24.0" class="panel-floor-wash" />"##
        ));
        out.push_str(&format!(
            r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" class="panel-floor" stroke-dasharray="12,3" />"##,
            px + pw
        ));
    }

    // Hand-inked panel frame with draft corner ticks (extending 4px beyond vertices)
    let tick = 4.0;
    out.push_str(&format!(
        r##"<g class="panel-frame-ink">
<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" class="panel-corner-tick" />
<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" class="panel-corner-tick" />
<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" class="panel-corner-tick" />
<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" class="panel-corner-tick" />
</g>"##,
        px - tick,
        py,
        px + pw + tick,
        py,
        px - tick,
        py + PANEL_HEIGHT,
        px + pw + tick,
        py + PANEL_HEIGHT,
        px,
        py - tick,
        px,
        py + PANEL_HEIGHT + tick,
        px + pw,
        py - tick,
        px + pw,
        py + PANEL_HEIGHT + tick
    ));

    // Panel number in top-right corner
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" font-size="10" font-weight="bold" fill="#aaaaaa" font-family="sans-serif">{panel_num}</text>"##,
        px + pw - 14.0,
        py + 14.0
    ));

    // 2. Identify characters present in this panel
    let mut speakers: Vec<String> = Vec::new();
    if !panel.characters.is_empty() {
        for c in &panel.characters {
            if !speakers.iter().any(|s| s.eq_ignore_ascii_case(c)) {
                speakers.push(c.clone());
            }
        }
    }
    for d in &panel.dialogues {
        if !speakers.iter().any(|s| s.eq_ignore_ascii_case(&d.speaker)) {
            speakers.push(d.speaker.clone());
        }
    }

    // 3. Resolve characters and positioning according to Comic Chat LayoutAvatars
    let mut slots: Vec<CharacterSlot> = Vec::new();
    let target_height = 125.0; // maxBodyHeight in Comic Chat (unitHeight / 1.9 = 240 / 1.9 ~ 126)

    let mut resolved_packs = Vec::new();
    let mut max_char_h: f64 = 400.0;
    for speaker in &speakers {
        let pack = registry.resolve_speaker(speaker);
        let dims = pack.dimensions();
        if dims.1 > max_char_h {
            max_char_h = dims.1;
        }
        resolved_packs.push((speaker.clone(), pack, dims));
    }

    // Comic Chat uniform scale ratio: newHeight = maxBodyHeight * (normHeight / maxNorm)
    let base_scale = target_height / max_char_h;

    // Check if sum of widths exceeds panel width
    let total_avatar_w: f64 = resolved_packs
        .iter()
        .map(|(_, _, d)| d.0 * base_scale)
        .sum();
    let max_allowed_w = pw - 24.0;
    let final_scale = if total_avatar_w > max_allowed_w && total_avatar_w > 0.0 {
        base_scale * (max_allowed_w / total_avatar_w)
    } else {
        base_scale
    };

    let actual_total_w: f64 = resolved_packs
        .iter()
        .map(|(_, _, d)| d.0 * final_scale)
        .sum();
    let margin = (pw - actual_total_w) / (resolved_packs.len() + 1).max(1) as f64;
    let mut cur_x = px + margin;
    let n_speakers = resolved_packs.len();

    for (idx, (speaker, pack, dims)) in resolved_packs.into_iter().enumerate() {
        let scale = final_scale;
        let scaled_w = dims.0 * scale;
        let scaled_h = dims.1 * scale;
        let char_id = pack.id.clone();
        let anchor = pack.balloon_anchor;
        let char_x = cur_x;
        let char_y = floor_y - scaled_h;

        // Facing direction:
        // In Comic Chat (m_flip), all Jim Woodring original base bitmaps face RIGHT.
        // In a two-character conversation, the character on the right faces LEFT (flipped)
        // so that they face each other directly.
        // In three-or-more character scenes, characters on the right half face LEFT,
        // characters on the left half face RIGHT, and center characters face their interlocutor.
        let flipped = if n_speakers <= 1 {
            false
        } else if n_speakers == 2 {
            idx == 1 // Right speaker faces Left speaker
        } else if idx < n_speakers / 2 {
            false // Left side faces right
        } else if idx > n_speakers / 2 {
            true // Right side faces left
        } else {
            // Center character in 3+ cast: face the first speaker or stage-left
            let first_speaker = panel
                .dialogues
                .first()
                .map(|d| d.speaker.as_str())
                .unwrap_or("");
            if let Some(first_idx) = speakers.iter().position(|s| s == first_speaker) {
                first_idx < idx
            } else {
                true
            }
        };

        // Pointer target points toward the mouth X position (adjusted for flip), terminating above the head apex!
        let mouth_x = if flipped {
            char_x + ((1.0 - anchor.x) * scaled_w)
        } else {
            char_x + (anchor.x * scaled_w)
        };
        let mouth = Point {
            x: mouth_x,
            y: char_y - 2.0,
        };

        let emotion = panel
            .dialogues
            .iter()
            .rev()
            .find(|d| d.speaker == speaker)
            .map(|d| d.emotion.clone())
            .unwrap_or(Emotion::Neutral);

        let pose_svg = pack.get_pose_svg(&emotion).to_string();

        slots.push(CharacterSlot {
            speaker,
            char_id,
            char_x,
            char_y,
            scale,
            scaled_w,
            mouth,
            pose_svg,
            flipped,
        });

        cur_x += scaled_w + margin;
    }

    // 4. Render Contact Shadows & Character Avatars
    for slot in &slots {
        if !has_backdrop {
            out.push_str(&format!(
                r##"<ellipse cx="{:.1}" cy="{:.1}" rx="{:.1}" ry="6" class="contact-shadow" fill="rgba(0,0,0,0.12)" />"##,
                slot.char_x + (slot.scaled_w / 2.0),
                floor_y + 1.0,
                (slot.scaled_w * 0.45).clamp(20.0, 45.0)
            ));
        }

        if slot.flipped {
            out.push_str(&format!(
                r##"<g class="character-pose flipped" id="char-{}" data-speaker="{}" aria-label="{}" transform="translate({:.1}, {:.1}) scale(-{:.4}, {:.4})">"##,
                escape_xml(&slot.char_id),
                escape_xml(&slot.speaker),
                escape_xml(&slot.speaker),
                slot.char_x + slot.scaled_w,
                slot.char_y,
                slot.scale,
                slot.scale
            ));
        } else {
            out.push_str(&format!(
                r##"<g class="character-pose" id="char-{}" data-speaker="{}" aria-label="{}" transform="translate({:.1}, {:.1}) scale({:.4})">"##,
                escape_xml(&slot.char_id),
                escape_xml(&slot.speaker),
                escape_xml(&slot.speaker),
                slot.char_x,
                slot.char_y,
                slot.scale
            ));
        }
        out.push_str(&slot.pose_svg);
        out.push_str("</g>");
    }

    // 4.5. Render Props (Academic & comic items in authentic nanquim style)
    let mut untargeted_count = 0;
    for prop in &panel.props {
        let is_targeted = if let Some(target_name) = &prop.target {
            slots
                .iter()
                .any(|s| s.speaker.eq_ignore_ascii_case(target_name))
        } else {
            false
        };
        if !is_targeted {
            untargeted_count += 1;
        }
    }

    let mut speaker_prop_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let mut untargeted_idx = 0;
    for prop in &panel.props {
        let (cx, cy, scale, flipped) = if let Some(target_name) = &prop.target {
            if let Some(slot) = slots
                .iter()
                .find(|s| s.speaker.eq_ignore_ascii_case(target_name))
            {
                // Character-held prop: position around hand / waist level
                let count = speaker_prop_counts.entry(slot.speaker.clone()).or_insert(0);
                let offset_x = (*count as f64) * 22.0;
                *count += 1;
                let (base_cx, flipped) = if slot.flipped {
                    (slot.char_x + (slot.scaled_w * 0.22) - offset_x, true)
                } else {
                    (slot.char_x + (slot.scaled_w * 0.78) + offset_x, false)
                };
                let cy = match prop.prop_type {
                    crate::model::PropType::Microphone => floor_y - 18.0,
                    crate::model::PropType::Backpack => floor_y - 28.0,
                    crate::model::PropType::Umbrella => floor_y - 36.0,
                    crate::model::PropType::Tray => floor_y - 30.0,
                    crate::model::PropType::Phone => floor_y - 34.0,
                    _ => floor_y - 32.0,
                };
                (base_cx.clamp(px + 16.0, px + pw - 16.0), cy, 0.95, flipped)
            } else {
                let idx = untargeted_idx;
                untargeted_idx += 1;
                let cx = if slots.len() == 1 {
                    if idx % 2 == 0 {
                        px + 36.0 + ((idx / 2) as f64 * 32.0)
                    } else {
                        px + pw - 36.0 - ((idx / 2) as f64 * 32.0)
                    }
                } else {
                    px + (pw * (idx + 1) as f64) / (untargeted_count + 1) as f64
                };
                let ground_offset = match prop.prop_type {
                    crate::model::PropType::Coffee
                    | crate::model::PropType::Book
                    | crate::model::PropType::Caderno => 10.0,
                    crate::model::PropType::Tray => 6.0,
                    crate::model::PropType::Laptop => 14.0,
                    crate::model::PropType::Erlenmeyer => 15.0,
                    crate::model::PropType::Chalk => 4.0,
                    crate::model::PropType::Stethoscope => 20.0,
                    crate::model::PropType::Microphone => 18.0,
                    crate::model::PropType::Phone => 12.0,
                    crate::model::PropType::Backpack => 18.0,
                    crate::model::PropType::Umbrella => 24.0,
                    crate::model::PropType::Custom(_) => 10.0,
                };
                let cy = floor_y - ground_offset;
                (cx, cy, 0.95, false)
            }
        } else {
            let idx = untargeted_idx;
            untargeted_idx += 1;
            let cx = if slots.len() == 1 {
                if idx % 2 == 0 {
                    px + 36.0 + ((idx / 2) as f64 * 32.0)
                } else {
                    px + pw - 36.0 - ((idx / 2) as f64 * 32.0)
                }
            } else {
                px + (pw * (idx + 1) as f64) / (untargeted_count + 1) as f64
            };
            let ground_offset = match prop.prop_type {
                crate::model::PropType::Coffee
                | crate::model::PropType::Book
                | crate::model::PropType::Caderno => 10.0,
                crate::model::PropType::Tray => 6.0,
                crate::model::PropType::Laptop => 14.0,
                crate::model::PropType::Erlenmeyer => 15.0,
                crate::model::PropType::Chalk => 4.0,
                crate::model::PropType::Stethoscope => 20.0,
                crate::model::PropType::Microphone => 18.0,
                crate::model::PropType::Phone => 12.0,
                crate::model::PropType::Backpack => 18.0,
                crate::model::PropType::Umbrella => 24.0,
                crate::model::PropType::Custom(_) => 10.0,
            };
            let cy = floor_y - ground_offset;
            (cx, cy, 0.95, false)
        };

        out.push_str(&crate::props::render_prop_svg(prop, cx, cy, scale, flipped));
    }

    // 5. Render Visual Effects (Onomatopoeia & Stickers)
    let mut sticker_offset_x = px + pw - 36.0;
    for effect in &panel.effects {
        match effect {
            VisualEffect::Onomatopoeia(text) => {
                render_onomatopoeia_starburst(out, text, px + pw / 2.0, py + 75.0);
            }
            VisualEffect::Sticker(sticker) => {
                render_sticker(out, *sticker, sticker_offset_x, py + 38.0);
                sticker_offset_x -= 30.0;
            }
        }
    }

    // 6. Render Balloons with Comic Chat Anti-Crossing Algorithm
    let num_dialogues = panel.dialogues.len();
    if num_dialogues == 1 {
        let d = &panel.dialogues[0];
        let slot = slots
            .iter()
            .find(|s| s.speaker.eq_ignore_ascii_case(&d.speaker));
        let mouth_target = if let Some(s) = slot {
            s.mouth
        } else {
            Point {
                x: px + 65.0,
                y: py + 180.0,
            }
        };

        let origin = if mouth_target.x < (px + pw / 2.0) {
            Point {
                x: px + 12.0,
                y: py + 14.0,
            }
        } else {
            Point {
                x: px + 40.0,
                y: py + 14.0,
            }
        };

        out.push_str(r##"<g class="balloon-container anti-crossing balloon-left">"##);
        let balloon_svg =
            render_balloon_svg(&d.text, d.balloon_type, origin, mouth_target, pw - 40.0);
        out.push_str(&balloon_svg);
        out.push_str("</g>");
    } else if num_dialogues == 2 {
        // Multi-speaker layout: Comic Chat Rule (No crossing tails)
        let d0 = &panel.dialogues[0];
        let d1 = &panel.dialogues[1];

        let slot0 = slots
            .iter()
            .find(|s| s.speaker.eq_ignore_ascii_case(&d0.speaker));
        let slot1 = slots
            .iter()
            .find(|s| s.speaker.eq_ignore_ascii_case(&d1.speaker));

        let mouth0 = slot0.map(|s| s.mouth).unwrap_or(Point {
            x: px + 50.0,
            y: py + 170.0,
        });
        let mouth1 = slot1.map(|s| s.mouth).unwrap_or(Point {
            x: px + pw - 60.0,
            y: py + 170.0,
        });

        let is_d0_left = mouth0.x <= mouth1.x;

        let max_balloon_w = ((pw - 28.0) / 2.0).clamp(90.0, 190.0);

        let (bw1, _, _) = measure_balloon(&d1.text, max_balloon_w);
        let (bw0, _, _) = measure_balloon(&d0.text, max_balloon_w);

        let (origin0, origin1) = if is_d0_left {
            // First speaker on left (top-left), second on right (top-right)
            (
                Point {
                    x: px + 10.0,
                    y: py + 10.0,
                },
                Point {
                    x: px + pw - bw1 - 10.0,
                    y: py + 12.0,
                },
            )
        } else {
            // First speaker on right (top-right), second on left (top-left)
            (
                Point {
                    x: px + pw - bw0 - 10.0,
                    y: py + 10.0,
                },
                Point {
                    x: px + 10.0,
                    y: py + 12.0,
                },
            )
        };

        // Render first balloon
        out.push_str(r##"<g class="balloon-container anti-crossing balloon-left">"##);
        out.push_str(&render_balloon_svg(
            &d0.text,
            d0.balloon_type,
            origin0,
            mouth0,
            max_balloon_w,
        ));
        out.push_str("</g>");

        // Render second balloon
        out.push_str(r##"<g class="balloon-container anti-crossing balloon-right">"##);
        out.push_str(&render_balloon_svg(
            &d1.text,
            d1.balloon_type,
            origin1,
            mouth1,
            max_balloon_w,
        ));
        out.push_str("</g>");
    } else if num_dialogues == 3 {
        // 3-dialogue layout: 3 side-by-side columns sorted by speaker mouth X (anti-crossing)
        let max_balloon_w = ((pw - 24.0) / 3.0).clamp(80.0, 160.0);

        struct BalloonItem<'a> {
            orig_idx: usize,
            dialogue: &'a DialogueLine,
            mouth: Point,
            bubble_w: f64,
        }

        let mut items: Vec<BalloonItem> = panel
            .dialogues
            .iter()
            .enumerate()
            .map(|(i, d)| {
                let slot = slots
                    .iter()
                    .find(|s| s.speaker.eq_ignore_ascii_case(&d.speaker));
                let mouth = slot.map(|s| s.mouth).unwrap_or(Point {
                    x: px + 40.0 + (i as f64 * (pw - 80.0) / 2.0),
                    y: py + 170.0,
                });
                let (bw, _, _) = measure_balloon(&d.text, max_balloon_w);
                BalloonItem {
                    orig_idx: i,
                    dialogue: d,
                    mouth,
                    bubble_w: bw,
                }
            })
            .collect();

        // Sort items by mouth X so balloon columns match character positions left-to-right (no crossed tails)
        items.sort_by(|a, b| {
            a.mouth
                .x
                .partial_cmp(&b.mouth.x)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let x0 = px + 8.0;
        let x2 = px + pw - items[2].bubble_w - 8.0;
        let ideal_x1 = px + (pw / 2.0) - (items[1].bubble_w / 2.0);
        let x1 = ideal_x1.clamp(
            x0 + items[0].bubble_w + 6.0,
            (x2 - items[1].bubble_w - 6.0).max(x0 + items[0].bubble_w + 6.0),
        );
        let origins_x = [x0, x1, x2];

        for (col_idx, item) in items.iter().enumerate() {
            let origin_x = origins_x[col_idx];

            // Stagger Y slightly based on original dialogue order to indicate temporal precedence
            let origin_y = py + 8.0 + (item.orig_idx as f64 * 6.0);
            let origin = Point {
                x: origin_x,
                y: origin_y,
            };

            out.push_str(&format!(
                r##"<g class="balloon-container anti-crossing balloon-col-{}">"##,
                col_idx
            ));
            out.push_str(&render_balloon_svg(
                &item.dialogue.text,
                item.dialogue.balloon_type,
                origin,
                item.mouth,
                max_balloon_w,
            ));
            out.push_str("</g>");
        }
    } else if num_dialogues >= 4 {
        // 4+ dialogues: 2x2 grid layout (take first 4)
        let max_balloon_w = ((pw - 36.0) / 2.0).clamp(90.0, 190.0);

        struct BalloonItem<'a> {
            orig_idx: usize,
            dialogue: &'a DialogueLine,
            mouth: Point,
            bubble_w: f64,
        }

        let items: Vec<BalloonItem> = panel
            .dialogues
            .iter()
            .take(4)
            .enumerate()
            .map(|(i, d)| {
                let slot = slots
                    .iter()
                    .find(|s| s.speaker.eq_ignore_ascii_case(&d.speaker));
                let mouth = slot.map(|s| s.mouth).unwrap_or(Point {
                    x: px + 50.0 + ((i % 2) as f64 * (pw - 100.0)),
                    y: py + 170.0,
                });
                let (bw, _, _) = measure_balloon(&d.text, max_balloon_w);
                BalloonItem {
                    orig_idx: i,
                    dialogue: d,
                    mouth,
                    bubble_w: bw,
                }
            })
            .collect();

        for (i, item) in items.iter().enumerate() {
            let is_left = item.mouth.x <= px + (pw / 2.0);
            let is_top = i < 2;

            let origin_x = if is_left {
                px + 10.0
            } else {
                px + pw - item.bubble_w - 10.0
            };
            let origin_y = if is_top {
                py + 8.0 + ((item.orig_idx % 2) as f64 * 6.0)
            } else {
                py + 68.0 + ((item.orig_idx % 2) as f64 * 6.0)
            };

            let origin = Point {
                x: origin_x,
                y: origin_y,
            };

            out.push_str(&format!(
                r##"<g class="balloon-container anti-crossing balloon-grid-{}">"##,
                i
            ));
            out.push_str(&render_balloon_svg(
                &item.dialogue.text,
                item.dialogue.balloon_type,
                origin,
                item.mouth,
                max_balloon_w,
            ));
            out.push_str("</g>");
        }
    }

    out.push_str("</g>");
}

fn render_onomatopoeia_starburst(out: &mut String, text: &str, cx: f64, cy: f64) {
    // Generate 16-point starburst polygon
    let num_points = 16;
    let r_outer = 48.0;
    let r_inner = 28.0;
    let mut points = String::new();

    for i in 0..num_points {
        let angle = std::f64::consts::PI * 2.0 * (i as f64) / (num_points as f64);
        let r = if i % 2 == 0 { r_outer } else { r_inner };
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        points.push_str(&format!("{:.1},{:.1} ", x, y));
    }

    out.push_str(&format!(
        r##"<polygon points="{}" fill="#ffe600" stroke="#d90429" stroke-width="3" class="onomatopoeia-starburst" />"##,
        points.trim()
    ));

    out.push_str(&format!(
        r##"<text x="{cx:.1}" y="{:.1}" class="sfx-text" transform="rotate(-8 {cx:.1} {cy:.1})">{}</text>"##,
        cy + 8.0,
        escape_xml(text)
    ));
}

fn render_sticker(out: &mut String, sticker: StickerType, sx: f64, sy: f64) {
    match sticker {
        StickerType::Sweat => {
            out.push_str(&format!(
                r##"<g class="sticker-sweat" transform="translate({sx:.1}, {sy:.1})"><path d="M 0 -12 C 4 -6, 8 0, 8 5 C 8 10, 4 14, 0 14 C -4 14, -8 10, -8 5 C -8 0, -4 -6, 0 -12 Z" fill="#00b4d8" stroke="#03045e" stroke-width="1.5" /></g>"##
            ));
        }
        StickerType::Shock => {
            out.push_str(&format!(
                r##"<g class="sticker-shock" transform="translate({sx:.1}, {sy:.1})"><path d="M -4 -12 L 8 -2 L 1 0 L 8 12 L -6 2 L 0 0 Z" fill="#ffb703" stroke="#d00000" stroke-width="1.5" /></g>"##
            ));
        }
        StickerType::Heart => {
            out.push_str(&format!(
                r##"<g class="sticker-heart" transform="translate({sx:.1}, {sy:.1})"><path d="M 0 -4 C 0 -10, -10 -10, -10 -2 C -10 4, 0 12, 0 14 C 0 12, 10 4, 10 -2 C 10 -10, 0 -10, 0 -4 Z" fill="#ff4d6d" stroke="#590d22" stroke-width="1.5" /></g>"##
            ));
        }
        StickerType::Idea => {
            out.push_str(&format!(
                r##"<g class="sticker-idea" transform="translate({sx:.1}, {sy:.1})"><circle cx="0" cy="-2" r="8" fill="#ffd166" stroke="#222" stroke-width="1.5" /><rect x="-4" y="6" width="8" height="4" rx="1" fill="#bbb" stroke="#222" stroke-width="1" /></g>"##
            ));
        }
        StickerType::Question => {
            out.push_str(&format!(
                r##"<g class="sticker-question" transform="translate({sx:.1}, {sy:.1})"><text x="0" y="8" font-family="'Comic Sans MS', sans-serif" font-size="22" font-weight="bold" fill="#7209b7" stroke="#222" stroke-width="1" text-anchor="middle">?</text></g>"##
            ));
        }
    }
}
