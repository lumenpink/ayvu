use crate::model::BalloonType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Helper to wrap text into lines based on approximate character count per line
pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let words = text.split_whitespace();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= max_chars {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    if lines.is_empty() {
        lines.push(text.to_string());
    }
    lines
}

pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Calculate balloon dimensions and wrapped text for a given max_width
pub fn measure_balloon(text: &str, max_width: f64) -> (f64, f64, Vec<String>) {
    let line_height = 16.0;
    let approx_chars = ((max_width - 20.0) / 7.5).max(7.0) as usize;
    let lines = wrap_text(text, approx_chars);
    let max_line_len = lines.iter().map(|l| l.chars().count()).max().unwrap_or(1);
    let bubble_width = ((max_line_len as f64 * 7.5) + 24.0).clamp(48.0, max_width);
    let bubble_height = (lines.len() as f64 * line_height) + 16.0;
    (bubble_width, bubble_height, lines)
}

/// Render a single speech/thought/shout balloon in SVG vector format
pub fn render_balloon_svg(
    text: &str,
    balloon_type: BalloonType,
    origin: Point,
    target: Point,
    max_width: f64,
) -> String {
    let font_size = 12.0;
    let line_height = 16.0;
    let (bubble_width, bubble_height, lines) = measure_balloon(text, max_width);

    let bx = origin.x;
    let by = origin.y;
    let bw = bubble_width;
    let bh = bubble_height;

    let mut out = String::new();

    match balloon_type {
        BalloonType::Speech => {
            // Dynamically place tail base on the balloon bottom edge closest to target
            let (tail_x1, tail_x2) = if target.x < bx + (bw * 0.35) {
                // Speaker is to the left of balloon center
                (bx + 16.0, (bx + 36.0).min(bx + bw - 12.0))
            } else if target.x > bx + (bw * 0.65) {
                // Speaker is to the right of balloon center
                ((bx + bw - 36.0).max(bx + 12.0), bx + bw - 16.0)
            } else {
                // Speaker is centered under balloon
                let mid = bx + (bw * 0.5);
                (mid - 10.0, mid + 10.0)
            };
            let tail_y = by + bh;

            out.push_str(&format!(
                r##"<path d="M {bx_start} {by} h {bw_inner} a 8 8 0 0 1 8 8 v {bh_inner} a 8 8 0 0 1 -8 8 H {tail_x2} L {target_x} {target_y} L {tail_x1} {tail_y} H {bx_start} a 8 8 0 0 1 -8 -8 V {by_start} a 8 8 0 0 1 8 -8 Z" fill="#ffffff" stroke="#000000" stroke-width="2" stroke-linejoin="round" />"##,
                bx_start = bx + 8.0,
                by = by,
                bw_inner = (bw - 16.0).max(10.0),
                bh_inner = (bh - 16.0).max(10.0),
                tail_x2 = tail_x2,
                target_x = target.x,
                target_y = target.y,
                tail_x1 = tail_x1,
                tail_y = tail_y,
                by_start = by + 8.0
            ));
        }
        BalloonType::Thought => {
            // Cloud rounded rectangle + trailing circles
            out.push_str(&format!(
                r##"<rect x="{bx}" y="{by}" width="{bw}" height="{bh}" rx="14" ry="14" fill="#ffffff" stroke="#000000" stroke-width="2" />"##,
                bx = bx,
                by = by,
                bw = bw,
                bh = bh
            ));
            // 3 trailing thought bubbles pointing toward target
            let c1_x = bx + (bw * 0.45);
            let c1_y = by + bh + 6.0;
            let c2_x = (c1_x * 0.6) + (target.x * 0.4);
            let c2_y = (c1_y * 0.6) + (target.y * 0.4);
            let c3_x = (c1_x * 0.2) + (target.x * 0.8);
            let c3_y = (c1_y * 0.2) + (target.y * 0.8);

            out.push_str(&format!(
                r##"<circle cx="{c1_x:.1}" cy="{c1_y:.1}" r="6" fill="#ffffff" stroke="#000000" stroke-width="2" />"##
            ));
            out.push_str(&format!(
                r##"<circle cx="{c2_x:.1}" cy="{c2_y:.1}" r="4" fill="#ffffff" stroke="#000000" stroke-width="2" />"##
            ));
            out.push_str(&format!(
                r##"<circle cx="{c3_x:.1}" cy="{c3_y:.1}" r="2.5" fill="#ffffff" stroke="#000000" stroke-width="2" />"##
            ));
        }
        BalloonType::Shout => {
            // Starburst polygon around the text
            let p_cx = bx + (bw / 2.0);
            let p_cy = by + (bh / 2.0);
            let rx = (bw / 2.0) + 10.0;
            let ry = (bh / 2.0) + 8.0;
            let points_count = 14;
            let mut pts = Vec::new();

            for i in 0..points_count {
                let angle = (i as f64) * (2.0 * std::f64::consts::PI / (points_count as f64));
                let factor = if i % 2 == 0 { 1.15 } else { 0.85 };
                let px = p_cx + angle.cos() * rx * factor;
                let py = p_cy + angle.sin() * ry * factor;
                pts.push(format!("{:.1},{:.1}", px, py));
            }
            out.push_str(&format!(
                r##"<polygon points="{}" fill="#ffffff" stroke="#000000" stroke-width="2.5" stroke-linejoin="miter" />"##,
                pts.join(" ")
            ));
        }
        BalloonType::Whisper => {
            out.push_str(&format!(
                r##"<rect x="{bx}" y="{by}" width="{bw}" height="{bh}" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="1.8" stroke-dasharray="4,3" />"##
            ));
        }
    }

    // Render text lines inside the bubble
    let text_x = bx + 10.0;
    let text_y = by + 16.0;

    out.push_str(&format!(
        r##"<text x="{text_x:.1}" y="{text_y:.1}" font-family="'Comic Sans MS', 'Comic Relief', 'Comic Neue', cursive, sans-serif" font-size="{font_size}" font-weight="bold" fill="#000000" style="text-transform: uppercase;">"##
    ));

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            out.push_str(&escape_xml(line));
        } else {
            out.push_str(&format!(
                r##"<tspan x="{text_x:.1}" dy="{line_height}">{}</tspan>"##,
                escape_xml(line)
            ));
        }
    }
    out.push_str("</text>");

    out
}
