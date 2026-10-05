// Authentic vector-based comic backdrops in clean hand-inked comic style
// Includes academic settings (UFSC inspired) and classic comic strips

pub fn get_vector_backdrop_svg(name: &str, px: f64, py: f64, w: f64, h: f64) -> Option<String> {
    match name.to_lowercase().as_str() {
        "sala_de_aula" | "classroom" | "aula" => Some(render_classroom_svg(px, py, w, h)),
        "cafe_ccs" | "cafe" | "coffee" => Some(render_cafe_ccs_svg(px, py, w, h)),
        "laboratorio" | "lab" | "laboratory" => Some(render_laboratory_svg(px, py, w, h)),
        "pastoral_vector" | "pastoral_vec" => Some(render_pastoral_vector_svg(px, py, w, h)),
        "room" | "quarto" | "sala" => Some(render_room_svg(px, py, w, h)),
        _ => None,
    }
}

pub fn list_available_backdrops() -> &'static [&'static str] {
    &[
        "pastoral",
        "sala_de_aula",
        "cafe_ccs",
        "laboratorio",
        "room",
    ]
}

/// Sala de Aula (Classroom) - Blackboard with chalk formulas, eraser, window to campus, wooden floor
fn render_classroom_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-classroom">"##);

    // Wall tint
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fcfcfc" />"##
    ));

    // Window on the left side
    let win_x = px + 12.0;
    let win_y = py + 22.0;
    let win_w = 48.0;
    let win_h = 75.0;
    // Window frame
    out.push_str(&format!(
        r##"<rect x="{win_x:.1}" y="{win_y:.1}" width="{win_w:.1}" height="{win_h:.1}" fill="#f0f6fc" stroke="#111111" stroke-width="2" />"##
    ));
    // Window panes
    let win_mid_x = win_x + (win_w / 2.0);
    let win_mid_y = win_y + (win_h / 2.0);
    out.push_str(&format!(
        r##"<line x1="{win_mid_x:.1}" y1="{win_y:.1}" x2="{win_mid_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.5" />"##,
        win_y + win_h
    ));
    out.push_str(&format!(
        r##"<line x1="{win_x:.1}" y1="{win_mid_y:.1}" x2="{:.1}" y2="{win_mid_y:.1}" stroke="#111111" stroke-width="1.5" />"##,
        win_x + win_w
    ));
    // Tree silhouette outside window
    out.push_str(&format!(
        r##"<path d="M {x1:.1} {y1:.1} q 10 -15 20 -2 q 12 -12 18 10" fill="none" stroke="#666666" stroke-width="1.2" />"##,
        x1 = win_x + 5.0,
        y1 = win_y + win_h - 10.0
    ));

    // Large Chalkboard (Quadro Negro)
    let board_x = px + 68.0;
    let board_y = py + 18.0;
    let board_w = w - 82.0;
    let board_h = 88.0;

    // Wood frame
    out.push_str(&format!(
        r##"<rect x="{board_x:.1}" y="{board_y:.1}" width="{board_w:.1}" height="{board_h:.1}" fill="#24292e" stroke="#111111" stroke-width="3" rx="2" />"##
    ));
    // Inner board
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="#1b1f23" stroke="#30363d" stroke-width="1" />"##,
        board_x + 4.0,
        board_y + 4.0,
        board_w - 8.0,
        board_h - 8.0
    ));

    // Chalk tray at bottom
    let tray_y = board_y + board_h;
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{tray_y:.1}" x2="{:.1}" y2="{tray_y:.1}" stroke="#111111" stroke-width="4" stroke-linecap="round" />"##,
        board_x - 2.0,
        board_x + board_w + 2.0
    ));
    // Chalk piece & eraser
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="14" height="6" fill="#e1e4e8" stroke="#111111" stroke-width="1" />"##,
        board_x + 10.0,
        tray_y - 7.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#ffffff" stroke-width="2.5" />"##,
        board_x + 30.0,
        tray_y - 3.0,
        board_x + 38.0,
        tray_y - 3.0
    ));

    // Chalk writings / formulas on board
    let font = r##"font-family="'Comic Sans MS', cursive, sans-serif" font-size="8" fill="#d1d5da" opacity="0.85""##;
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" {font}>UFSC - CALCULO I</text>"##,
        board_x + 12.0,
        board_y + 20.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" {font}>f&apos;(x) = lim &#916;x&#8594;0</text>"##,
        board_x + 12.0,
        board_y + 36.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" {font}>&#8747; e^x dx = e^x + C</text>"##,
        board_x + 12.0,
        board_y + 52.0
    ));
    // Quick parabola sketch on the right side of the board
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q 16 28 32 0" fill="none" stroke="#d1d5da" stroke-width="1.2" stroke-dasharray="2,2" opacity="0.75" />"##,
        board_x + board_w - 46.0,
        board_y + 35.0
    ));

    // Floor baseline & perspective floor planks
    let floor_y = py + h - 16.0;
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.5" stroke-dasharray="10,3" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// Café do CCS - Coffee tables, steam lines, pendant lamp, pergola greenery
fn render_cafe_ccs_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-cafe-ccs">"##);

    // Light warm background
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Pergola ceiling wooden beams
    for i in 0..5 {
        let beam_x = px + 10.0 + (i as f64 * 55.0);
        out.push_str(&format!(
            r##"<line x1="{beam_x:.1}" y1="{py:.1}" x2="{:.1}" y2="{:.1}" stroke="#24292e" stroke-width="2" />"##,
            beam_x + 20.0,
            py + 28.0
        ));
    }
    // Overhead crossbar
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.5" />"##,
        py + 26.0,
        px + w,
        py + 26.0
    ));

    // Pendant lamp hanging from ceiling
    let lamp_x = px + (w / 2.0);
    out.push_str(&format!(
        r##"<line x1="{lamp_x:.1}" y1="{py:.1}" x2="{lamp_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.5" />"##,
        py + 44.0
    ));
    // Lamp shade
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} L {:.1} {:.1} L {:.1} {:.1} Z" fill="#24292e" stroke="#111111" stroke-width="1.5" />"##,
        lamp_x - 12.0,
        py + 54.0,
        lamp_x + 12.0,
        py + 54.0,
        lamp_x,
        py + 44.0
    ));
    // Light bulb dot
    out.push_str(&format!(
        r##"<circle cx="{lamp_x:.1}" cy="{:.1}" r="3" fill="#ffe58f" stroke="#111111" stroke-width="0.8" />"##,
        py + 55.0
    ));

    // Coffee counter / bistro sign
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="70" height="24" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5" />"##,
        px + w - 82.0,
        py + 38.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" font-family="'Comic Sans MS', cursive, sans-serif" font-size="9" font-weight="bold" fill="#111111">CAFÉ CCS</text>"##,
        px + w - 74.0,
        py + 54.0
    ));

    // Hanging ivy / leaves from pergola
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q 5 12 2 20 q -4 -6 -1 -12 M {:.1} {:.1} q 6 10 3 18" fill="none" stroke="#2ea44f" stroke-width="1.5" />"##,
        px + 24.0,
        py + 26.0,
        px + 36.0,
        py + 26.0
    ));

    // Floor line
    let floor_y = py + h - 16.0;
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.5" stroke-dasharray="10,3" />"##,
        px + w
    ));

    // Bistro coffee cup on small side stand
    let stand_x = px + 18.0;
    let stand_y = floor_y - 38.0;
    // Small round table surface
    out.push_str(&format!(
        r##"<ellipse cx="{:.1}" cy="{stand_y:.1}" rx="22" ry="5" fill="#ffffff" stroke="#111111" stroke-width="1.8" />"##,
        stand_x + 15.0
    ));
    // Table leg
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{stand_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2" />"##,
        stand_x + 15.0,
        stand_x + 15.0
    ));
    // Coffee cup & saucer
    let cup_x = stand_x + 12.0;
    let cup_y = stand_y - 2.0;
    out.push_str(&format!(
        r##"<rect x="{cup_x:.1}" y="{:.1}" width="8" height="6" rx="1" fill="#ffffff" stroke="#111111" stroke-width="1" />"##,
        cup_y - 6.0
    ));
    // Steam curl
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q 2 -6 -2 -10" fill="none" stroke="#888888" stroke-width="1" />"##,
        cup_x + 4.0,
        cup_y - 8.0
    ));

    out.push_str("</g>");
    out
}

/// Laboratório (Laboratory) - Lab bench, glassware, microscope, reagent shelf
fn render_laboratory_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-laboratory">"##);

    // Wall tint with tile grid lines
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#f6f8fa" />"##
    ));

    // Tiled wall pattern
    let tile_size = 28.0;
    let max_y = py + h - 60.0;
    let mut ty = py + 12.0;
    while ty < max_y {
        out.push_str(&format!(
            r##"<line x1="{px:.1}" y1="{ty:.1}" x2="{:.1}" y2="{ty:.1}" stroke="#e1e4e8" stroke-width="1" />"##,
            px + w
        ));
        ty += tile_size;
    }
    let mut tx = px + 14.0;
    while tx < px + w {
        out.push_str(&format!(
            r##"<line x1="{tx:.1}" y1="{py:.1}" x2="{tx:.1}" y2="{max_y:.1}" stroke="#e1e4e8" stroke-width="1" />"##
        ));
        tx += tile_size;
    }

    // Reagent shelf near top
    let shelf_y = py + 34.0;
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{shelf_y:.1}" width="{:.1}" height="5" fill="#24292e" stroke="#111111" stroke-width="1.5" />"##,
        px + 14.0,
        w - 28.0
    ));
    // Chemical bottles on shelf
    for i in 0..5 {
        let bot_x = px + 24.0 + (i as f64 * 36.0);
        let bot_h = if i % 2 == 0 { 16.0 } else { 12.0 };
        out.push_str(&format!(
            r##"<rect x="{bot_x:.1}" y="{:.1}" width="10" height="{bot_h:.1}" rx="1" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
            shelf_y - bot_h
        ));
        // Bottle cap
        out.push_str(&format!(
            r##"<rect x="{:.1}" y="{:.1}" width="6" height="3" fill="#24292e" />"##,
            bot_x + 2.0,
            shelf_y - bot_h - 3.0
        ));
    }

    // Lab bench countertop
    let bench_y = py + h - 55.0;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{bench_y:.1}" width="{w:.1}" height="8" fill="#d1d5da" stroke="#111111" stroke-width="1.8" />"##
    ));

    // Microscope on the right side of bench
    let mic_x = px + w - 46.0;
    let mic_base_y = bench_y;
    // Base
    out.push_str(&format!(
        r##"<rect x="{mic_x:.1}" y="{:.1}" width="24" height="4" rx="1" fill="#24292e" stroke="#111111" stroke-width="1" />"##,
        mic_base_y - 4.0
    ));
    // Arm
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q -8 -16 2 -26 l 8 0" fill="none" stroke="#24292e" stroke-width="3" />"##,
        mic_x + 18.0,
        mic_base_y - 4.0
    ));
    // Eyepiece tube
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.5" />"##,
        mic_x + 16.0,
        mic_base_y - 34.0,
        mic_x + 24.0,
        mic_base_y - 42.0
    ));

    // Erlenmeyer flask on the left side of bench
    let flask_x = px + 22.0;
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 5 -16 l 0 -6 l 4 0 l 0 6 l 5 16 Z" fill="#ffffff" stroke="#111111" stroke-width="1.5" stroke-linejoin="round" />"##,
        flask_x,
        bench_y,
    ));
    // Liquid inside flask
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 2.5 -8 l 9 0 l 2.5 8 Z" fill="#b3e1f7" stroke="none" />"##,
        flask_x + 2.0,
        bench_y - 1.0,
    ));

    // Floor line
    let floor_y = py + h - 16.0;
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.5" stroke-dasharray="10,3" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// Sala / Quarto (Room) - Window, wall picture, classic rug
fn render_room_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-room">"##);

    // Warm wall
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Picture frame on the wall
    let pic_x = px + (w / 2.0) - 26.0;
    let pic_y = py + 26.0;
    out.push_str(&format!(
        r##"<rect x="{pic_x:.1}" y="{pic_y:.1}" width="52" height="38" fill="#ffffff" stroke="#111111" stroke-width="2" />"##
    ));
    // Mini landscape inside picture
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="6" fill="#ffe58f" stroke="#111111" stroke-width="0.8" />"##,
        pic_x + 14.0,
        pic_y + 14.0
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 14 -12 l 14 12 l 10 -8 l 10 8" fill="none" stroke="#111111" stroke-width="1.2" />"##,
        pic_x + 4.0,
        pic_y + 32.0
    ));

    // Floor line
    let floor_y = py + h - 16.0;
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.5" stroke-dasharray="10,3" />"##,
        px + w
    ));

    // Patterned rug on floor
    out.push_str(&format!(
        r##"<ellipse cx="{:.1}" cy="{:.1}" rx="80" ry="10" fill="#f0f6fc" stroke="#111111" stroke-width="1.5" stroke-dasharray="6,3" />"##,
        px + (w / 2.0),
        floor_y + 4.0
    ));

    out.push_str("</g>");
    out
}

/// Pastoral Vector (Rolling hills, fence, tree) - Clean vector recreation
fn render_pastoral_vector_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-pastoral">"##);

    // Sky
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fcfcfc" />"##
    ));

    // Sun / Cloud
    let sun_x = px + w - 40.0;
    let sun_y = py + 30.0;
    out.push_str(&format!(
        r##"<circle cx="{sun_x:.1}" cy="{sun_y:.1}" r="14" fill="#ffffff" stroke="#111111" stroke-width="1.5" />"##
    ));

    // Distant rolling hills
    out.push_str(&format!(
        r##"<path d="M {px:.1} {:.1} q 70 -35 140 0 q 70 -30 140 5 L {:.1} {:.1} L {px:.1} {:.1} Z" fill="#f6f8fa" stroke="#111111" stroke-width="1.5" />"##,
        py + 130.0,
        px + w,
        py + h,
        py + h
    ));

    // Post and rail fence in midground
    let fence_y = py + 155.0;
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{fence_y:.1}" x2="{:.1}" y2="{fence_y:.1}" stroke="#111111" stroke-width="1.5" />"##,
        px + w
    ));
    for i in 0..6 {
        let post_x = px + 15.0 + (i as f64 * 46.0);
        out.push_str(&format!(
            r##"<line x1="{post_x:.1}" y1="{:.1}" x2="{post_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="2" />"##,
            fence_y - 8.0,
            fence_y + 16.0
        ));
    }

    // Floor line
    let floor_y = py + h - 16.0;
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.5" stroke-dasharray="10,3" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}
