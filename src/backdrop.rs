// Authentic vector-based comic backdrops in clean hand-inked comic style
// Includes academic settings (UFSC inspired) and classic comic strips

pub fn get_vector_backdrop_svg(name: &str, px: f64, py: f64, w: f64, h: f64) -> Option<String> {
    match name.to_lowercase().as_str() {
        "sala_de_aula" | "classroom" | "aula" => Some(render_classroom_svg(px, py, w, h)),
        "cafe_ccs" | "cafe" | "coffee" => Some(render_cafe_ccs_svg(px, py, w, h)),
        "laboratorio" | "lab" | "laboratory" => Some(render_laboratory_svg(px, py, w, h)),
        "hu_fachada" | "hu" | "hospital" | "hospital_universitario" => {
            Some(render_hu_fachada_svg(px, py, w, h))
        }
        "ru" | "bandejao" | "restaurante" | "restaurante_universitario" => {
            Some(render_ru_svg(px, py, w, h))
        }
        "hall_ccs" | "ccs" | "hall" | "centro_ciencias_saude" => {
            Some(render_hall_ccs_svg(px, py, w, h))
        }
        "hu_enfermaria" | "enfermaria" | "leito" | "consultorio" | "clinica" => {
            Some(render_hu_enfermaria_svg(px, py, w, h))
        }
        "bu_ufsc" | "bu" | "biblioteca" | "biblioteca_universitaria" => {
            Some(render_bu_ufsc_svg(px, py, w, h))
        }
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
        "hu_fachada",
        "ru",
        "hall_ccs",
        "hu_enfermaria",
        "bu_ufsc",
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

/// Hospital Universitário (HU/UFSC) - Iconic brutalist entrance canopy, signage, building facade & campus palms
fn render_hu_fachada_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-hu-fachada">"##);

    // Sky background
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#f4f7fb" />"##
    ));

    // Distant hospital main block (Bloco do HU)
    let bld_x = px + 15.0;
    let bld_y = py + 20.0;
    let bld_w = w - 30.0;
    let bld_h = 100.0;
    out.push_str(&format!(
        r##"<rect x="{bld_x:.1}" y="{bld_y:.1}" width="{bld_w:.1}" height="{bld_h:.1}" fill="#e6edf5" stroke="#111111" stroke-width="2" />"##
    ));

    // Roof equipment / technical block
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="40" height="12" fill="#d0d7de" stroke="#111111" stroke-width="1.8" />"##,
        bld_x + 20.0,
        bld_y - 12.0
    ));

    // Ribbon windows (Janelas em fita dos andares do HU)
    for floor_idx in 0..2 {
        let win_y = bld_y + 16.0 + (floor_idx as f64 * 26.0);
        out.push_str(&format!(
            r##"<rect x="{:.1}" y="{win_y:.1}" width="{:.1}" height="14" fill="#ffffff" stroke="#111111" stroke-width="1.5" />"##,
            bld_x + 8.0,
            bld_w - 16.0
        ));
        // Vertical mullions / dividers
        let num_panes = 8;
        let pane_step = (bld_w - 16.0) / (num_panes as f64);
        for p in 1..num_panes {
            let div_x = bld_x + 8.0 + (p as f64 * pane_step);
            out.push_str(&format!(
                r##"<line x1="{div_x:.1}" y1="{win_y:.1}" x2="{div_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.2" />"##,
                win_y + 14.0
            ));
        }
    }

    // Concrete canopy / entrance portico (Marquise do HU) projecting forward
    let portico_x = px + 35.0;
    let portico_y = py + 84.0;
    let portico_w = w - 70.0;
    let portico_h = 20.0;

    // Canopy slab
    out.push_str(&format!(
        r##"<rect x="{portico_x:.1}" y="{portico_y:.1}" width="{portico_w:.1}" height="{portico_h:.1}" fill="#ffffff" stroke="#111111" stroke-width="2.5" rx="1" />"##
    ));

    // Official lettering on the canopy beam
    let font_title = r##"font-family="'Helvetica Neue', Arial, sans-serif" font-size="6.5" font-weight="bold" fill="#0969da" letter-spacing="0.5""##;
    let font_sub = r##"font-family="'Helvetica Neue', Arial, sans-serif" font-size="5" font-weight="bold" fill="#24292e" letter-spacing="0.8""##;
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" {font_title}>HOSPITAL UNIVERSITÁRIO</text>"##,
        portico_x + (portico_w / 2.0),
        portico_y + 9.5
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" {font_sub}>PROF. POLYDORO ERNANI DE SÃO THIAGO • UFSC</text>"##,
        portico_x + (portico_w / 2.0),
        portico_y + 16.5
    ));

    // Emergency red cross emblem on the left of canopy
    let cross_x = portico_x + 8.0;
    let cross_y = portico_y + 5.5;
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="9" height="3" fill="#cf222e" rx="0.5" />"##,
        cross_x - 3.0,
        cross_y + 3.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="3" height="9" fill="#cf222e" rx="0.5" />"##,
        cross_x, cross_y
    ));

    // Robust structural support columns (Pilares de concreto)
    let col1_x = portico_x + 16.0;
    let col2_x = portico_x + portico_w - 22.0;
    let floor_y = py + h - 16.0;

    for col_x in [col1_x, col2_x] {
        out.push_str(&format!(
            r##"<rect x="{col_x:.1}" y="{:.1}" width="6" height="{:.1}" fill="#eaeef2" stroke="#111111" stroke-width="2" />"##,
            portico_y + portico_h,
            floor_y - (portico_y + portico_h)
        ));
    }

    // Glass entrance doors under canopy
    let door_x = portico_x + 26.0;
    let door_w = portico_w - 52.0;
    let door_y = portico_y + portico_h;
    let door_h = floor_y - door_y;
    out.push_str(&format!(
        r##"<rect x="{door_x:.1}" y="{door_y:.1}" width="{door_w:.1}" height="{door_h:.1}" fill="#f6f8fa" stroke="#111111" stroke-width="1.8" />"##
    ));
    // Automatic sliding door center line & handles
    let door_mid_x = door_x + (door_w / 2.0);
    out.push_str(&format!(
        r##"<line x1="{door_mid_x:.1}" y1="{door_y:.1}" x2="{door_mid_x:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.5" />"##
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="1.5" height="12" fill="#111111" />"##,
        door_mid_x - 4.0,
        door_y + 14.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="1.5" height="12" fill="#111111" />"##,
        door_mid_x + 2.5,
        door_y + 14.0
    ));

    // Tropical Campus Palm Tree on right edge (UFSC landscaping)
    let palm_x = px + w - 16.0;
    let palm_base_y = floor_y;
    // Curved trunk
    out.push_str(&format!(
        r##"<path d="M {palm_x:.1} {palm_base_y:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#24292e" stroke-width="3" stroke-linecap="round" />"##,
        palm_x - 6.0,
        palm_base_y - 60.0,
        palm_x - 4.0,
        palm_base_y - 120.0
    ));
    // Palm fronds
    let crown_x = palm_x - 4.0;
    let crown_y = palm_base_y - 120.0;
    out.push_str(&format!(
        r##"<path d="M {crown_x:.1} {crown_y:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#2da44e" stroke-width="2" stroke-linecap="round" />"##,
        crown_x - 25.0, crown_y - 15.0, crown_x - 35.0, crown_y + 12.0
    ));
    out.push_str(&format!(
        r##"<path d="M {crown_x:.1} {crown_y:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#2da44e" stroke-width="2" stroke-linecap="round" />"##,
        crown_x - 18.0, crown_y - 25.0, crown_x - 28.0, crown_y - 10.0
    ));
    out.push_str(&format!(
        r##"<path d="M {crown_x:.1} {crown_y:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#2da44e" stroke-width="2" stroke-linecap="round" />"##,
        crown_x + 15.0, crown_y - 20.0, crown_x + 20.0, crown_y - 5.0
    ));

    // Ground baseline & access ramp lines
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2" />"##,
        px + w
    ));
    // Curb / sidewalk line
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#8c959f" stroke-width="1.2" stroke-dasharray="8,4" />"##,
        floor_y + 6.0,
        px + w,
        floor_y + 6.0
    ));

    out.push_str("</g>");
    out
}

/// RU (Restaurante Universitário UFSC) - Dining hall, large glass windows, stainless steel tables, round stools & authentic bandejão
fn render_ru_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-ru">"##);

    // Warm high-ceiling dining hall background
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Ceiling industrial beams & pendant strip lighting
    for i in 0..4 {
        let beam_x = px + 15.0 + (i as f64 * 65.0);
        out.push_str(&format!(
            r##"<line x1="{beam_x:.1}" y1="{py:.1}" x2="{beam_x:.1}" y2="{:.1}" stroke="#d0d7de" stroke-width="1.5" />"##,
            py + 22.0
        ));
    }
    // Hanging tube lamps
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="4" fill="#ffffff" stroke="#111111" stroke-width="1.2" rx="1" />"##,
        px + 20.0,
        py + 20.0,
        w - 40.0
    ));

    // Panoramic dining hall glass windows overlooking campus
    let win_x = px + 12.0;
    let win_y = py + 30.0;
    let win_w = w - 24.0;
    let win_h = 58.0;
    out.push_str(&format!(
        r##"<rect x="{win_x:.1}" y="{win_y:.1}" width="{win_w:.1}" height="{win_h:.1}" fill="#f0f6fc" stroke="#111111" stroke-width="2" />"##
    ));
    // Window mullions (vertical)
    let num_cols = 5;
    let col_w = win_w / (num_cols as f64);
    for c in 1..num_cols {
        let line_x = win_x + (c as f64 * col_w);
        out.push_str(&format!(
            r##"<line x1="{line_x:.1}" y1="{win_y:.1}" x2="{line_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.5" />"##,
            win_y + win_h
        ));
    }
    // Trees silhouette outside window
    out.push_str(&format!(
        r##"<path d="M {win_x:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#2da44e" stroke-width="1.5" opacity="0.65" />"##,
        win_y + win_h - 4.0,
        win_x + 30.0, win_y + win_h - 28.0, win_x + 60.0, win_y + win_h - 10.0,
        win_x + 110.0, win_y + win_h - 35.0, win_x + 150.0, win_y + win_h - 8.0,
        win_x + 185.0, win_y + win_h - 25.0, win_x + win_w, win_y + win_h - 12.0
    ));

    // Official RU sign on upper right wall
    let sign_x = px + w - 75.0;
    let sign_y = py + 36.0;
    out.push_str(&format!(
        r##"<rect x="{sign_x:.1}" y="{sign_y:.1}" width="65" height="24" fill="#ffffff" stroke="#111111" stroke-width="1.5" rx="2" />"##
    ));
    let font_ru = r##"font-family="'Helvetica Neue', Arial, sans-serif" font-size="6.5" font-weight="bold" fill="#0969da""##;
    let font_ru_sub =
        r##"font-family="'Helvetica Neue', Arial, sans-serif" font-size="4.8" fill="#57606a""##;
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" {font_ru}>RU TRINDADE</text>"##,
        sign_x + 32.5,
        sign_y + 10.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" {font_ru_sub}>DEVOLVA SUA BANDEJA</text>"##,
        sign_x + 32.5,
        sign_y + 18.5
    ));

    // Floor baseline
    let floor_y = py + h - 16.0;
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        px + w
    ));

    // Perspective stainless steel table with round fixed stools (Mesas de Inox do RU)
    let tbl_x = px + 18.0;
    let tbl_y = py + 112.0;
    let tbl_w = w - 36.0;
    let tbl_h = 24.0;

    // Table top (metallic inox finish)
    out.push_str(&format!(
        r##"<path d="M {tbl_x:.1} {tbl_y:.1} L {:.1} {tbl_y:.1} L {:.1} {:.1} L {:.1} {:.1} Z" fill="#eaeef2" stroke="#111111" stroke-width="2" />"##,
        tbl_x + tbl_w,
        tbl_x + tbl_w + 6.0,
        tbl_y + tbl_h,
        tbl_x - 6.0,
        tbl_y + tbl_h
    ));
    // Table edge bevel highlight
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#ffffff" stroke-width="1.5" />"##,
        tbl_x - 5.0,
        tbl_y + 2.0,
        tbl_x + tbl_w + 5.0,
        tbl_y + 2.0
    ));

    // Table vertical tubular legs down to floor
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="5" height="{:.1}" fill="#8c959f" stroke="#111111" stroke-width="1.5" />"##,
        tbl_x + 30.0,
        tbl_y + tbl_h,
        floor_y - (tbl_y + tbl_h)
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="5" height="{:.1}" fill="#8c959f" stroke="#111111" stroke-width="1.5" />"##,
        tbl_x + tbl_w - 35.0,
        tbl_y + tbl_h,
        floor_y - (tbl_y + tbl_h)
    ));

    // Fixed round stools (Bancos redondos do RU)
    for i in 0..3 {
        let stool_cx = tbl_x + 22.0 + (i as f64 * 72.0);
        let stool_cy = tbl_y + tbl_h + 12.0;
        // Stool support tube
        out.push_str(&format!(
            r##"<line x1="{stool_cx:.1}" y1="{stool_cy:.1}" x2="{stool_cx:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="3" />"##
        ));
        // Stool seat
        out.push_str(&format!(
            r##"<ellipse cx="{stool_cx:.1}" cy="{stool_cy:.1}" rx="14" ry="4.5" fill="#f6f8fa" stroke="#111111" stroke-width="1.8" />"##
        ));
    }

    // THE ICONIC BANDEJÃO DE INOX (UFSC Stainless Steel Cafeteria Tray) on the table!
    let tray_x = tbl_x + 16.0;
    let tray_y = tbl_y + 4.0;
    let tray_w = 64.0;
    let tray_h = 16.0;

    // Outer tray
    out.push_str(&format!(
        r##"<rect x="{tray_x:.1}" y="{tray_y:.1}" width="{tray_w:.1}" height="{tray_h:.1}" fill="#ffffff" stroke="#111111" stroke-width="1.8" rx="2" />"##
    ));
    // Tray compartments (Arroz/Feijão, Guarnição, Salada)
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="22" height="10" fill="#eaeef2" stroke="#111111" stroke-width="1" rx="1.5" />"##,
        tray_x + 4.0,
        tray_y + 3.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="16" height="10" fill="#eaeef2" stroke="#111111" stroke-width="1" rx="1.5" />"##,
        tray_x + 28.0,
        tray_y + 3.0
    ));
    // Round indent for water/juice cup
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="4" fill="#eaeef2" stroke="#111111" stroke-width="1" />"##,
        tray_x + 52.0,
        tray_y + 8.0
    ));
    // Water/Juice plastic cup
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="#ffffff" stroke="#111111" stroke-width="1" />"##,
        tray_x + 49.0, tray_y + 8.0,
        tray_x + 55.0, tray_y + 8.0,
        tray_x + 54.0, tray_y + 1.0,
        tray_x + 50.0, tray_y + 1.0
    ));
    // Fork and spoon beside tray
    let fork_x = tray_x + tray_w + 5.0;
    let fork_y = tray_y + 2.0;
    out.push_str(&format!(
        r##"<line x1="{fork_x:.1}" y1="{fork_y:.1}" x2="{fork_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.2" />"##,
        fork_y + 12.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{fork_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.2" />"##,
        fork_x + 4.0,
        fork_x + 4.0,
        fork_y + 12.0
    ));

    out.push_str("</g>");
    out
}

/// Hall do CCS (Centro de Ciências da Saúde) - Brutalist concrete pillars, open staircase, student notice board & potted flora
fn render_hall_ccs_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-hall-ccs">"##);

    // Architectural background
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#f6f8fa" />"##
    ));

    // Floor line
    let floor_y = py + h - 16.0;

    // Heavy Brutalist Concrete Columns (Pilares de Concreto da UFSC)
    let pilar1_x = px + 12.0;
    let pilar_w = 26.0;
    out.push_str(&format!(
        r##"<rect x="{pilar1_x:.1}" y="{py:.1}" width="{pilar_w:.1}" height="{:.1}" fill="#eaeef2" stroke="#111111" stroke-width="2" />"##,
        floor_y - py
    ));
    // Concrete formwork texture lines (marcas de ripas de concreto aparente)
    for l in 1..7 {
        let line_y = py + (l as f64 * 25.0);
        if line_y < floor_y {
            out.push_str(&format!(
                r##"<line x1="{pilar1_x:.1}" y1="{line_y:.1}" x2="{:.1}" y2="{line_y:.1}" stroke="#d0d7de" stroke-width="1.2" />"##,
                pilar1_x + pilar_w
            ));
        }
    }

    // Overhead structural concrete crossbeam
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="22" fill="#d0d7de" stroke="#111111" stroke-width="2" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#afb8c1" stroke-width="1" />"##,
        py + 18.0,
        px + w,
        py + 18.0
    ));

    // Open staircase ascending to 2nd floor (Escadaria do Hall do CCS)
    let stair_x = px + 48.0;
    let stair_top_y = py + 34.0;
    let num_steps = 7;
    let step_dx = 14.0;
    let step_dy = 14.0;

    let mut step_path = format!(
        "M {stair_x:.1} {:.1}",
        stair_top_y + (num_steps as f64 * step_dy)
    );
    for s in 0..num_steps {
        let curr_y = stair_top_y + ((num_steps - s) as f64 * step_dy);
        let curr_x = stair_x + (s as f64 * step_dx);
        step_path.push_str(&format!(
            " L {:.1} {:.1} L {:.1} {:.1}",
            curr_x,
            curr_y - step_dy,
            curr_x + step_dx,
            curr_y - step_dy
        ));
    }
    out.push_str(&format!(
        r##"<path d="{step_path}" fill="none" stroke="#111111" stroke-width="2" />"##
    ));
    // Industrial black metal handrail (Corrimão tubular preto)
    let rail_start_x = stair_x - 4.0;
    let rail_start_y = stair_top_y + (num_steps as f64 * step_dy) - 18.0;
    let rail_end_x = stair_x + (num_steps as f64 * step_dx) + 4.0;
    let rail_end_y = stair_top_y - 18.0;
    out.push_str(&format!(
        r##"<line x1="{rail_start_x:.1}" y1="{rail_start_y:.1}" x2="{rail_end_x:.1}" y2="{rail_end_y:.1}" stroke="#111111" stroke-width="2.5" />"##
    ));
    // Railing upright posts
    for p in 0..4 {
        let px_pos = rail_start_x + (p as f64 * 32.0);
        let py_pos = rail_start_y - (p as f64 * 28.0);
        out.push_str(&format!(
            r##"<line x1="{px_pos:.1}" y1="{py_pos:.1}" x2="{px_pos:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.5" />"##,
            py_pos + 18.0
        ));
    }

    // Student Bulletin Board (Mural de Avisos do CCS) on the right wall
    let board_x = px + w - 85.0;
    let board_y = py + 38.0;
    let board_w = 75.0;
    let board_h = 72.0;

    // Cork/Aluminum frame
    out.push_str(&format!(
        r##"<rect x="{board_x:.1}" y="{board_y:.1}" width="{board_w:.1}" height="{board_h:.1}" fill="#fff8e7" stroke="#111111" stroke-width="2" rx="1.5" />"##
    ));
    // Header tag "MURAL CCS"
    out.push_str(&format!(
        r##"<rect x="{board_x:.1}" y="{board_y:.1}" width="{board_w:.1}" height="10" fill="#1f2328" />"##
    ));
    let font_board = r##"font-family="'Helvetica Neue', Arial, sans-serif" font-size="5" font-weight="bold" fill="#ffffff""##;
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" {font_board}>MURAL • CCS / UFSC</text>"##,
        board_x + (board_w / 2.0),
        board_y + 7.5
    ));

    // Pinned Academic Flyers / Posters
    // Flyer 1: Semana Acadêmica
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="30" height="24" fill="#ffffff" stroke="#111111" stroke-width="1" rx="0.5" />"##,
        board_x + 5.0,
        board_y + 14.0
    ));
    let font_flyer = r##"font-family="'Comic Sans MS', sans-serif" font-size="3.8" font-weight="bold" fill="#0969da""##;
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" {font_flyer}>SEMANA</text>"##,
        board_x + 7.0,
        board_y + 20.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" {font_flyer}>ACADÊMICA</text>"##,
        board_x + 7.0,
        board_y + 25.0
    ));
    // Mini thumb pin (percevejo)
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="1.5" fill="#cf222e" stroke="#111111" stroke-width="0.8" />"##,
        board_x + 20.0,
        board_y + 14.0
    ));

    // Flyer 2: Bolsa de IC / Monitoria
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="32" height="24" fill="#ddf4ff" stroke="#111111" stroke-width="1" rx="0.5" />"##,
        board_x + 38.0,
        board_y + 14.0
    ));
    let font_ic = r##"font-family="'Comic Sans MS', sans-serif" font-size="3.8" font-weight="bold" fill="#1a7f37""##;
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" {font_ic}>VAGA IC</text>"##,
        board_x + 41.0,
        board_y + 21.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" font-size="3" fill="#24292e">Farmácia / Med</text>"##,
        board_x + 41.0,
        board_y + 27.0
    ));
    // Mini thumb pin
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="1.5" fill="#0969da" stroke="#111111" stroke-width="0.8" />"##,
        board_x + 54.0,
        board_y + 14.0
    ));

    // Flyer 3: Anúncio com picotes para rasgar na base
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="42" height="22" fill="#ffffff" stroke="#111111" stroke-width="1" rx="0.5" />"##,
        board_x + 8.0,
        board_y + 42.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" font-size="3.8" font-weight="bold" fill="#24292e">GRUPO DE ESTUDOS</text>"##,
        board_x + 11.0,
        board_y + 49.0
    ));
    // Slits / tear-off tabs (tiras picotadas de telefone)
    for t in 0..5 {
        let tab_x = board_x + 11.0 + (t as f64 * 7.0);
        out.push_str(&format!(
            r##"<line x1="{tab_x:.1}" y1="{:.1}" x2="{tab_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" />"##,
            board_y + 57.0,
            board_y + 64.0
        ));
    }

    // Concrete waiting bench at bottom right
    let bench_x = board_x;
    let bench_y = floor_y - 12.0;
    out.push_str(&format!(
        r##"<rect x="{bench_x:.1}" y="{bench_y:.1}" width="70" height="12" fill="#e6edf5" stroke="#111111" stroke-width="1.8" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{bench_x:.1}" y1="{bench_y:.1}" x2="{:.1}" y2="{bench_y:.1}" stroke="#111111" stroke-width="2.2" />"##,
        bench_x + 70.0
    ));

    // Potted Monstera / Tropical Plant in the Hall (Costela-de-Adão)
    let pot_x = pilar1_x + pilar_w + 6.0;
    let pot_y = floor_y - 16.0;
    // Pot
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="#afb8c1" stroke="#111111" stroke-width="1.8" />"##,
        pot_x, pot_y + 16.0,
        pot_x + 16.0, pot_y + 16.0,
        pot_x + 19.0, pot_y,
        pot_x - 3.0, pot_y
    ));
    // Monstera leaves
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="#e6ffed" stroke="#1a7f37" stroke-width="1.8" />"##,
        pot_x + 8.0, pot_y,
        pot_x - 12.0, pot_y - 20.0,
        pot_x - 18.0, pot_y - 32.0,
        pot_x + 2.0, pot_y - 24.0,
        pot_x + 8.0, pot_y
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="#e6ffed" stroke="#1a7f37" stroke-width="1.8" />"##,
        pot_x + 8.0, pot_y,
        pot_x + 24.0, pot_y - 22.0,
        pot_x + 30.0, pot_y - 34.0,
        pot_x + 14.0, pot_y - 22.0,
        pot_x + 8.0, pot_y
    ));

    // Floor line and perspective floor joint
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2" />"##,
        px + w
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#afb8c1" stroke-width="1" stroke-dasharray="8,4" />"##,
        px + 50.0,
        px + 10.0,
        floor_y + 14.0
    ));

    out.push_str("</g>");
    out
}

/// Enfermaria / Leito do HU (Hospital Ward / Clinic) - Hospital bed, IV drip stand, medical gas panel & swinging door
fn render_hu_enfermaria_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-hu-enfermaria">"##);

    // Sterile hospital wall background
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fbfcfd" />"##
    ));

    // Wall bumper guard rail (Faixa bate-maca vinílica)
    let rail_y = py + 92.0;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{rail_y:.1}" width="{w:.1}" height="8" fill="#e1e4e8" stroke="#111111" stroke-width="1.2" />"##
    ));

    // Hospital swinging double doors on the left (Porta vai-e-vem)
    let door_x = px + 12.0;
    let door_y = py + 26.0;
    let door_w = 44.0;
    let floor_y = py + h - 16.0;
    let door_h = floor_y - door_y;

    out.push_str(&format!(
        r##"<rect x="{door_x:.1}" y="{door_y:.1}" width="{door_w:.1}" height="{door_h:.1}" fill="#f6f8fa" stroke="#111111" stroke-width="2" />"##
    ));
    // Door split
    let door_mid_x = door_x + (door_w / 2.0);
    out.push_str(&format!(
        r##"<line x1="{door_mid_x:.1}" y1="{door_y:.1}" x2="{door_mid_x:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.5" />"##
    ));
    // Circular glass portholes (Visores redondos da porta hospitalar)
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="6.5" fill="#e6edf5" stroke="#111111" stroke-width="1.5" />"##,
        door_x + 11.0,
        door_y + 24.0
    ));
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="6.5" fill="#e6edf5" stroke="#111111" stroke-width="1.5" />"##,
        door_x + 33.0,
        door_y + 24.0
    ));
    // Stainless steel push plates (Placas de empurrar)
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="4" height="20" fill="#d0d7de" stroke="#111111" stroke-width="1" />"##,
        door_mid_x - 6.0,
        door_y + 44.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="4" height="20" fill="#d0d7de" stroke="#111111" stroke-width="1" />"##,
        door_mid_x + 2.0,
        door_y + 44.0
    ));

    // Medical Gas Wall Console (Painel de Gases Medicinais: O2, Vácuo, Ar)
    let panel_x = px + 68.0;
    let panel_y = py + 34.0;
    out.push_str(&format!(
        r##"<rect x="{panel_x:.1}" y="{panel_y:.1}" width="46" height="20" fill="#ffffff" stroke="#111111" stroke-width="1.8" rx="2" />"##
    ));
    // Green O2 outlet (Oxigênio)
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="3.5" fill="#2da44e" stroke="#111111" stroke-width="1" />"##,
        panel_x + 10.0,
        panel_y + 10.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" font-size="3" font-weight="bold" fill="#111111">O&#8322;</text>"##,
        panel_x + 10.0,
        panel_y + 18.0
    ));
    // Yellow Air outlet (Ar comprimido)
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="3.5" fill="#d4a72c" stroke="#111111" stroke-width="1" />"##,
        panel_x + 23.0,
        panel_y + 10.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" font-size="3" font-weight="bold" fill="#111111">AR</text>"##,
        panel_x + 23.0,
        panel_y + 18.0
    ));
    // Grey Vacuum outlet (Vácuo)
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="3.5" fill="#8c959f" stroke="#111111" stroke-width="1" />"##,
        panel_x + 36.0,
        panel_y + 10.0
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" font-size="3" font-weight="bold" fill="#111111">VAC</text>"##,
        panel_x + 36.0,
        panel_y + 18.0
    ));

    // Patient Medical Chart Clipboard (Prancheta de Prontuário)
    let clip_x = panel_x + 56.0;
    let clip_y = py + 32.0;
    out.push_str(&format!(
        r##"<rect x="{clip_x:.1}" y="{clip_y:.1}" width="18" height="24" fill="#ffffff" stroke="#111111" stroke-width="1.5" rx="1" />"##
    ));
    // Metal clip on top
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="8" height="3.5" fill="#57606a" stroke="#111111" stroke-width="1" rx="0.5" />"##,
        clip_x + 5.0,
        clip_y - 2.0
    ));
    // Chart lines (ECG heartbeat rhythm line)
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 2 0 l 2 -4 l 2 8 l 2 -4 l 4 0" fill="none" stroke="#cf222e" stroke-width="1" />"##,
        clip_x + 3.0,
        clip_y + 12.0
    ));

    // Hospital Bed / Stretcher (Maca Hospitalar com Rodinhas) on the right
    let bed_x = px + w - 88.0;
    let bed_y = py + 118.0;
    let bed_w = 78.0;
    let _bed_h = 24.0;

    // Bed mattress & inclined pillow/backrest
    out.push_str(&format!(
        r##"<path d="M {bed_x:.1} {bed_y:.1} L {:.1} {:.1} L {:.1} {:.1} L {:.1} {:.1} L {bed_x:.1} {:.1} Z" fill="#ffffff" stroke="#111111" stroke-width="2" rx="2" />"##,
        bed_x + 18.0, bed_y - 10.0,
        bed_x + bed_w, bed_y - 10.0,
        bed_x + bed_w, bed_y + 14.0,
        bed_y + 14.0
    ));
    // Mattress seam & pillow
    out.push_str(&format!(
        r##"<ellipse cx="{:.1}" cy="{:.1}" rx="8" ry="4" fill="#f0f6fc" stroke="#111111" stroke-width="1.2" />"##,
        bed_x + bed_w - 14.0,
        bed_y - 4.0
    ));
    // Tubular metal frame under mattress
    out.push_str(&format!(
        r##"<line x1="{bed_x:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.5" />"##,
        bed_y + 14.0,
        bed_x + bed_w,
        bed_y + 14.0
    ));
    // Legs & casters / wheels
    let leg1_x = bed_x + 12.0;
    let leg2_x = bed_x + bed_w - 14.0;
    for lx in [leg1_x, leg2_x] {
        out.push_str(&format!(
            r##"<line x1="{lx:.1}" y1="{:.1}" x2="{lx:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2" />"##,
            bed_y + 14.0
        ));
        out.push_str(&format!(
            r##"<circle cx="{lx:.1}" cy="{floor_y:.1}" r="3" fill="#57606a" stroke="#111111" stroke-width="1.2" />"##
        ));
    }

    // IV Drip Stand (Haste de Soro com Bolsa e Tubo)
    let iv_x = bed_x - 12.0;
    let iv_top_y = py + 30.0;
    // Central pole
    out.push_str(&format!(
        r##"<line x1="{iv_x:.1}" y1="{iv_top_y:.1}" x2="{iv_x:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##
    ));
    // Double hanger hooks at top
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {iv_x:.1} {iv_top_y:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#111111" stroke-width="2" />"##,
        iv_x - 8.0, iv_top_y + 5.0,
        iv_x - 6.0, iv_top_y - 4.0,
        iv_x + 6.0, iv_top_y - 4.0,
        iv_x + 8.0, iv_top_y + 5.0
    ));
    // IV Bag (Bolsa de Solução / Soro)
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="10" height="16" fill="#e6edf5" stroke="#111111" stroke-width="1.2" rx="2" />"##,
        iv_x - 13.0,
        iv_top_y + 4.0
    ));
    // Fluid level & measurement marks
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#0969da" stroke-width="1.5" />"##,
        iv_x - 11.0, iv_top_y + 12.0,
        iv_x - 5.0, iv_top_y + 12.0
    ));
    // Dangling IV tube down toward the bed
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#111111" stroke-width="1" stroke-dasharray="3,1" />"##,
        iv_x - 8.0, iv_top_y + 20.0,
        iv_x, iv_top_y + 50.0,
        bed_x + 6.0, bed_y + 6.0
    ));
    // Wheeled base (tri-pod base)
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2.5" />"##,
        iv_x - 10.0,
        iv_x + 10.0
    ));

    // Floor baseline & sanitary baseboard
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// Biblioteca Universitária (BU UFSC) - Grand bookshelves in receding perspective, study carrels & "SILÊNCIO" sign
fn render_bu_ufsc_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    out.push_str(r##"<g class="backdrop-vector backdrop-bu-ufsc">"##);

    // Warm quiet study hall background
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    let floor_y = py + h - 16.0;

    // Receding tall bookshelves on the left (Estantes monumentais da BU)
    let num_shelves = 4;
    for s in 0..num_shelves {
        let shelf_y = py + 24.0 + (s as f64 * 28.0);
        let shelf_w = 75.0;
        out.push_str(&format!(
            r##"<rect x="{px:.1}" y="{shelf_y:.1}" width="{shelf_w:.1}" height="24" fill="#f6f8fa" stroke="#111111" stroke-width="1.8" />"##
        ));
        // Book spines inside shelf
        let mut bx = px + 4.0;
        let mut b_count = 0;
        while bx < px + shelf_w - 6.0 {
            let bw = if b_count % 3 == 0 {
                6.0
            } else if b_count % 2 == 0 {
                4.5
            } else {
                7.5
            };
            let bh = 18.0 - (b_count % 4) as f64;
            let fill = if b_count % 2 == 0 {
                "#eaeef2"
            } else {
                "#ffffff"
            };
            out.push_str(&format!(
                r##"<rect x="{bx:.1}" y="{:.1}" width="{bw:.1}" height="{bh:.1}" fill="{fill}" stroke="#111111" stroke-width="1" />"##,
                shelf_y + 24.0 - bh
            ));
            bx += bw + 1.2;
            b_count += 1;
        }
    }

    // Classic BU hanging sign "SILÊNCIO / SILENCE"
    let sign_x = px + (w / 2.0) - 30.0;
    let sign_y = py + 22.0;
    // Hanging chains
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{py:.1}" x2="{:.1}" y2="{sign_y:.1}" stroke="#111111" stroke-width="1.2" stroke-dasharray="2,2" />"##,
        sign_x + 10.0, sign_x + 10.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{py:.1}" x2="{:.1}" y2="{sign_y:.1}" stroke="#111111" stroke-width="1.2" stroke-dasharray="2,2" />"##,
        sign_x + 50.0, sign_x + 50.0
    ));
    // Sign plate
    out.push_str(&format!(
        r##"<rect x="{sign_x:.1}" y="{sign_y:.1}" width="60" height="20" fill="#ffffff" stroke="#111111" stroke-width="2" rx="2" />"##
    ));
    let font_silence = r##"font-family="'Helvetica Neue', Arial, sans-serif" font-size="7" font-weight="bold" fill="#cf222e" letter-spacing="1""##;
    let font_bu =
        r##"font-family="'Helvetica Neue', Arial, sans-serif" font-size="4.5" fill="#57606a""##;
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" {font_silence}>🤫 SILÊNCIO</text>"##,
        sign_x + 30.0,
        sign_y + 11.5
    ));
    out.push_str(&format!(
        r##"<text x="{:.1}" y="{:.1}" text-anchor="middle" {font_bu}>BU • UFSC</text>"##,
        sign_x + 30.0,
        sign_y + 17.5
    ));

    // Study Carrel (Baia de Estudo Individual de Madeira) on the right
    let carrel_x = px + w - 80.0;
    let carrel_y = py + 104.0;
    let carrel_w = 70.0;
    let carrel_h = 32.0;

    // Wooden partition back & side
    out.push_str(&format!(
        r##"<rect x="{carrel_x:.1}" y="{carrel_y:.1}" width="{carrel_w:.1}" height="{carrel_h:.1}" fill="#f6f8fa" stroke="#111111" stroke-width="2" rx="1" />"##
    ));
    // Desk surface
    let desk_y = carrel_y + 18.0;
    out.push_str(&format!(
        r##"<line x1="{carrel_x:.1}" y1="{desk_y:.1}" x2="{:.1}" y2="{desk_y:.1}" stroke="#111111" stroke-width="2.5" />"##,
        carrel_x + carrel_w
    ));
    // Open textbook on the desk
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
        carrel_x + 14.0, desk_y - 2.0,
        carrel_x + 24.0, desk_y - 5.0,
        carrel_x + 34.0, desk_y - 2.0,
        carrel_x + 24.0, desk_y + 1.0,
        carrel_x + 14.0, desk_y - 2.0
    ));
    // Articulated desk reading lamp
    let lamp_base_x = carrel_x + carrel_w - 18.0;
    out.push_str(&format!(
        r##"<path d="M {lamp_base_x:.1} {desk_y:.1} L {:.1} {:.1} L {:.1} {:.1}" fill="none" stroke="#111111" stroke-width="1.5" />"##,
        lamp_base_x - 6.0, desk_y - 12.0,
        lamp_base_x - 14.0, desk_y - 8.0
    ));
    // Lamp shade
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="#24292e" stroke="#111111" stroke-width="1" />"##,
        lamp_base_x - 18.0, desk_y - 10.0,
        lamp_base_x - 10.0, desk_y - 6.0,
        lamp_base_x - 14.0, desk_y - 5.0
    ));

    // Carrel legs down to floor
    let leg_right_x = carrel_x + carrel_w;
    out.push_str(&format!(
        r##"<line x1="{carrel_x:.1}" y1="{:.1}" x2="{carrel_x:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2" />"##,
        carrel_y + carrel_h
    ));
    out.push_str(&format!(
        r##"<line x1="{leg_right_x:.1}" y1="{:.1}" x2="{leg_right_x:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2" />"##,
        carrel_y + carrel_h
    ));

    // Floor line
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}
