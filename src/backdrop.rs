// Authentic vector-based comic backdrops in clean hand-inked comic style
// Inspired by Jim Woodring (Microsoft Comic Chat) and real UFSC campus architectural references.
// Strict Ink Guidelines: Zero system fonts/labels (<text>), zero saturated paint colors, pure ink strokes & hatching.

pub fn get_vector_backdrop_svg(name: &str, px: f64, py: f64, w: f64, h: f64) -> Option<String> {
    if let Some(disk_svg) = crate::assets::load_backdrop_from_disk(name) {
        return Some(disk_svg);
    }

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
        "praca_cidadania" | "praca" | "cidadania" => Some(render_praca_cidadania_svg(px, py, w, h)),
        "ctc" | "engenharia" | "tecnologico" => Some(render_ctc_svg(px, py, w, h)),
        "cfh" | "humanas" => Some(render_cfh_svg(px, py, w, h)),
        "pantanal" | "morro" | "itacorubi" => Some(render_pantanal_svg(px, py, w, h)),
        "pastoral" | "pastoral_vector" | "pastoral_vec" => {
            Some(render_pastoral_vector_svg(px, py, w, h))
        }
        "room" | "quarto" | "sala" => Some(render_room_svg(px, py, w, h)),
        "bosque_cfh" | "bosque" | "bosquinho" => Some(render_bosque_cfh_svg(px, py, w, h)),
        "reitoria" | "predio_reitoria" => Some(render_reitoria_svg(px, py, w, h)),
        "pista_atletismo" | "atletismo" | "cds" | "pista" => {
            Some(render_pista_atletismo_svg(px, py, w, h))
        }
        "centro_eventos" | "eventos" | "centro_de_eventos" => {
            Some(render_centro_eventos_svg(px, py, w, h))
        }
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
        "praca_cidadania",
        "ctc",
        "cfh",
        "pantanal",
        "room",
        "bosque_cfh",
        "reitoria",
        "pista_atletismo",
        "centro_eventos",
    ]
}

/// Helper para tramas de hachuras paralelas diagonais em nanquim
fn ink_hatch_lines(out: &mut String, x1: f64, y1: f64, x2: f64, y2: f64, step: f64) {
    let mut cur = y1;
    while cur <= y2 {
        out.push_str(&format!(
            r##"<line x1="{x1:.1}" y1="{cur:.1}" x2="{x2:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.6" opacity="0.55" />"##,
            cur - 4.0
        ));
        cur += step;
    }
}

/// 1. Sala de Aula UFSC - Lousa verde-escura ampla com formulas e graficos em giz organico, janelas basculantes com vista pro campus e carteiras universitarias
fn render_classroom_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-classroom">"##);

    // Parede de alvenaria e rodapé
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#faf9f5" />"##
    ));
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{floor_y:.1}" width="{w:.1}" height="{:.1}" fill="#f0ede3" stroke="#111111" stroke-width="1.4" />"##,
        h - (floor_y - py)
    ));

    // Janelas basculantes da UFSC à esquerda com vista para árvores do campus
    let win_x = px + 10.0;
    let win_y = py + 16.0;
    let win_w = (w * 0.28).min(70.0);
    let win_h = 100.0;
    out.push_str(&format!(
        r##"<rect x="{win_x:.1}" y="{win_y:.1}" width="{win_w:.1}" height="{win_h:.1}" fill="#f2f5f4" stroke="#111111" stroke-width="1.8" />"##
    ));
    // Bandeiras basculantes entreabertas
    for i in 1..4 {
        let by = win_y + (i as f64 * (win_h / 4.0));
        out.push_str(&format!(
            r##"<line x1="{win_x:.1}" y1="{by:.1}" x2="{:.1}" y2="{by:.1}" stroke="#111111" stroke-width="1.4" />"##,
            win_x + win_w
        ));
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{by:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" stroke-dasharray="2,2" />"##,
            win_x + 4.0, win_x + win_w - 4.0, by - 6.0
        ));
    }
    // Folhagens e galhos da mata da UFSC vistos através da janela em nanquim
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q 8 -12 18 -4 q 12 -16 22 2 q 10 -8 18 10 M {:.1} {:.1} q 10 14 2 24" fill="none" stroke="#222222" stroke-width="1.2" />"##,
        win_x + 4.0, win_y + win_h - 12.0, win_x + 10.0, win_y + win_h - 40.0
    ));

    // Lousa panorâmica tradicional da UFSC (moldura de madeira/alumínio e quadro verde-escuro)
    let board_x = win_x + win_w + 14.0;
    let board_y = py + 16.0;
    let board_w = (px + w - board_x - 12.0).max(80.0);
    let board_h = 95.0;

    out.push_str(&format!(
        r##"<rect x="{board_x:.1}" y="{board_y:.1}" width="{board_w:.1}" height="{board_h:.1}" fill="#1a2621" stroke="#111111" stroke-width="2.2" rx="1.5" />"##
    ));
    // Calha inferior de giz e apagador de madeira
    let tray_y = board_y + board_h;
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{tray_y:.1}" width="{:.1}" height="5.0" fill="#dedbd2" stroke="#111111" stroke-width="1.5" />"##,
        board_x - 4.0, board_w + 8.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="16.0" height="7.0" fill="#f4f1ea" stroke="#111111" stroke-width="1.0" rx="1" />"##,
        board_x + 14.0, tray_y - 6.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#ffffff" stroke-width="2.2" stroke-linecap="round" />"##,
        board_x + 36.0, tray_y - 2.5, board_x + 46.0, tray_y - 2.5
    ));

    // Cálculos, integrais e gráficos feitos a mão livre em giz no quadro (sem qualquer fonte digital!)
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} c -4 -10 6 -12 2 -22 c -3 -8 -8 2 -6 8 M {:.1} {:.1} l 14 0 M {:.1} {:.1} l 8 16 l 8 -16 M {:.1} {:.1} q 10 -4 16 2" fill="none" stroke="#f0efe8" stroke-width="1.3" opacity="0.88" />"##,
        board_x + 18.0, board_y + 40.0,
        board_x + 12.0, board_y + 30.0,
        board_x + 38.0, board_y + 26.0,
        board_x + 58.0, board_y + 34.0
    ));
    // Eixos cartesianos com parábola e hachuras da área sob a curva
    let gx = board_x + board_w - 55.0;
    let gy = board_y + 22.0;
    out.push_str(&format!(
        r##"<line x1="{gx:.1}" y1="{:.1}" x2="{gx:.1}" y2="{:.1}" stroke="#f0efe8" stroke-width="1.2" opacity="0.85" />"##,
        gy + 50.0, gy
    ));
    out.push_str(&format!(
        r##"<line x1="{gx:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#f0efe8" stroke-width="1.2" opacity="0.85" />"##,
        gy + 45.0, gx + 45.0, gy + 45.0
    ));
    out.push_str(&format!(
        r##"<path d="M {gx:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#f0efe8" stroke-width="1.4" opacity="0.9" />"##,
        gy + 45.0, gx + 20.0, gy + 4.0, gx + 40.0, gy + 45.0
    ));
    // Hachura da integral sob a curva
    for k in 0..5 {
        let hx = gx + 6.0 + (k as f64 * 6.0);
        out.push_str(&format!(
            r##"<line x1="{hx:.1}" y1="{:.1}" x2="{hx:.1}" y2="{:.1}" stroke="#f0efe8" stroke-width="0.8" stroke-dasharray="2,2" opacity="0.75" />"##,
            gy + 44.0, gy + 20.0 + (k as f64 * 3.0)
        ));
    }

    // Carteira universitária com prancheta de braço em primeiro plano
    let desk_x = px + 18.0;
    let desk_y = floor_y - 28.0;
    out.push_str(&format!(
        r##"<path d="M {desk_x:.1} {desk_y:.1} l 28 -6 l 14 18 l -28 6 z" fill="#eae5d8" stroke="#111111" stroke-width="1.5" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{desk_x:.1}" y1="{desk_y:.1}" x2="{desk_x:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2.0" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        desk_x + 28.0, desk_y - 6.0, desk_x + 28.0
    ));

    // Tábuas de assoalho em perspectiva
    for i in 0..6 {
        let fx = px + (i as f64 * (w / 5.0));
        out.push_str(&format!(
            r##"<line x1="{fx:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.0" opacity="0.45" />"##,
            fx + 18.0, py + h
        ));
    }

    out.push_str("</g>");
    out
}

/// 2. Café do CCS - Máquina de expresso detalhada, vitrine de salgados, pérgola de madeira com samambaias e xadrez de ladrilho
fn render_cafe_ccs_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-cafe-ccs">"##);

    // Fundo aconchegante
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#faf8f5" />"##
    ));

    // Pérgola de vigas de madeira no teto
    for i in 0..7 {
        let vx = px + 6.0 + (i as f64 * (w / 6.0));
        out.push_str(&format!(
            r##"<line x1="{vx:.1}" y1="{py:.1}" x2="{:.1}" y2="{:.1}" stroke="#222222" stroke-width="2.4" />"##,
            vx + 14.0, py + 24.0
        ));
    }
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.5" />"##,
        py + 23.0, px + w, py + 23.0
    ));

    // Folhagens de samambaias e jiboias penduradas na pérgola
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q -8 18 2 28 q 6 -14 2 -24 M {:.1} {:.1} q 10 16 -2 26" fill="none" stroke="#222222" stroke-width="1.4" />"##,
        px + 24.0, py + 23.0, px + 36.0, py + 23.0
    ));

    // Balcão de café de madeira maciça com veios
    let bar_y = py + 72.0;
    let bar_h = floor_y - bar_y;
    let bar_w = (w * 0.65).min(170.0);
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{bar_y:.1}" width="{bar_w:.1}" height="{bar_h:.1}" fill="#f2ede4" stroke="#111111" stroke-width="2.2" />"##
    ));
    // Prancha de mogno superior do balcão
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{:.1}" width="{:.1}" height="8.0" fill="#e8dfcf" stroke="#111111" stroke-width="2.0" />"##,
        bar_y - 8.0, bar_w + 6.0
    ));
    // Painéis de madeira do balcão com hachuras
    for j in 0..3 {
        let pnl_x = px + 12.0 + (j as f64 * 48.0);
        if pnl_x + 38.0 < px + bar_w {
            out.push_str(&format!(
                r##"<rect x="{pnl_x:.1}" y="{:.1}" width="38.0" height="{:.1}" fill="none" stroke="#111111" stroke-width="1.2" />"##,
                bar_y + 10.0, bar_h - 20.0
            ));
            ink_hatch_lines(
                &mut out,
                pnl_x + 4.0,
                bar_y + 16.0,
                pnl_x + 34.0,
                bar_y + bar_h - 14.0,
                7.0,
            );
        }
    }

    // Máquina de expresso italiana profissional sobre o balcão
    let m_x = px + 22.0;
    let m_y = bar_y - 42.0;
    out.push_str(&format!(
        r##"<rect x="{m_x:.1}" y="{m_y:.1}" width="42.0" height="34.0" fill="#eae8e1" stroke="#111111" stroke-width="1.8" rx="2" />"##
    ));
    // Manômetro circular duplo e grupo de extração com cachimbo
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="4.0" fill="#ffffff" stroke="#111111" stroke-width="1.1" />"##,
        m_x + 12.0, m_y + 10.0
    ));
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="4.0" fill="#ffffff" stroke="#111111" stroke-width="1.1" />"##,
        m_x + 24.0, m_y + 10.0
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 0 10 l 6 0 l 0 -10 l 12 3" fill="none" stroke="#111111" stroke-width="1.6" />"##,
        m_x + 16.0, m_y + 20.0
    ));
    // Xícara de café fumegante sobre a grade
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="10.0" height="7.0" fill="#ffffff" stroke="#111111" stroke-width="1.1" rx="1" />"##,
        m_x + 14.0, m_y + 30.0
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q -3 -6 0 -10 q 3 -4 0 -8" fill="none" stroke="#111111" stroke-width="0.9" opacity="0.75" />"##,
        m_x + 19.0, m_y + 28.0
    ));

    // Vitrine de vidro de salgados ao lado
    let v_x = m_x + 52.0;
    out.push_str(&format!(
        r##"<path d="M {v_x:.1} {bar_y:.1} l 0 -28 l 30 0 l 6 28 z" fill="#f4f7f6" stroke="#111111" stroke-width="1.6" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.6" />"##,
        v_x + 4.0, bar_y - 24.0, v_x + 26.0, bar_y - 4.0
    ));

    // Piso com mosaico em xadrez de nanquim
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        px + w
    ));
    for k in 0..5 {
        let tx = px + bar_w + (k as f64 * 22.0);
        if tx < px + w {
            out.push_str(&format!(
                r##"<line x1="{tx:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.9" />"##,
                tx - 16.0, py + h
            ));
        }
    }

    out.push_str("</g>");
    out
}

/// 3. Laboratório UFSC - Azulejos com rejunte de nanquim, microscópio detalhado, vidrarias graduadas com menisco e suporte universal
fn render_laboratory_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-laboratory">"##);

    // Fundo de azulejos claros de laboratório com rejunte de nanquim
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#f9fafb" />"##
    ));
    let tile_w = 26.0;
    let tile_h = 20.0;
    let mut ty = py;
    while ty < floor_y - 30.0 {
        out.push_str(&format!(
            r##"<line x1="{px:.1}" y1="{ty:.1}" x2="{:.1}" y2="{ty:.1}" stroke="#222222" stroke-width="0.5" opacity="0.4" />"##,
            px + w
        ));
        ty += tile_h;
    }
    let mut tx = px;
    while tx < px + w {
        out.push_str(&format!(
            r##"<line x1="{tx:.1}" y1="{py:.1}" x2="{tx:.1}" y2="{:.1}" stroke="#222222" stroke-width="0.5" opacity="0.4" />"##,
            floor_y - 30.0
        ));
        tx += tile_w;
    }

    // Bancada de laboratório pesada com tampo de granito cinza e chanfro
    let bench_y = py + 68.0;
    let bench_h = floor_y - bench_y;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{bench_y:.1}" width="{w:.1}" height="{bench_h:.1}" fill="#e8eaed" stroke="#111111" stroke-width="2.2" />"##
    ));
    // Tampo superior de granito escuro com hachuras de textura
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{:.1}" width="{w:.1}" height="8.0" fill="#2d3748" stroke="#111111" stroke-width="2.0" />"##,
        bench_y - 8.0
    ));
    // Gaveteiros de laboratório com puxadores de concha em nanquim
    for g in 0..3 {
        let gx = px + 14.0 + (g as f64 * (w / 3.2));
        if gx + 45.0 < px + w {
            out.push_str(&format!(
                r##"<rect x="{gx:.1}" y="{:.1}" width="42.0" height="{:.1}" fill="none" stroke="#111111" stroke-width="1.2" />"##,
                bench_y + 10.0, bench_h - 18.0
            ));
            out.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.0" stroke-linecap="round" />"##,
                gx + 14.0, bench_y + 22.0, gx + 28.0, bench_y + 22.0
            ));
        }
    }

    // Microscópio óptico binocular sobre a bancada
    let mic_x = px + (w * 0.28).min(75.0);
    let mic_y = bench_y - 8.0;
    // Base em ferradura pesada
    out.push_str(&format!(
        r##"<path d="M {:.1} {mic_y:.1} c -4 0 -12 -3 -12 -8 l 24 0 c 0 5 -8 8 -12 8 z" fill="#1b2028" stroke="#111111" stroke-width="1.6" />"##,
        mic_x
    ));
    // Coluna do braço curvo e parafusos micrométricos estriados
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q 8 -18 2 -32 l -10 -4" fill="none" stroke="#111111" stroke-width="3.0" />"##,
        mic_x, mic_y - 8.0
    ));
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="3.2" fill="#d1d5db" stroke="#111111" stroke-width="1.0" />"##,
        mic_x + 5.0, mic_y - 18.0
    ));
    // Platina porta-lâminas e revólver de 3 objetivas
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.2" />"##,
        mic_x - 14.0, mic_y - 22.0, mic_x + 4.0, mic_y - 22.0
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l -4 6 l 4 0 l 0 -6 M {:.1} {:.1} l -2 6 l 3 0 l 0 -6" fill="#111111" stroke="#111111" stroke-width="1.0" />"##,
        mic_x - 4.0, mic_y - 28.0, mic_x - 9.0, mic_y - 28.0
    ));
    // Tubo ocular inclinado
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.4" />"##,
        mic_x - 8.0, mic_y - 44.0, mic_x - 14.0, mic_y - 52.0
    ));

    // Suporte universal com bureta graduada e erlenmeyer
    let sup_x = mic_x + 52.0;
    out.push_str(&format!(
        r##"<line x1="{sup_x:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.2" />"##,
        mic_y, sup_x + 22.0, mic_y
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.8" />"##,
        sup_x + 11.0, mic_y, sup_x + 11.0, mic_y - 48.0
    ));
    // Garra metálica segurando erlenmeyer com líquido e menisco hachurado
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 3 -10 l 6 0 l 3 10 z" fill="#f0fdf4" stroke="#111111" stroke-width="1.2" />"##,
        sup_x + 6.0, mic_y
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.7" />"##,
        sup_x + 8.0, mic_y - 4.0, sup_x + 16.0, mic_y - 4.0
    ));

    // Piso com rodapé
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.6" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 4. HU Fachada UFSC - Arquitetura brutalista do Hospital Universitário Polydoro Ernani com brises-soleils ritmicos, marquise suspensa, palmeiras e ambulância
/// 4. HU Fachada UFSC - Arquitetura brutalista do Hospital Universitário Polydoro Ernani baseada nas fotos reais.
///
/// Inclui a placa monumental frontal, torre d'água cilíndrica, blocos de leitos em perspectiva, poste duplo e cerca de tela.
fn render_hu_fachada_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h * 0.92;
    out.push_str(r##"<g class="backdrop-vector backdrop-hu-fachada">"##);

    // Céu e silhueta geral
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Nuvens estilizadas em arco clássico de nanquim
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#222222" stroke-width="1.0" opacity="0.55" />"##,
        px + w * 0.05, py + h * 0.05,
        px + w * 0.11, py + h * 0.02,
        px + w * 0.17, py + h * 0.05,
        px + w * 0.22, py + h * 0.03,
        px + w * 0.27, py + h * 0.06
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#222222" stroke-width="1.0" opacity="0.55" />"##,
        px + w * 0.53, py + h * 0.04,
        px + w * 0.60, py + h * 0.01,
        px + w * 0.67, py + h * 0.04,
        px + w * 0.72, py + h * 0.02,
        px + w * 0.78, py + h * 0.05
    ));

    // Morros de Florianópolis ao fundo (vegetação da Trindade / Maciço da Cruz)
    let m_y = py + h * 0.28;
    out.push_str(&format!(
        r##"<path d="M {px:.1} {m_y:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} L {:.1} {:.1} L {px:.1} {:.1} Z" fill="#f1f3f5" stroke="#111111" stroke-width="1.2" />"##,
        px + w * 0.25, py + h * 0.20,
        px + w * 0.52, py + h * 0.27,
        px + w * 0.77, py + h * 0.18,
        px + w, py + h * 0.25,
        px + w, py + h * 0.37,
        py + h * 0.37
    ));
    // Hachuras do morro
    for k in 0..16 {
        let hx = px + w * 0.04 + (k as f64 * (w * 0.06));
        out.push_str(&format!(
            r##"<line x1="{hx:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.6" opacity="0.35" />"##,
            py + h * 0.27, hx - 5.0, py + h * 0.33
        ));
    }

    // Caixa d'água / Torre cilíndrica do HU à esquerda (hu-water-tower)
    let tw_x = px + w * 0.033;
    let tw_y = py + h * 0.167;
    let tw_w = w * 0.053;
    let tw_h = h * 0.60;
    out.push_str(r##"<g class="hu-water-tower">"##);
    out.push_str(&format!(
        r##"<rect x="{tw_x:.1}" y="{tw_y:.1}" width="{tw_w:.1}" height="{tw_h:.1}" fill="#f8f9fa" stroke="#111111" stroke-width="1.8" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{tw_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.5" />"##,
        tw_x + tw_w * 0.3, tw_x + tw_w * 0.3, tw_y + tw_h
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{tw_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.5" />"##,
        tw_x + tw_w * 0.7, tw_x + tw_w * 0.7, tw_y + tw_h
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="#ffffff" stroke="#111111" stroke-width="1.4" />"##,
        tw_x - 2.0, tw_y - 4.0, tw_w + 4.0, 6.0
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} h 3 v -2 h 2 v 2 h 3 v 2 h -3 v 3 h -2 v -3 h -3 z" fill="#111111" />"##,
        tw_x + tw_w * 0.3, tw_y + 8.0
    ));
    out.push_str("</g>");

    // Blocos do hospital em perspectiva e volumes escalonados (hu-hospital-blocks)
    out.push_str(r##"<g class="hu-hospital-blocks">"##);
    // Pavilhão 1 (Recuado)
    let p1_x = px + w * 0.30;
    let p1_y = py + h * 0.307;
    let p1_w = w * 0.16;
    let p1_h = h * 0.45;
    out.push_str(&format!(
        r##"<rect x="{p1_x:.1}" y="{p1_y:.1}" width="{p1_w:.1}" height="{p1_h:.1}" fill="#ffffff" stroke="#111111" stroke-width="1.8" />"##
    ));
    out.push_str(&format!(
        r##"<rect x="{p1_x:.1}" y="{p1_y:.1}" width="{:.1}" height="{p1_h:.1}" fill="#f1f3f5" stroke="#111111" stroke-width="1.4" />"##,
        p1_w * 0.3
    ));
    // Pavilhão 2 (Intermediário)
    let p2_x = px + w * 0.473;
    let p2_y = py + h * 0.267;
    let p2_w = w * 0.193;
    let p2_h = h * 0.50;
    out.push_str(&format!(
        r##"<rect x="{p2_x:.1}" y="{p2_y:.1}" width="{p2_w:.1}" height="{p2_h:.1}" fill="#ffffff" stroke="#111111" stroke-width="2.0" />"##
    ));
    out.push_str(&format!(
        r##"<rect x="{p2_x:.1}" y="{p2_y:.1}" width="{:.1}" height="{p2_h:.1}" fill="#f1f3f5" stroke="#111111" stroke-width="1.4" />"##,
        p2_w * 0.28
    ));
    // Pavilhão 3 (Frontal)
    let p3_x = px + w * 0.68;
    let p3_y = py + h * 0.227;
    let p3_w = w * 0.293;
    let p3_h = h * 0.55;
    out.push_str(&format!(
        r##"<rect x="{p3_x:.1}" y="{p3_y:.1}" width="{p3_w:.1}" height="{p3_h:.1}" fill="#ffffff" stroke="#111111" stroke-width="2.2" />"##
    ));
    out.push_str(&format!(
        r##"<rect x="{p3_x:.1}" y="{p3_y:.1}" width="{:.1}" height="{p3_h:.1}" fill="#f1f3f5" stroke="#111111" stroke-width="1.6" />"##,
        p3_w * 0.25
    ));
    // Janelas e brises verticais nos andares do pavilhão frontal
    for fl in 0..5 {
        let f_y = p3_y + 14.0 + (fl as f64 * 26.0);
        out.push_str(&format!(
            r##"<rect x="{:.1}" y="{f_y:.1}" width="{:.1}" height="16.0" fill="#ffffff" stroke="#111111" stroke-width="1.4" />"##,
            p3_x + p3_w * 0.3, p3_w * 0.65
        ));
        for br in 0..8 {
            let br_x = p3_x + p3_w * 0.32 + (br as f64 * 6.0);
            out.push_str(&format!(
                r##"<line x1="{br_x:.1}" y1="{f_y:.1}" x2="{br_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" />"##,
                f_y + 16.0
            ));
        }
    }
    // Pilotis e marquise térrea
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="#f8f9fa" stroke="#111111" stroke-width="1.6" />"##,
        p2_x - 2.0, ground_y - 46.0,
        px + w - 8.0, ground_y - 46.0,
        px + w - 8.0, ground_y - 34.0,
        p2_x - 2.0, ground_y - 34.0
    ));
    for pil in 0..6 {
        let pil_x = p2_x + 10.0 + (pil as f64 * 24.0);
        out.push_str(&format!(
            r##"<line x1="{pil_x:.1}" y1="{:.1}" x2="{pil_x:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
            ground_y - 34.0
        ));
    }
    out.push_str("</g>");

    // Poste de iluminação icônico do HU (hu-lamp-post)
    let lp_x = px + w * 0.113;
    out.push_str(r##"<g class="hu-lamp-post">"##);
    out.push_str(&format!(
        r##"<line x1="{lp_x:.1}" y1="{:.1}" x2="{lp_x:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        py + h * 0.217
    ));
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="#ffffff" stroke="#111111" stroke-width="1.4" />"##,
        lp_x - 6.0, py + h * 0.20, lp_x + 2.0, py + h * 0.18, lp_x + 8.0, py + h * 0.20, lp_x, py + h * 0.22
    ));
    out.push_str("</g>");

    // Placa monumental frontal do HU (hu-monumental-sign)
    let s_x = px + w * 0.04;
    let s_y = py + h * 0.227;
    let s_w = w * 0.32;
    let s_h = h * 0.227;
    let post_x = s_x + s_w * 0.46;
    out.push_str(r##"<g class="hu-monumental-sign">"##);
    out.push_str(&format!(
        r##"<rect x="{post_x:.1}" y="{:.1}" width="5.0" height="{:.1}" fill="#111111" stroke="#111111" stroke-width="1.2" />"##,
        s_y + s_h - 2.0, ground_y - (s_y + s_h - 2.0)
    ));
    out.push_str(&format!(
        r##"<rect x="{s_x:.1}" y="{s_y:.1}" width="{s_w:.1}" height="{s_h:.1}" rx="2" fill="#ffffff" stroke="#111111" stroke-width="2.6" />"##
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="none" stroke="#111111" stroke-width="0.8" />"##,
        s_x + 2.0, s_y + 2.0, s_w - 4.0, s_h - 4.0
    ));
    // Faixa superior com brasão/cruz estilizados em nanquim
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" />"##,
        s_x + 2.0, s_y + 10.0, s_x + s_w - 2.0, s_y + 10.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="10.0" height="12.0" fill="none" stroke="#111111" stroke-width="1.2" />"##,
        s_x + 14.0, s_y + 14.0
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} h 3 v -3 h 3 v 3 h 3 v 3 h -3 v 3 h -3 v -3 h -3 z" fill="#111111" />"##,
        s_x + 28.0, s_y + 17.0
    ));
    // Linhas tipográficas gravadas em nanquim
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.6" />"##,
        s_x + 10.0, s_y + 34.0, s_x + s_w - 10.0, s_y + 34.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.6" />"##,
        s_x + 13.0, s_y + 42.0, s_x + s_w - 13.0, s_y + 42.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="22.0" height="6.0" fill="#111111" />"##,
        s_x + 16.0,
        s_y + 50.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" />"##,
        s_x + 42.0, s_y + 53.0, s_x + s_w - 16.0, s_y + 53.0
    ));
    out.push_str("</g>");

    // Cerca aramada em losangos na frente da calçada (hu-mesh-fence)
    let fence_top = ground_y - 61.0;
    out.push_str(r##"<g class="hu-mesh-fence">"##);
    for p in 0..7 {
        let px_post = px + 2.0 + (p as f64 * 22.0);
        out.push_str(&format!(
            r##"<line x1="{px_post:.1}" y1="{fence_top:.1}" x2="{px_post:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="1.4" />"##
        ));
    }
    for m in 0..18 {
        let mx = px + 2.0 + (m as f64 * 7.0);
        out.push_str(&format!(
            r##"<line x1="{mx:.1}" y1="{:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="0.5" opacity="0.4" />"##,
            fence_top + 3.0, mx + 14.0
        ));
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{mx:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="0.5" opacity="0.4" />"##,
            mx + 14.0, fence_top + 3.0
        ));
    }
    out.push_str("</g>");

    // Linha do chão / calçada
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.2" />"##,
        px + w
    ));

    // Grama e tufos de nanquim no canteiro
    for t in 0..25 {
        let tx = px + 4.0 + (t as f64 * 12.0);
        out.push_str(&format!(
            r##"<line x1="{tx:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.9" opacity="0.6" />"##,
            tx - 3.0, ground_y - 5.0
        ));
        out.push_str(&format!(
            r##"<line x1="{tx:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.6" />"##,
            tx + 2.0, ground_y - 4.0
        ));
    }

    out.push_str("</g>");
    out
}

/// 5. RU UFSC - Vão monumental do Restaurante Universitário, tesouras de aço no teto, esteira de bandejas de inox e mesas em perspectiva profunda
fn render_ru_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-ru">"##);

    // Salão iluminado
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#faf9f6" />"##
    ));

    // Estrutura metálica industrial em treliça espacial no teto do RU
    for i in 0..5 {
        let tx = px + 10.0 + (i as f64 * (w / 4.2));
        out.push_str(&format!(
            r##"<line x1="{tx:.1}" y1="{py:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.8" />"##,
            tx + 18.0, py + 26.0
        ));
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{py:.1}" x2="{tx:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.8" />"##,
            tx + 36.0, py + 26.0
        ));
    }
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.2" />"##,
        py + 26.0, px + w, py + 26.0
    ));

    // Janelões envidraçados ao fundo do salão
    let win_y = py + 32.0;
    let win_h = 42.0;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{win_y:.1}" width="{:.1}" height="{win_h:.1}" fill="#f2f5f4" stroke="#111111" stroke-width="1.4" />"##,
        w * 0.6
    ));
    for p in 1..4 {
        let p_x = px + (p as f64 * ((w * 0.6) / 4.0));
        out.push_str(&format!(
            r##"<line x1="{p_x:.1}" y1="{win_y:.1}" x2="{p_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.0" />"##,
            win_y + win_h
        ));
    }

    // Mesas de fórmica compridas com bancos integrados em ponto de fuga
    let vp_x = px + (w * 0.35);
    let vp_y = py + 55.0;
    for m in 0..3 {
        let start_x = px + 10.0 + (m as f64 * 32.0);
        out.push_str(&format!(
            r##"<polygon points="{start_x:.1},{floor_y:.1} {:.1},{floor_y:.1} {:.1},{vp_y:.1} {:.1},{vp_y:.1}" fill="#f0ece1" stroke="#111111" stroke-width="1.5" />"##,
            start_x + 22.0, vp_x + (m as f64 * 8.0) + 4.0, vp_x + (m as f64 * 8.0)
        ));
    }

    // Balcão de distribuição e esteira de devolução de bandejas à direita
    let cntr_x = px + (w * 0.65).min(w - 70.0);
    let cntr_y = py + 68.0;
    let cntr_w = px + w - cntr_x;
    let cntr_h = floor_y - cntr_y;
    out.push_str(&format!(
        r##"<rect x="{cntr_x:.1}" y="{cntr_y:.1}" width="{cntr_w:.1}" height="{cntr_h:.1}" fill="#e8eaed" stroke="#111111" stroke-width="2.2" />"##
    ));
    // Trilho de inox para escorregar bandejas
    out.push_str(&format!(
        r##"<line x1="{cntr_x:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.0" />"##,
        cntr_y - 4.0, px + w, cntr_y - 4.0
    ));
    out.push_str(&format!(
        r##"<line x1="{cntr_x:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.0" />"##,
        cntr_y - 8.0, px + w, cntr_y - 8.0
    ));

    // A emblemática bandeja de inox do bandejão com as cavidades para refeição
    let tray_x = cntr_x + 10.0;
    let tray_y = cntr_y - 14.0;
    out.push_str(&format!(
        r##"<rect x="{tray_x:.1}" y="{tray_y:.1}" width="34.0" height="18.0" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.6" />"##
    ));
    // Cavidades individuais na bandeja
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="14.0" height="12.0" rx="2" fill="#f8f9fa" stroke="#111111" stroke-width="1.0" />"##,
        tray_x + 3.0, tray_y + 3.0
    ));
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{:.1}" r="4.0" fill="#f8f9fa" stroke="#111111" stroke-width="1.0" />"##,
        tray_x + 24.0, tray_y + 8.0
    ));

    // Linha do piso
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 6. Hall do CCS UFSC - Pilares monumentais de concreto aparente com textura de cofragem, jardim de inverno interno com costelas-de-adão e mural estudantil
fn render_hall_ccs_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-hall-ccs">"##);

    // Fundo do hall
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafaf8" />"##
    ));

    // Passarela suspensa do segundo andar com guarda-corpo tubular
    let pass_y = py + 38.0;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{pass_y:.1}" width="{w:.1}" height="12.0" fill="#eae8e1" stroke="#111111" stroke-width="2.0" />"##
    ));
    for r in 0..14 {
        let rx = px + 6.0 + (r as f64 * (w / 13.0));
        out.push_str(&format!(
            r##"<line x1="{rx:.1}" y1="{:.1}" x2="{rx:.1}" y2="{pass_y:.1}" stroke="#111111" stroke-width="1.2" />"##,
            pass_y - 16.0
        ));
    }
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.8" />"##,
        pass_y - 16.0, px + w, pass_y - 16.0
    ));

    // Pilares monumentais de concreto armado com textura de fôrma de madeira
    let pilar_x = px + (w * 0.22).min(50.0);
    out.push_str(&format!(
        r##"<rect x="{pilar_x:.1}" y="{pass_y:.1}" width="28.0" height="{:.1}" fill="#f0eee6" stroke="#111111" stroke-width="2.2" />"##,
        floor_y - pass_y
    ));
    // Texturas horizontais de cofragem no pilar
    for k in 0..8 {
        let ky = pass_y + 12.0 + (k as f64 * 16.0);
        if ky < floor_y {
            out.push_str(&format!(
                r##"<line x1="{pilar_x:.1}" y1="{ky:.1}" x2="{:.1}" y2="{ky:.1}" stroke="#111111" stroke-width="0.8" opacity="0.5" />"##,
                pilar_x + 28.0
            ));
        }
    }

    // O icônico jardim de inverno do CCS com mureta de pedra e costelas-de-adão
    let jard_x = px + w - 85.0;
    let jard_y = floor_y - 36.0;
    // Mureta de pedras curvas
    out.push_str(&format!(
        r##"<path d="M {jard_x:.1} {floor_y:.1} q 15 -18 45 -12 q 25 4 40 12 z" fill="#e2ded4" stroke="#111111" stroke-width="1.8" />"##
    ));
    // Costelas-de-adão (Monstera) com recortes de folha em bico de pena
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q -8 -22 14 -32 q 12 14 -14 32 M {:.1} {:.1} l -6 -6 M {:.1} {:.1} l -6 -4" fill="none" stroke="#222222" stroke-width="1.5" />"##,
        jard_x + 22.0, jard_y + 14.0, jard_x + 26.0, jard_y - 6.0, jard_x + 28.0, jard_y - 16.0
    ));
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q 12 -20 30 -16 q -4 20 -30 16" fill="none" stroke="#222222" stroke-width="1.4" />"##,
        jard_x + 36.0, jard_y + 12.0
    ));

    // Mural de avisos estudantis com cartazes sobrepostos desenhados à mão
    let mrl_x = pilar_x + 38.0;
    let mrl_y = pass_y + 24.0;
    out.push_str(&format!(
        r##"<rect x="{mrl_x:.1}" y="{mrl_y:.1}" width="42.0" height="30.0" fill="#fdfbf7" stroke="#111111" stroke-width="1.3" />"##
    ));
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="#f7f4ed" stroke="#111111" stroke-width="1.1" />"##,
        mrl_x + 6.0, mrl_y + 4.0, mrl_x + 22.0, mrl_y + 2.0, mrl_x + 20.0, mrl_y + 18.0, mrl_x + 4.0, mrl_y + 18.0
    ));
    ink_hatch_lines(
        &mut out,
        mrl_x + 6.0,
        mrl_y + 8.0,
        mrl_x + 18.0,
        mrl_y + 16.0,
        3.0,
    );

    // Linha do piso
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 7. HU Enfermaria - Leito hospitalar articulado de ferro, suporte telescópico de soro com gotejador, régua de gases na parede e biombo pregueado
fn render_hu_enfermaria_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-hu-enfermaria">"##);

    // Parede de enfermaria asséptica e calma
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Régua hospitalar de gases medicinais na parede (oxigênio, ar e vácuo em nanquim)
    let reg_x = px + (w * 0.35);
    let reg_y = py + 38.0;
    out.push_str(&format!(
        r##"<rect x="{reg_x:.1}" y="{reg_y:.1}" width="65.0" height="12.0" fill="#f1f3f5" stroke="#111111" stroke-width="1.4" rx="2" />"##
    ));
    for i in 0..3 {
        let cx = reg_x + 12.0 + (i as f64 * 18.0);
        out.push_str(&format!(
            r##"<circle cx="{cx:.1}" cy="{:.1}" r="3.2" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
            reg_y + 6.0
        ));
    }

    // Biombo hospitalar dobrável de três corpos à esquerda com cortina pregueada
    let bio_x = px + 12.0;
    let bio_w = (w * 0.28).min(65.0);
    let bio_h = 92.0;
    out.push_str(&format!(
        r##"<rect x="{bio_x:.1}" y="{:.1}" width="{bio_w:.1}" height="{bio_h:.1}" fill="#f8faf9" stroke="#111111" stroke-width="1.8" />"##,
        floor_y - bio_h
    ));
    // Pregas verticais da cortina com hachuras de sombra
    for k in 1..4 {
        let kx = bio_x + (k as f64 * (bio_w / 4.0));
        out.push_str(&format!(
            r##"<line x1="{kx:.1}" y1="{:.1}" x2="{kx:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.0" />"##,
            floor_y - bio_h
        ));
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="0.6" opacity="0.45" />"##,
            kx + 2.0, floor_y - bio_h + 8.0, kx + 2.0
        ));
    }

    // Leito hospitalar articulado
    let bed_x = bio_x + bio_w + 14.0;
    let bed_y = floor_y - 42.0;
    let bed_w = (px + w - bed_x - 16.0).max(75.0);

    // Cabeceira tubular de ferro curva
    out.push_str(&format!(
        r##"<path d="M {bed_x:.1} {floor_y:.1} l 0 -36 q 6 -8 16 -8 l 0 44" fill="none" stroke="#111111" stroke-width="2.2" />"##
    ));
    // Colchão hospitalar com vincos de lençol
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{bed_y:.1}" width="{:.1}" height="12.0" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.6" />"##,
        bed_x + 8.0, bed_w - 10.0
    ));
    // Grade de proteção articulada
    for g in 0..4 {
        let gx = bed_x + 18.0 + (g as f64 * 14.0);
        out.push_str(&format!(
            r##"<line x1="{gx:.1}" y1="{bed_y:.1}" x2="{gx:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.3" />"##,
            bed_y - 10.0
        ));
    }
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.6" />"##,
        bed_x + 16.0, bed_y - 10.0, bed_x + 62.0, bed_y - 10.0
    ));

    // Suporte de soro com rodízios, frasco e equipo com câmara de gotejamento
    let sup_x = bed_x - 6.0;
    out.push_str(&format!(
        r##"<line x1="{sup_x:.1}" y1="{floor_y:.1}" x2="{sup_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.8" />"##,
        py + 18.0
    ));
    // Rodízios na base
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        sup_x - 6.0, sup_x + 6.0
    ));
    // Frasco de soro invertido e mangueira flexível
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 8 0 l 0 14 q -4 4 -8 0 z" fill="#f8fafc" stroke="#111111" stroke-width="1.3" />"##,
        sup_x - 4.0, py + 22.0
    ));
    out.push_str(&format!(
        r##"<path d="M {sup_x:.1} {:.1} q 8 20 -2 36" fill="none" stroke="#111111" stroke-width="0.9" opacity="0.8" />"##,
        py + 38.0
    ));

    // Linha do piso
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 8. BU UFSC - Perspectiva monumental de corredor de estantes de livros, lombadas individuais em bico de pena, mesas de leitura e laje nervurada
fn render_bu_ufsc_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-bu-ufsc">"##);

    // Fundo do salão
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fbfaf7" />"##
    ));

    // Teto de laje nervurada de concreto (caixotões clássicos dos anos 70 da BU)
    for c in 0..6 {
        let cx = px + (c as f64 * (w / 5.0));
        out.push_str(&format!(
            r##"<rect x="{cx:.1}" y="{py:.1}" width="{:.1}" height="18.0" fill="#f2efe6" stroke="#111111" stroke-width="1.3" />"##,
            w / 5.2
        ));
    }

    // Perspectiva convergente para o ponto de fuga central
    let vp_x = px + (w / 2.0);
    let vp_y = py + 65.0;

    // Estante de livros monumental à esquerda (de alto a baixo)
    let left_w = (w * 0.38).min(90.0);
    out.push_str(&format!(
        r##"<polygon points="{px:.1},{:.1} {:.1},{:.1} {:.1},{floor_y:.1} {px:.1},{floor_y:.1}" fill="#ede9de" stroke="#111111" stroke-width="2.2" />"##,
        py + 18.0, px + left_w, vp_y - 20.0, px + left_w
    ));
    // Prateleiras e lombadas de livros na estante da esquerda
    for p in 0..5 {
        let py_start = py + 26.0 + (p as f64 * 20.0);
        let py_end = vp_y - 15.0 + (p as f64 * 10.0);
        out.push_str(&format!(
            r##"<line x1="{px:.1}" y1="{py_start:.1}" x2="{:.1}" y2="{py_end:.1}" stroke="#111111" stroke-width="1.5" />"##,
            px + left_w
        ));
        // Lombadas de livros verticais
        for b in 0..7 {
            let bx = px + 6.0 + (b as f64 * 11.0);
            if bx < px + left_w - 4.0 {
                out.push_str(&format!(
                    r##"<line x1="{bx:.1}" y1="{py_start:.1}" x2="{bx:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.1" />"##,
                    py_start - 12.0
                ));
                // Hachuras finas na lombada
                out.push_str(&format!(
                    r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.6" opacity="0.5" />"##,
                    bx + 2.0, py_start - 10.0, bx + 2.0, py_start - 2.0
                ));
            }
        }
    }

    // Estante monumental à direita
    let right_x = px + w - left_w;
    out.push_str(&format!(
        r##"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{floor_y:.1} {:.1},{floor_y:.1}" fill="#ede9de" stroke="#111111" stroke-width="2.2" />"##,
        px + w, py + 18.0, right_x, vp_y - 20.0, right_x, px + w
    ));
    for p in 0..5 {
        let py_start = py + 26.0 + (p as f64 * 20.0);
        let py_end = vp_y - 15.0 + (p as f64 * 10.0);
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{py_start:.1}" x2="{right_x:.1}" y2="{py_end:.1}" stroke="#111111" stroke-width="1.5" />"##,
            px + w
        ));
        for b in 0..7 {
            let bx = right_x + 6.0 + (b as f64 * 11.0);
            if bx < px + w - 4.0 {
                out.push_str(&format!(
                    r##"<line x1="{bx:.1}" y1="{py_start:.1}" x2="{bx:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.1" />"##,
                    py_start - 12.0
                ));
            }
        }
    }

    // Mesa de estudo individual de madeira no corredor central
    let desk_w = 48.0;
    let desk_x = vp_x - (desk_w / 2.0);
    let desk_y = floor_y - 24.0;
    out.push_str(&format!(
        r##"<rect x="{desk_x:.1}" y="{desk_y:.1}" width="{desk_w:.1}" height="6.0" fill="#dfd8c8" stroke="#111111" stroke-width="1.6" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        desk_x + 4.0, desk_y + 6.0, desk_x + 4.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="1.8" />"##,
        desk_x + desk_w - 4.0, desk_y + 6.0, desk_x + desk_w - 4.0
    ));
    // Luminária de mesa clássica tipo banqueiro
    out.push_str(&format!(
        r##"<path d="M {:.1} {desk_y:.1} l 0 -12 q 4 -4 10 0" fill="none" stroke="#111111" stroke-width="1.4" />"##,
        desk_x + 12.0
    ));

    // Linhas de fuga do piso de granilite
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));
    out.push_str(&format!(
        r##"<line x1="{vp_x:.1}" y1="{vp_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.0" opacity="0.5" />"##,
        vp_x - 30.0, py + h
    ));
    out.push_str(&format!(
        r##"<line x1="{vp_x:.1}" y1="{vp_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.0" opacity="0.5" />"##,
        vp_x + 30.0, py + h
    ));

    out.push_str("</g>");
    out
}

/// 9. Praça da Cidadania UFSC - A lendária Concha Acústica com gomos parabolicos, canteiros sinuosos de Burle Marx, postes republicanos e silhueta da Reitoria
fn render_praca_cidadania_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-praca-cidadania">"##);

    // Céu claro de Florianópolis
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Silhueta dos morros da ilha e da Reitoria ao longe
    out.push_str(&format!(
        r##"<path d="M {px:.1} {:.1} q 40 -16 90 -8 q 60 -18 110 -4" fill="none" stroke="#222222" stroke-width="1.2" opacity="0.6" />"##,
        ground_y - 65.0
    ));
    // Fachada distante do prédio da Reitoria
    let reit_x = px + (w * 0.55);
    out.push_str(&format!(
        r##"<rect x="{reit_x:.1}" y="{:.1}" width="55.0" height="28.0" fill="#f4f5f7" stroke="#111111" stroke-width="1.2" />"##,
        ground_y - 62.0
    ));

    // A icônica Concha Acústica da UFSC em primeiro plano
    let concha_x = px + 16.0;
    let concha_w = (w * 0.58).min(140.0);
    let concha_h = 75.0;
    let concha_y = ground_y - concha_h;

    // Arco parabólico externo da concha
    out.push_str(&format!(
        r##"<path d="M {concha_x:.1} {ground_y:.1} Q {:.1} {:.1} {:.1} {ground_y:.1} Z" fill="#edeae1" stroke="#111111" stroke-width="2.5" />"##,
        concha_x + (concha_w / 2.0), concha_y - 12.0, concha_x + concha_w
    ));
    // Gomos internos de reflexão acústica em nanquim
    for g in 1..4 {
        let span = concha_w * (1.0 - (g as f64 * 0.22));
        let gx = concha_x + ((concha_w - span) / 2.0);
        let gy = concha_y + (g as f64 * 12.0);
        out.push_str(&format!(
            r##"<path d="M {gx:.1} {ground_y:.1} Q {:.1} {gy:.1} {:.1} {ground_y:.1}" fill="none" stroke="#111111" stroke-width="1.4" />"##,
            gx + (span / 2.0), gx + span
        ));
        // Hachura sutil da concavidade da concha
        ink_hatch_lines(
            &mut out,
            gx + 4.0,
            gy + 4.0,
            gx + span - 4.0,
            gy + 12.0,
            5.0,
        );
    }
    // Palco elevado da concha acústica com degraus em meia-lua
    out.push_str(&format!(
        r##"<rect x="{concha_x:.1}" y="{:.1}" width="{concha_w:.1}" height="8.0" fill="#ded9cb" stroke="#111111" stroke-width="1.8" />"##,
        ground_y - 8.0
    ));

    // Canteiros paisagísticos de linhas orgânicas (Burle Marx) à direita
    let cant_x = concha_x + concha_w + 12.0;
    out.push_str(&format!(
        r##"<path d="M {cant_x:.1} {ground_y:.1} q 18 -18 36 -6 q 20 12 34 6" fill="none" stroke="#111111" stroke-width="1.6" />"##
    ));
    // Árvore de ipê com copa expressiva em bico de pena
    let tree_x = cant_x + 22.0;
    out.push_str(&format!(
        r##"<path d="M {tree_x:.1} {ground_y:.1} l 0 -36 q -14 -12 -12 -24 q 16 2 24 18 q 16 -12 24 6 q -8 20 -28 14" fill="#f5f7f4" stroke="#111111" stroke-width="1.8" />"##
    ));

    // Poste de iluminação republicano clássico
    let post_x = px + w - 18.0;
    out.push_str(&format!(
        r##"<line x1="{post_x:.1}" y1="{ground_y:.1}" x2="{post_x:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.0" />"##,
        ground_y - 52.0
    ));
    out.push_str(&format!(
        r##"<circle cx="{post_x:.1}" cy="{:.1}" r="4.0" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
        ground_y - 52.0
    ));

    // Calçamento da Praça da Cidadania
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 10. CTC UFSC - Passarela suspensa de concreto armado conectando os blocos da engenharia, vigas estruturais, tubulações industriais e bancada de hardware
fn render_ctc_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-ctc">"##);

    // Fundo industrial/brutalista
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafafc" />"##
    ));

    // Blocos de concreto da engenharia no plano de fundo
    let bldg_y = py + 22.0;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{bldg_y:.1}" width="{:.1}" height="95.0" fill="#f0f1f4" stroke="#111111" stroke-width="2.0" />"##,
        w * 0.45
    ));
    for j in 0..3 {
        let jy = bldg_y + 12.0 + (j as f64 * 26.0);
        out.push_str(&format!(
            r##"<rect x="{:.1}" y="{jy:.1}" width="24.0" height="14.0" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
            px + 12.0
        ));
    }

    // A icônica passarela suspensa de concreto armado que liga os blocos do CTC
    let pass_y = py + 34.0;
    let pass_h = 24.0;
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{pass_y:.1}" width="{w:.1}" height="{pass_h:.1}" fill="#e6e8ec" stroke="#111111" stroke-width="2.4" />"##
    ));
    // Guarda-corpos e aberturas rítmicas da passarela
    for p in 0..9 {
        let px_win = px + 8.0 + (p as f64 * (w / 8.5));
        if px_win + 14.0 < px + w {
            out.push_str(&format!(
                r##"<rect x="{px_win:.1}" y="{:.1}" width="14.0" height="14.0" fill="#ffffff" stroke="#111111" stroke-width="1.3" />"##,
                pass_y + 5.0
            ));
        }
    }
    // Vigas I e pilares chanfrados sustentando a passarela
    let pil_x = px + (w * 0.65).min(w - 45.0);
    out.push_str(&format!(
        r##"<polygon points="{pil_x:.1},{:.1} {:.1},{:.1} {:.1},{ground_y:.1} {pil_x:.1},{ground_y:.1}" fill="#dadde3" stroke="#111111" stroke-width="2.2" />"##,
        pass_y + pass_h, pil_x + 24.0, pass_y + pass_h, pil_x + 18.0
    ));
    ink_hatch_lines(
        &mut out,
        pil_x + 2.0,
        pass_y + pass_h + 4.0,
        pil_x + 20.0,
        ground_y - 4.0,
        8.0,
    );

    // Eletrocalhas e tubulações industriais correndo pela parede
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="2.2" />"##,
        pass_y - 6.0, px + w, pass_y - 6.0
    ));

    // Bicicletário do CTC com bicicletas universitárias em nanquim
    let bike_x = px + 18.0;
    let bike_y = ground_y - 12.0;
    out.push_str(&format!(
        r##"<circle cx="{bike_x:.1}" cy="{bike_y:.1}" r="7.0" fill="none" stroke="#111111" stroke-width="1.3" />"##
    ));
    out.push_str(&format!(
        r##"<circle cx="{:.1}" cy="{bike_y:.1}" r="7.0" fill="none" stroke="#111111" stroke-width="1.3" />"##,
        bike_x + 20.0
    ));
    out.push_str(&format!(
        r##"<path d="M {bike_x:.1} {bike_y:.1} l 10 -10 l 10 10 l -10 0 l -4 -10" fill="none" stroke="#111111" stroke-width="1.4" />"##
    ));

    // Calçamento intertravado do CTC
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 11. CFH UFSC - O clássico "Aquário" envidraçado do CFH, murais estudantis, escadaria de convivência e eucaliptos do bosque
fn render_cfh_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-cfh">"##);

    // Fundo do campus
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#faf9f6" />"##
    ));

    // Troncos altos dos eucaliptos do Bosque do CFH à esquerda
    for i in 0..3 {
        let ex = px + 8.0 + (i as f64 * 14.0);
        out.push_str(&format!(
            r##"<line x1="{ex:.1}" y1="{py:.1}" x2="{ex:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="3.2" />"##
        ));
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{py:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="0.8" opacity="0.6" stroke-dasharray="8,4" />"##,
            ex + 1.2, ex + 1.2
        ));
    }

    // Fachada do Bloco do CFH com "O Aquário" (hall envidraçado piso ao teto)
    let aqua_x = px + 52.0;
    let aqua_y = py + 26.0;
    let aqua_w = (px + w - aqua_x - 10.0).max(85.0);
    let aqua_h = ground_y - aqua_y;

    out.push_str(&format!(
        r##"<rect x="{aqua_x:.1}" y="{aqua_y:.1}" width="{aqua_w:.1}" height="{aqua_h:.1}" fill="#f2f5f4" stroke="#111111" stroke-width="2.2" />"##
    ));
    // Montantes de ferro do Aquário
    for m in 1..4 {
        let mx = aqua_x + (m as f64 * (aqua_w / 4.0));
        out.push_str(&format!(
            r##"<line x1="{mx:.1}" y1="{aqua_y:.1}" x2="{mx:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="1.6" />"##
        ));
    }
    // Cartazes estudantis e informativos colados no vidro do Aquário
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="22.0" height="28.0" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
        aqua_x + 12.0, aqua_y + 18.0
    ));
    ink_hatch_lines(
        &mut out,
        aqua_x + 14.0,
        aqua_y + 22.0,
        aqua_x + 32.0,
        aqua_y + 42.0,
        4.0,
    );

    // Escadaria externa de concreto onde ocorrem assembleias e rodas de conversa
    let esc_y = ground_y - 18.0;
    for s in 0..3 {
        let sy = esc_y + (s as f64 * 6.0);
        let sx = aqua_x + 6.0 - (s as f64 * 8.0);
        out.push_str(&format!(
            r##"<line x1="{sx:.1}" y1="{sy:.1}" x2="{:.1}" y2="{sy:.1}" stroke="#111111" stroke-width="1.8" />"##,
            aqua_x + aqua_w
        ));
    }

    // Linha do chão
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 12. Pantanal UFSC - Córrego sinuoso da Trindade, vegetação de taboas, margem de seixos e os morros da ilha em densas hachuras
fn render_pantanal_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-pantanal">"##);

    // Céu sobre a bacia do Itacorubi/Pantanal
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Morro da Cruz e Maciço da Costeira ao fundo com densas tramas de hachuras
    out.push_str(&format!(
        r##"<path d="M {px:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} T {:.1} {:.1}" fill="#f1f5f2" stroke="#111111" stroke-width="1.8" />"##,
        ground_y - 45.0, px + (w * 0.35), ground_y - 85.0, px + (w * 0.7), ground_y - 60.0, px + w, ground_y - 75.0
    ));
    ink_hatch_lines(
        &mut out,
        px + 10.0,
        ground_y - 75.0,
        px + w - 10.0,
        ground_y - 50.0,
        6.0,
    );

    // Córrego do manguezal com águas calmas e reflexos
    let water_y = ground_y - 14.0;
    out.push_str(&format!(
        r##"<path d="M {px:.1} {ground_y:.1} q 40 -12 90 -4 q 50 8 110 -6" fill="none" stroke="#111111" stroke-width="1.6" />"##
    ));
    for k in 0..4 {
        let rx = px + 15.0 + (k as f64 * 42.0);
        out.push_str(&format!(
            r##"<line x1="{rx:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.6" />"##,
            water_y + (k as f64 * 3.0), rx + 24.0, water_y + (k as f64 * 3.0)
        ));
    }

    // Touceiras altas de taboas (Typha) e capim nativo na margem
    let grass_x = px + (w * 0.22).min(55.0);
    for t in 0..4 {
        let gx = grass_x + (t as f64 * 8.0);
        out.push_str(&format!(
            r##"<path d="M {gx:.1} {ground_y:.1} q -4 -28 2 -48 M {gx:.1} {ground_y:.1} q 6 -24 10 -42" fill="none" stroke="#222222" stroke-width="1.2" />"##
        ));
        // Espiga cilíndrica de taboa na ponta
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="3.2" stroke-linecap="round" />"##,
            gx + 2.0, ground_y - 48.0, gx + 2.0, ground_y - 36.0
        ));
    }

    // Tronco caído na margem (onde descansam capivaras e jacarés)
    let log_x = px + w - 75.0;
    out.push_str(&format!(
        r##"<path d="M {log_x:.1} {ground_y:.1} q 25 -8 55 -4 l -2 6 q -28 -2 -53 4 z" fill="#eae5d8" stroke="#111111" stroke-width="1.6" />"##
    ));

    // Linha de solo da margem
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 13. Pastoral Comic Chat - Homenagem à obra de Jim Woodring: colinas suaves hachuradas, cerca rústica de ripas tortas de madeira e árvore retorcida
fn render_pastoral_vector_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-pastoral">"##);

    // Céu límpido de quadrinhos
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Nuvens concêntricas orgânicas características de Jim Woodring
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} q 18 -16 38 -6 q 16 -18 36 -4 q 18 -14 36 2" fill="none" stroke="#222222" stroke-width="1.3" opacity="0.75" />"##,
        px + 20.0, py + 22.0
    ));

    // Colinas onduladas com linhas de nível e hachura topográfica
    out.push_str(&format!(
        r##"<path d="M {px:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} T {:.1} {ground_y:.1}" fill="#f6f8f5" stroke="#111111" stroke-width="2.0" />"##,
        ground_y - 38.0, px + (w * 0.3), ground_y - 68.0, px + (w * 0.65), ground_y - 42.0, px + w
    ));
    ink_hatch_lines(
        &mut out,
        px + 20.0,
        ground_y - 58.0,
        px + w - 20.0,
        ground_y - 36.0,
        7.0,
    );

    // A icônica cerca de madeira de mourões tortos e lascados do Comic Chat
    for m in 0..5 {
        let mx = px + 16.0 + (m as f64 * (w / 4.4));
        let my = ground_y - 28.0 + if m % 2 == 0 { -4.0 } else { 3.0 };
        // Mourão de cerca com topo chanfrado
        out.push_str(&format!(
            r##"<path d="M {mx:.1} {ground_y:.1} l -2 {:.1} l 6 -4 l 4 {:.1} z" fill="#f0ede4" stroke="#111111" stroke-width="1.6" />"##,
            my - ground_y, ground_y - my + 4.0
        ));
    }
    // Ripas horizontais de madeira pregadas com pregos tortos
    out.push_str(&format!(
        r##"<path d="M {px:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#111111" stroke-width="2.2" />"##,
        ground_y - 18.0, px + (w * 0.5), ground_y - 24.0, px + w, ground_y - 16.0
    ));
    out.push_str(&format!(
        r##"<path d="M {px:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#111111" stroke-width="2.0" />"##,
        ground_y - 8.0, px + (w * 0.5), ground_y - 14.0, px + w, ground_y - 6.0
    ));

    // Árvore retorcida e expressiva no plano médio à direita
    let tree_x = px + w - 48.0;
    out.push_str(&format!(
        r##"<path d="M {tree_x:.1} {ground_y:.1} q -8 -22 4 -44 q -18 -8 -16 -24 q 22 -6 32 10 q 18 -10 24 14 q -12 22 -24 44 z" fill="#f4f6f2" stroke="#111111" stroke-width="2.0" />"##
    ));
    // Nós e fendas no tronco
    out.push_str(&format!(
        r##"<ellipse cx="{:.1}" cy="{:.1}" rx="2.5" ry="5.0" fill="#111111" stroke="#111111" stroke-width="1.0" />"##,
        tree_x + 6.0, ground_y - 22.0
    ));

    // Linha do solo com tufos de grama
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.2" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 14. Room - Estúdio de quadrinhos com prancheta inclinada, régua paralela, luminária pantográfica, frascos de nanquim e estante de zines
fn render_room_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-room">"##);

    // Fundo do estúdio
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#faf8f5" />"##
    ));

    // Janela com batente de madeira e cortina amarrada à esquerda
    let win_x = px + 12.0;
    let win_y = py + 18.0;
    let win_w = (w * 0.28).min(65.0);
    let win_h = 88.0;
    out.push_str(&format!(
        r##"<rect x="{win_x:.1}" y="{win_y:.1}" width="{win_w:.1}" height="{win_h:.1}" fill="#f2f4f8" stroke="#111111" stroke-width="2.0" />"##
    ));
    // Lua crescente no céu visto pela janela
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} a 6 6 0 1 0 8 8 a 8 8 0 0 1 -8 -8 z" fill="#111111" stroke="#111111" stroke-width="0.8" />"##,
        win_x + win_w - 18.0, win_y + 12.0
    ));
    // Cortina recolhida com laço
    out.push_str(&format!(
        r##"<path d="M {win_x:.1} {win_y:.1} q 16 20 8 45 q 14 20 4 43" fill="none" stroke="#111111" stroke-width="1.6" />"##
    ));

    // Prancheta de desenho inclinada de cartunista
    let pranc_x = win_x + win_w + 14.0;
    let pranc_y = py + 52.0;
    let pranc_w = (px + w - pranc_x - 14.0).max(85.0);

    // Tampo inclinado com folha de tira de quadrinhos presa com fita crepe
    out.push_str(&format!(
        r##"<polygon points="{pranc_x:.1},{pranc_y:.1} {:.1},{:.1} {:.1},{floor_y:.1} {pranc_x:.1},{floor_y:.1}" fill="#ede8dc" stroke="#111111" stroke-width="2.2" />"##,
        pranc_x + pranc_w, pranc_y - 12.0, pranc_x + pranc_w
    ));
    // Tira de quadrinhos na prancheta com quadros divididos
    let sheet_x = pranc_x + 12.0;
    let sheet_y = pranc_y + 8.0;
    out.push_str(&format!(
        r##"<rect x="{sheet_x:.1}" y="{sheet_y:.1}" width="42.0" height="26.0" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{sheet_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.9" />"##,
        sheet_x + 14.0, sheet_x + 14.0, sheet_y + 26.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{sheet_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.9" />"##,
        sheet_x + 28.0, sheet_x + 28.0, sheet_y + 26.0
    ));

    // Pote de nanquim e canetas bico de pena sobre a mesa
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="8.0" height="10.0" fill="#111111" stroke="#111111" stroke-width="1.0" rx="1" />"##,
        sheet_x + 48.0, sheet_y + 12.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.5" />"##,
        sheet_x + 52.0, sheet_y + 12.0, sheet_x + 58.0, sheet_y - 6.0
    ));

    // Luminária pantográfica articulada de prancheta
    let lamp_x = pranc_x + 4.0;
    let lamp_y = pranc_y - 2.0;
    out.push_str(&format!(
        r##"<line x1="{lamp_x:.1}" y1="{lamp_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.8" />"##,
        lamp_x + 10.0, lamp_y - 24.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.8" />"##,
        lamp_x + 10.0, lamp_y - 24.0, lamp_x + 26.0, lamp_y - 18.0
    ));
    // Cúpula cônica da lâmpada
    out.push_str(&format!(
        r##"<path d="M {:.1} {:.1} l 8 4 l -12 6 z" fill="#e2ded4" stroke="#111111" stroke-width="1.4" />"##,
        lamp_x + 26.0, lamp_y - 18.0
    ));

    // Tábuas corridas do chão com pregos aparentes
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 15. Bosque do CFH - Eucaliptos imponentes com hachuras de casca, luz filtrada, bancos de concreto e terra batida
fn render_bosque_cfh_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let floor_y = py + h - 25.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-bosque-cfh">"##);

    // Sky tint & soft canopy back
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#f4f7f4" />"##
    ));

    // Distant tree trunks and dense foliage in background
    for i in 0..6 {
        let tx = px + 15.0 + (i as f64 * (w / 5.5));
        let tw = 7.0 + ((i % 3) as f64 * 3.0);
        out.push_str(&format!(
            r##"<rect x="{tx:.1}" y="{py:.1}" width="{tw:.1}" height="{:.1}" fill="#e6eae6" stroke="#111111" stroke-width="1.2" />"##,
            floor_y - py
        ));
        for k in 0..4 {
            let hy = py + 20.0 + (k as f64 * 35.0);
            out.push_str(&format!(
                r##"<line x1="{tx:.1}" y1="{hy:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.6" />"##,
                tx + tw, hy + 8.0
            ));
        }
    }

    // Organic canopy clouds
    out.push_str(&format!(
        r##"<path d="M {px:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#222222" stroke-width="1.8" />"##,
        py + 25.0,
        px + (w * 0.15), py + 5.0, px + (w * 0.35), py + 22.0,
        px + (w * 0.5), py + 8.0, px + (w * 0.7), py + 24.0,
        px + (w * 0.85), py + 6.0, px + w, py + 20.0
    ));

    // Sun rays filtered through branches (hatch lines)
    for r in 0..5 {
        let rx1 = px + 30.0 + (r as f64 * 40.0);
        let rx2 = rx1 - 25.0;
        out.push_str(&format!(
            r##"<line x1="{rx1:.1}" y1="{py:.1}" x2="{rx2:.1}" y2="{floor_y:.1}" stroke="#ffffff" stroke-width="1.2" stroke-dasharray="4,4" opacity="0.45" />"##
        ));
    }

    // Concrete park bench under the eucalyptus
    let bench_x = px + (w * 0.55).min(w - 75.0);
    let bench_y = floor_y - 18.0;
    out.push_str(&format!(
        r##"<rect x="{bench_x:.1}" y="{bench_y:.1}" width="58.0" height="7.0" rx="1.5" fill="#dededc" stroke="#111111" stroke-width="1.6" />"##
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="6.0" height="11.0" fill="#dededc" stroke="#111111" stroke-width="1.4" />"##,
        bench_x + 8.0, bench_y + 7.0
    ));
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{:.1}" width="6.0" height="11.0" fill="#dededc" stroke="#111111" stroke-width="1.4" />"##,
        bench_x + 44.0, bench_y + 7.0
    ));

    // Ground line & fallen leaf hatching
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{floor_y:.1}" x2="{:.1}" y2="{floor_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));
    for f in 0..8 {
        let lx = px + 10.0 + (f as f64 * (w / 8.5));
        out.push_str(&format!(
            r##"<line x1="{lx:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.0" opacity="0.5" />"##,
            floor_y + 3.0, lx + 6.0, floor_y + 7.0
        ));
    }

    out.push_str("</g>");
    out
}

/// 16. Reitoria UFSC - Prédio Central com esquadrias imponentes, marquise, mastro cívico e piso cimentado
fn render_reitoria_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-reitoria">"##);

    // Sky tint
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Main facade block
    let bldg_x = px + 14.0;
    let bldg_y = py + 20.0;
    let bldg_w = w - 28.0;
    let bldg_h = 80.0;
    out.push_str(&format!(
        r##"<rect x="{bldg_x:.1}" y="{bldg_y:.1}" width="{bldg_w:.1}" height="{bldg_h:.1}" fill="#f6f8fa" stroke="#111111" stroke-width="2.2" />"##
    ));

    // Rhythmic architectural windows (grid of rectangular glass panes)
    for row in 0..3 {
        let wy = bldg_y + 10.0 + (row as f64 * 18.0);
        for col in 0..6 {
            let wx = bldg_x + 8.0 + (col as f64 * ((bldg_w - 16.0) / 5.5));
            out.push_str(&format!(
                r##"<rect x="{wx:.1}" y="{wy:.1}" width="14.0" height="11.0" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##
            ));
            out.push_str(&format!(
                r##"<line x1="{wx:.1}" y1="{wy:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.6" opacity="0.4" />"##,
                wx + 14.0, wy + 11.0
            ));
        }
    }

    // Extended grand canopy / entrance portico
    let portico_x = bldg_x + (bldg_w * 0.32);
    let portico_y = bldg_y + bldg_h - 10.0;
    let portico_w = (bldg_w * 0.36).min(70.0);
    out.push_str(&format!(
        r##"<rect x="{portico_x:.1}" y="{portico_y:.1}" width="{portico_w:.1}" height="6.0" fill="#e1e4e8" stroke="#111111" stroke-width="2.0" />"##
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        portico_x + 4.0, portico_y + 6.0, portico_x + 4.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        portico_x + portico_w - 4.0, portico_y + 6.0, portico_x + portico_w - 4.0
    ));

    // Civic flagpole on the left side
    let pole_x = bldg_x - 4.0;
    out.push_str(&format!(
        r##"<line x1="{pole_x:.1}" y1="{py:.1}" x2="{pole_x:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="1.8" />"##
    ));
    out.push_str(&format!(
        r##"<path d="M {pole_x:.1} {:.1} l 14 4 l -14 4 z" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
        py + 8.0
    ));

    // Ground and stone pavers
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));
    for p in 0..5 {
        let paver_x = px + 10.0 + (p as f64 * (w / 4.5));
        out.push_str(&format!(
            r##"<line x1="{paver_x:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.8" opacity="0.5" />"##,
            paver_x - 12.0, py + h
        ));
    }

    out.push_str("</g>");
    out
}

/// 17. Pista de Atletismo CDS - Curvas de pista tartan hachuradas, postes de holofotes e arquibancadas
fn render_pista_atletismo_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let horizon_y = py + (h * 0.42);
    out.push_str(r##"<g class="backdrop-vector backdrop-pista-atletismo">"##);

    // Open sky
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#fafbfc" />"##
    ));

    // Background bleachers / arquibancada com degraus em perspectiva
    let bleacher_x = px + 15.0;
    let bleacher_w = (w * 0.55).min(110.0);
    out.push_str(&format!(
        r##"<polygon points="{bleacher_x:.1},{horizon_y:.1} {:.1},{horizon_y:.1} {:.1},{:.1} {bleacher_x:.1},{:.1}" fill="#f0f2f5" stroke="#111111" stroke-width="1.8" />"##,
        bleacher_x + bleacher_w, bleacher_x + bleacher_w - 20.0, horizon_y - 25.0, horizon_y - 25.0
    ));
    for s in 1..4 {
        let sy = horizon_y - (s as f64 * 6.0);
        out.push_str(&format!(
            r##"<line x1="{bleacher_x:.1}" y1="{sy:.1}" x2="{:.1}" y2="{sy:.1}" stroke="#111111" stroke-width="1.0" />"##,
            bleacher_x + bleacher_w - (s as f64 * 5.0)
        ));
    }

    // High floodlight tower on the right
    let flood_x = px + w - 30.0;
    out.push_str(&format!(
        r##"<line x1="{flood_x:.1}" y1="{py:.1}" x2="{:.1}" y2="{horizon_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        flood_x - 5.0
    ));
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{py:.1}" x2="{:.1}" y2="{horizon_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        flood_x + 10.0, flood_x + 5.0
    ));
    // Truss crossbars on floodlight
    for b in 0..4 {
        let by = py + 6.0 + (b as f64 * 12.0);
        out.push_str(&format!(
            r##"<line x1="{flood_x:.1}" y1="{by:.1}" x2="{:.1}" y2="{by:.1}" stroke="#111111" stroke-width="1.0" />"##,
            flood_x + 10.0
        ));
    }

    // Curving running track lanes (linhas curvas de nanquim da pista de atletismo)
    let track_base_y = py + h - 18.0;
    for lane in 0..5 {
        let offset = lane as f64 * 8.0;
        out.push_str(&format!(
            r##"<path d="M {px:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}" fill="none" stroke="#111111" stroke-width="{:.1}" />"##,
            horizon_y + 12.0 + offset,
            px + (w * 0.4), horizon_y + 24.0 + (offset * 1.4),
            px + w, track_base_y - (lane as f64 * 6.0),
            if lane == 0 { 2.0 } else { 1.4 }
        ));
    }

    // Ground baseline
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{track_base_y:.1}" x2="{:.1}" y2="{track_base_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));

    out.push_str("</g>");
    out
}

/// 18. Centro de Cultura e Eventos UFSC - Linhas curvas modernistas, envidraçado amplo e esplanada da Praça da Cidadania
fn render_centro_eventos_svg(px: f64, py: f64, w: f64, h: f64) -> String {
    let mut out = String::new();
    let ground_y = py + h - 22.0;
    out.push_str(r##"<g class="backdrop-vector backdrop-centro-eventos">"##);

    // Clean sky
    out.push_str(&format!(
        r##"<rect x="{px:.1}" y="{py:.1}" width="{w:.1}" height="{h:.1}" fill="#f8fafc" />"##
    ));

    // Monumental modern curved facade of Centro de Eventos
    let evt_x = px + 12.0;
    let evt_y = py + 22.0;
    let evt_w = w - 24.0;
    let _evt_h = 76.0;

    // Curved sweeping roofline
    out.push_str(&format!(
        r##"<path d="M {evt_x:.1} {:.1} Q {:.1} {evt_y:.1} {:.1} {:.1} L {:.1} {ground_y:.1} L {evt_x:.1} {ground_y:.1} Z" fill="#f1f3f6" stroke="#111111" stroke-width="2.4" />"##,
        evt_y + 14.0, evt_x + (evt_w * 0.45), evt_x + evt_w, evt_y + 8.0, evt_x + evt_w
    ));

    // Glass curtain wall with diagonal ink reflections
    let glass_y = evt_y + 24.0;
    let glass_h = 36.0;
    out.push_str(&format!(
        r##"<rect x="{:.1}" y="{glass_y:.1}" width="{:.1}" height="{glass_h:.1}" fill="#ffffff" stroke="#111111" stroke-width="1.8" />"##,
        evt_x + 10.0, evt_w - 20.0
    ));
    for g in 1..6 {
        let gx = evt_x + 10.0 + (g as f64 * ((evt_w - 20.0) / 6.0));
        out.push_str(&format!(
            r##"<line x1="{gx:.1}" y1="{glass_y:.1}" x2="{gx:.1}" y2="{:.1}" stroke="#111111" stroke-width="1.2" />"##,
            glass_y + glass_h
        ));
        // Hatch reflection line
        out.push_str(&format!(
            r##"<line x1="{gx:.1}" y1="{glass_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.7" opacity="0.4" />"##,
            gx - 12.0, glass_y + glass_h
        ));
    }

    // Plaza esplanade pavers in perspective
    out.push_str(&format!(
        r##"<line x1="{px:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{ground_y:.1}" stroke="#111111" stroke-width="2.0" />"##,
        px + w
    ));
    for paver in 0..6 {
        let px1 = px + 20.0 + (paver as f64 * (w / 5.2));
        out.push_str(&format!(
            r##"<line x1="{px1:.1}" y1="{ground_y:.1}" x2="{:.1}" y2="{:.1}" stroke="#111111" stroke-width="0.9" opacity="0.45" />"##,
            px1 - 20.0, py + h
        ));
    }

    out.push_str("</g>");
    out
}
