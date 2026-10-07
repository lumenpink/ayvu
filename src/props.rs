// Authentic comic props rendered in clean hand-inked nanquim style
use crate::model::{PropItem, PropType};

/// Render a prop item into SVG string positioned at (cx, cy)
pub fn render_prop_svg(prop: &PropItem, cx: f64, cy: f64, scale: f64, flipped: bool) -> String {
    let mut out = String::new();
    let flip_transform = if flipped {
        format!(r#" transform="translate({cx:.1}, {cy:.1}) scale(-{scale:.2}, {scale:.2})""#)
    } else {
        format!(r#" transform="translate({cx:.1}, {cy:.1}) scale({scale:.2}, {scale:.2})""#)
    };

    out.push_str(&format!(
        r##"<g class="comic-prop prop-{}"{}>"##,
        prop.prop_type.as_str(),
        flip_transform
    ));

    match prop.prop_type {
        PropType::Coffee => {
            out.push_str(&render_coffee_svg());
        }
        PropType::Book => {
            out.push_str(&render_book_svg());
        }
        PropType::Laptop => {
            out.push_str(&render_laptop_svg());
        }
        PropType::Erlenmeyer => {
            out.push_str(&render_erlenmeyer_svg());
        }
        PropType::Chalk => {
            out.push_str(&render_chalk_svg());
        }
        PropType::Stethoscope => {
            out.push_str(&render_stethoscope_svg());
        }
        PropType::Microphone => {
            out.push_str(&render_microphone_svg());
        }
        PropType::Tray => {
            out.push_str(&render_tray_svg());
        }
        PropType::Backpack => {
            out.push_str(&render_backpack_svg());
        }
        PropType::Phone => {
            out.push_str(&render_phone_svg());
        }
        PropType::Umbrella => {
            out.push_str(&render_umbrella_svg());
        }
        PropType::Caderno => {
            out.push_str(&render_caderno_svg());
        }
        PropType::Custom(_) => {
            out.push_str(&render_coffee_svg());
        }
    }

    out.push_str("</g>");
    out
}

/// Coffee / Tea Mug with saucer & rising steam curls
fn render_coffee_svg() -> String {
    let mut s = String::new();
    // Saucer
    s.push_str(r##"<ellipse cx="0" cy="10" rx="16" ry="4.5" fill="#f8f9fa" stroke="#111111" stroke-width="1.8" />"##);
    // Saucer inner ring
    s.push_str(r##"<ellipse cx="0" cy="9.5" rx="10" ry="2.5" fill="none" stroke="#666666" stroke-width="1" />"##);
    // Cup Body
    s.push_str(r##"<path d="M -10 -4 L -9 8 Q 0 11 9 8 L 10 -4 Z" fill="#ffffff" stroke="#111111" stroke-width="2" stroke-linejoin="round" />"##);
    // Cup Rim
    s.push_str(r##"<ellipse cx="0" cy="-4" rx="10" ry="3.5" fill="#3e2723" stroke="#111111" stroke-width="1.6" />"##);
    // Coffee liquid reflection
    s.push_str(r##"<ellipse cx="-2" cy="-4" rx="6" ry="1.8" fill="#5d4037" opacity="0.85" />"##);
    // Handle on the right
    s.push_str(r##"<path d="M 8 -2 Q 16 0 14 6 Q 12 8 7 6" fill="none" stroke="#111111" stroke-width="2.2" stroke-linecap="round" />"##);
    // Rising steam curls
    s.push_str(r##"<path d="M -4 -8 Q -7 -14 -3 -19 Q 1 -24 -2 -28" fill="none" stroke="#555555" stroke-width="1.4" stroke-linecap="round" opacity="0.8" />"##);
    s.push_str(r##"<path d="M 3 -7 Q 7 -13 4 -18 Q 1 -23 4 -27" fill="none" stroke="#555555" stroke-width="1.4" stroke-linecap="round" opacity="0.8" />"##);
    s
}

/// Open Book / Academic Textbook with bookmarked ribbon and text lines
fn render_book_svg() -> String {
    let mut s = String::new();
    // Book cover base
    s.push_str(r##"<path d="M -22 6 Q 0 10 22 6 L 24 10 Q 0 14 -24 10 Z" fill="#1b1f23" stroke="#111111" stroke-width="1.5" />"##);
    // Left Page
    s.push_str(r##"<path d="M 0 4 Q -10 -1 -22 2 L -22 -12 Q -10 -15 0 -10 Z" fill="#fdfbf7" stroke="#111111" stroke-width="1.8" stroke-linejoin="round" />"##);
    // Right Page
    s.push_str(r##"<path d="M 0 4 Q 10 -1 22 2 L 22 -12 Q 10 -15 0 -10 Z" fill="#fcfcfc" stroke="#111111" stroke-width="1.8" stroke-linejoin="round" />"##);
    // Center spine fold line
    s.push_str(r##"<line x1="0" y1="-10" x2="0" y2="4" stroke="#111111" stroke-width="1.5" />"##);
    // Text lines on left page
    s.push_str(r##"<line x1="-18" y1="-9" x2="-4" y2="-7" stroke="#444444" stroke-width="1" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="-18" y1="-5" x2="-5" y2="-3" stroke="#444444" stroke-width="1" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="-17" y1="-1" x2="-6" y2="1" stroke="#444444" stroke-width="1" stroke-linecap="round" />"##);
    // Text lines on right page
    s.push_str(r##"<line x1="4" y1="-7" x2="18" y2="-9" stroke="#444444" stroke-width="1" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="5" y1="-3" x2="18" y2="-5" stroke="#444444" stroke-width="1" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="6" y1="1" x2="17" y2="-1" stroke="#444444" stroke-width="1" stroke-linecap="round" />"##);
    // Red bookmark ribbon hanging below
    s.push_str(r##"<path d="M 0 4 Q -2 11 2 16 L 0 18 L 4 17" fill="none" stroke="#d90429" stroke-width="1.8" stroke-linecap="round" />"##);
    s
}

/// Laptop in perspective with glowing screen and keyboard
fn render_laptop_svg() -> String {
    let mut s = String::new();
    // Screen casing (tilted back)
    s.push_str(r##"<polygon points="-18,-18 18,-18 15,2 -15,2" fill="#24292e" stroke="#111111" stroke-width="1.8" stroke-linejoin="round" />"##);
    // Inner glowing display
    s.push_str(r##"<polygon points="-16,-16 16,-16 13,0 -13,0" fill="#f0f6fc" stroke="#111111" stroke-width="1" />"##);
    // Code / window text on screen
    s.push_str(r##"<line x1="-13" y1="-12" x2="-2" y2="-12" stroke="#0969da" stroke-width="1.2" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="-13" y1="-8" x2="4" y2="-8" stroke="#1f2328" stroke-width="1.2" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="-13" y1="-4" x2="9" y2="-4" stroke="#2da44e" stroke-width="1.2" stroke-linecap="round" />"##);
    // Base / keyboard deck in isometric projection
    s.push_str(r##"<polygon points="-15,2 15,2 22,12 -22,12" fill="#e1e4e8" stroke="#111111" stroke-width="2" stroke-linejoin="round" />"##);
    // Keyboard trackpad area
    s.push_str(r##"<polygon points="-13,3 13,3 18,9 -18,9" fill="#d0d7de" stroke="#24292e" stroke-width="0.8" />"##);
    s.push_str(r##"<rect x="-4" y="9.5" width="8" height="2" rx="0.5" fill="#ffffff" stroke="#24292e" stroke-width="0.8" />"##);
    // Front edge thickness
    s.push_str(r##"<polygon points="-22,12 22,12 21,14 -21,14" fill="#24292e" stroke="#111111" stroke-width="1" />"##);
    s
}

/// Scientific Erlenmeyer Flask with liquid, bubbles and volume gradations
fn render_erlenmeyer_svg() -> String {
    let mut s = String::new();
    // Outer glass flask body
    s.push_str(r##"<path d="M -4 -16 L 4 -16 L 4 -8 L 16 12 Q 17 15 13 15 L -13 15 Q -17 15 -16 12 L -4 -8 Z" fill="#ffffff" stroke="#111111" stroke-width="2" stroke-linejoin="round" />"##);
    // Top lip
    s.push_str(r##"<ellipse cx="0" cy="-16" rx="5" ry="1.5" fill="#f6f8fa" stroke="#111111" stroke-width="1.8" />"##);
    // Liquid inside (cyan/blue chemical reagent)
    s.push_str(r##"<path d="M -9 2 Q 0 4 9 2 L 14.5 12 Q 15 14 12 14 L -12 14 Q -15 14 -14.5 12 Z" fill="#0284c7" stroke="#0369a1" stroke-width="1" />"##);
    // Meniscus reflection
    s.push_str(r##"<ellipse cx="0" cy="2" rx="9" ry="2" fill="#38bdf8" opacity="0.8" />"##);
    // Graduation tick marks
    s.push_str(r##"<line x1="-3" y1="-2" x2="1" y2="-2" stroke="#111111" stroke-width="1" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="-5" y1="3" x2="0" y2="3" stroke="#ffffff" stroke-width="1" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="-7" y1="8" x2="-1" y2="8" stroke="#ffffff" stroke-width="1" stroke-linecap="round" />"##);
    // Bubbles
    s.push_str(
        r##"<circle cx="-4" cy="7" r="1.5" fill="#ffffff" stroke="#0284c7" stroke-width="0.5" />"##,
    );
    s.push_str(
        r##"<circle cx="3" cy="9" r="1.8" fill="#ffffff" stroke="#0284c7" stroke-width="0.5" />"##,
    );
    s.push_str(
        r##"<circle cx="1" cy="4" r="1.2" fill="#ffffff" stroke="#0284c7" stroke-width="0.5" />"##,
    );
    s
}

/// Chalk stick with bevel tip and chalk dust marks
fn render_chalk_svg() -> String {
    let mut s = String::new();
    // Chalk stick
    s.push_str(r##"<path d="M -12 -4 L 8 -4 L 14 -1 L 14 2 L 8 4 L -12 4 Z" fill="#ffffff" stroke="#111111" stroke-width="1.8" stroke-linejoin="round" />"##);
    // Chalk tip bevel
    s.push_str(r##"<polygon points="8,-4 14,-1 14,2 8,4" fill="#e1e4e8" stroke="#111111" stroke-width="1.2" />"##);
    // Chalk dust particles around
    s.push_str(
        r##"<circle cx="17" cy="-2" r="1" fill="#ffffff" stroke="#111111" stroke-width="0.6" />"##,
    );
    s.push_str(
        r##"<circle cx="19" cy="3" r="0.8" fill="#ffffff" stroke="#111111" stroke-width="0.6" />"##,
    );
    s.push_str(
        r##"<circle cx="15" cy="5" r="0.9" fill="#ffffff" stroke="#111111" stroke-width="0.6" />"##,
    );
    s
}

/// Medical Stethoscope (binaural arch & chest disc)
fn render_stethoscope_svg() -> String {
    let mut s = String::new();
    // Binaural arch (ear tubes)
    s.push_str(r##"<path d="M -10 -16 Q -5 -22 0 -16 Q 5 -22 10 -16" fill="none" stroke="#24292e" stroke-width="2" stroke-linecap="round" />"##);
    // Ear tips
    s.push_str(r##"<circle cx="-10" cy="-16" r="2" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##);
    s.push_str(
        r##"<circle cx="10" cy="-16" r="2" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##,
    );
    // Flexible tube loop
    s.push_str(r##"<path d="M 0 -18 Q 0 -8 4 -4 Q 8 2 2 10 Q -4 16 2 20 Q 8 22 10 18" fill="none" stroke="#0969da" stroke-width="2.6" stroke-linecap="round" />"##);
    // Chest piece bell & diaphragm
    s.push_str(
        r##"<circle cx="12" cy="18" r="6" fill="#f0f6fc" stroke="#111111" stroke-width="2" />"##,
    );
    s.push_str(
        r##"<circle cx="12" cy="18" r="3.5" fill="#d0d7de" stroke="#111111" stroke-width="1" />"##,
    );
    s
}

/// Podium / Conference Microphone with gooseneck stand
fn render_microphone_svg() -> String {
    let mut s = String::new();
    // Heavy base
    s.push_str(r##"<ellipse cx="0" cy="18" rx="14" ry="4" fill="#24292e" stroke="#111111" stroke-width="1.8" />"##);
    s.push_str(r##"<ellipse cx="0" cy="16" rx="9" ry="2.5" fill="#30363d" stroke="#111111" stroke-width="1.2" />"##);
    // Gooseneck arm
    s.push_str(r##"<path d="M 0 16 L 0 6 Q -1 -2 4 -8 L 8 -14" fill="none" stroke="#111111" stroke-width="2.5" stroke-linecap="round" />"##);
    // Microphone capsule
    s.push_str(r##"<g transform="translate(8, -14) rotate(30)">"##);
    s.push_str(r##"<rect x="-4" y="-10" width="8" height="12" rx="4" fill="#24292e" stroke="#111111" stroke-width="1.8" />"##);
    // Metallic mesh grille lines
    s.push_str(r##"<line x1="-3" y1="-6" x2="3" y2="-6" stroke="#8c959f" stroke-width="1" />"##);
    s.push_str(r##"<line x1="-3" y1="-3" x2="3" y2="-3" stroke="#8c959f" stroke-width="1" />"##);
    s.push_str(r##"<circle cx="0" cy="1" r="1.5" fill="#d90429" />"##); // On-air red tally light
    s.push_str("</g>");
    s
}

/// RU Stainless Steel Food Tray (Bandeja do RU)
fn render_tray_svg() -> String {
    let mut s = String::new();
    // Tray outer rim
    s.push_str(r##"<rect x="-24" y="-7" width="48" height="14" rx="3" fill="#e1e4e8" stroke="#111111" stroke-width="2" />"##);
    // Main food compartment (rice/beans)
    s.push_str(r##"<rect x="-21" y="-5" width="18" height="10" rx="1.5" fill="#f6f8fa" stroke="#111111" stroke-width="1.2" />"##);
    // Food texture lines
    s.push_str(r##"<line x1="-18" y1="-2" x2="-8" y2="-2" stroke="#afb8c1" stroke-width="1" stroke-linecap="round" />"##);
    s.push_str(r##"<line x1="-16" y1="1" x2="-6" y2="1" stroke="#afb8c1" stroke-width="1" stroke-linecap="round" />"##);
    // Salad compartment (top middle)
    s.push_str(r##"<rect x="-1" y="-5" width="11" height="4.5" rx="1" fill="#f0fdf4" stroke="#111111" stroke-width="1" />"##);
    // Protein compartment (bottom middle)
    s.push_str(r##"<rect x="-1" y="0.5" width="11" height="4.5" rx="1" fill="#fffbeb" stroke="#111111" stroke-width="1" />"##);
    // Cup / dessert holder ring (right)
    s.push_str(r##"<ellipse cx="16" cy="-2" rx="4.5" ry="3" fill="#ffffff" stroke="#111111" stroke-width="1.2" />"##);
    s.push_str(r##"<ellipse cx="16" cy="-2" rx="3" ry="1.8" fill="#ffd166" opacity="0.8" />"##);
    // Fork & knife slot
    s.push_str(r##"<line x1="13" y1="3" x2="21" y2="3" stroke="#24292e" stroke-width="1.2" stroke-linecap="round" />"##);
    s
}

/// University Student Backpack (Mochila Universitária)
fn render_backpack_svg() -> String {
    let mut s = String::new();
    // Top handle loop
    s.push_str(r##"<path d="M -5 -19 Q 0 -25 5 -19" fill="none" stroke="#111111" stroke-width="2.2" stroke-linecap="round" />"##);
    // Main backpack body
    s.push_str(r##"<path d="M -14 -18 Q 0 -22 14 -18 Q 16 2 15 14 Q 0 17 -15 14 Q -16 2 -14 -18 Z" fill="#24292e" stroke="#111111" stroke-width="2" stroke-linejoin="round" />"##);
    // Top zipper arc
    s.push_str(r##"<path d="M -11 -14 Q 0 -17 11 -14" fill="none" stroke="#8c959f" stroke-width="1.2" stroke-dasharray="2,2" />"##);
    // Front pocket pouch
    s.push_str(r##"<rect x="-10" y="-4" width="20" height="14" rx="2" fill="#30363d" stroke="#111111" stroke-width="1.6" />"##);
    // Front pocket zipper
    s.push_str(r##"<line x1="-8" y1="-1" x2="8" y2="-1" stroke="#8c959f" stroke-width="1.2" stroke-dasharray="1.5,1.5" />"##);
    // Zipper pull tab
    s.push_str(r##"<polygon points="-1,-1 1,-1 0,2" fill="#ffffff" stroke="#111111" stroke-width="0.8" />"##);
    // Side water bottle in mesh pocket
    s.push_str(r##"<rect x="12" y="-6" width="5" height="14" rx="1.5" fill="#0969da" stroke="#111111" stroke-width="1.2" />"##);
    s.push_str(r##"<ellipse cx="14.5" cy="-6" rx="2.5" ry="1" fill="#f0f6fc" stroke="#111111" stroke-width="1" />"##);
    // Shoulder straps outline
    s.push_str(r##"<path d="M -12 -12 Q -18 0 -13 12" fill="none" stroke="#111111" stroke-width="2.2" stroke-linecap="round" />"##);
    s
}

/// Modern Smartphone with notification bubbles (Celular)
fn render_phone_svg() -> String {
    let mut s = String::new();
    // Phone casing
    s.push_str(r##"<rect x="-8" y="-16" width="16" height="32" rx="3.5" fill="#1b1f23" stroke="#111111" stroke-width="2" />"##);
    // Screen bezel
    s.push_str(r##"<rect x="-6.5" y="-13.5" width="13" height="27" rx="2" fill="#0d1117" stroke="#30363d" stroke-width="0.8" />"##);
    // Front camera punch-hole
    s.push_str(r##"<circle cx="0" cy="-14.5" r="0.8" fill="#484f58" />"##);
    // Chat notification bubble 1
    s.push_str(r##"<rect x="-5" y="-10" width="10" height="5" rx="1.5" fill="#238636" />"##);
    s.push_str(r##"<line x1="-3.5" y1="-7.5" x2="3.5" y2="-7.5" stroke="#ffffff" stroke-width="0.8" stroke-linecap="round" />"##);
    // Chat notification bubble 2
    s.push_str(r##"<rect x="-5" y="-3" width="7" height="4" rx="1" fill="#1f6feb" />"##);
    s.push_str(r##"<line x1="-3.5" y1="-1" x2="0.5" y2="-1" stroke="#ffffff" stroke-width="0.8" stroke-linecap="round" />"##);
    // Home indicator bar
    s.push_str(r##"<line x1="-3" y1="11" x2="3" y2="11" stroke="#8c959f" stroke-width="1" stroke-linecap="round" />"##);
    s
}

/// Umbrella with J-handle and folded canopy (Guarda-chuva)
fn render_umbrella_svg() -> String {
    let mut s = String::new();
    // Metal shaft
    s.push_str(r##"<line x1="0" y1="-20" x2="0" y2="16" stroke="#111111" stroke-width="2.2" stroke-linecap="round" />"##);
    // Ferrule tip
    s.push_str(r##"<polygon points="-1.5,-20 1.5,-20 0,-25" fill="#8c959f" stroke="#111111" stroke-width="1" />"##);
    // Folded canopy fabric
    s.push_str(r##"<path d="M 0 -18 Q -9 0 -7 10 L 7 10 Q 9 0 0 -18 Z" fill="#24292e" stroke="#111111" stroke-width="2" stroke-linejoin="round" />"##);
    // Fabric rib lines
    s.push_str(r##"<path d="M 0 -18 L -3 10" fill="none" stroke="#484f58" stroke-width="1" />"##);
    s.push_str(r##"<path d="M 0 -18 L 3 10" fill="none" stroke="#484f58" stroke-width="1" />"##);
    // Wrap strap with snap button
    s.push_str(r##"<rect x="-7.5" y="-2" width="15" height="3" rx="1" fill="#1b1f23" stroke="#111111" stroke-width="0.8" />"##);
    s.push_str(r##"<circle cx="4" cy="-0.5" r="0.9" fill="#f0883e" />"##);
    // Classic wooden J-handle
    s.push_str(r##"<path d="M 0 16 L 0 22 Q 0 28 6 28 Q 12 28 12 22 Q 12 18 8 18" fill="none" stroke="#5d4037" stroke-width="2.8" stroke-linecap="round" stroke-linejoin="round" />"##);
    s
}

/// Spiral Academic Notebook with ruled pages and pen (Caderno)
fn render_caderno_svg() -> String {
    let mut s = String::new();
    // Notebook cover backing
    s.push_str(r##"<rect x="-14" y="-16" width="28" height="32" rx="2" fill="#0969da" stroke="#111111" stroke-width="2" />"##);
    // Paper pages
    s.push_str(r##"<rect x="-12" y="-14" width="24" height="28" rx="1" fill="#fdfbf7" stroke="#111111" stroke-width="1.2" />"##);
    // Margin line
    s.push_str(r##"<line x1="-7" y1="-14" x2="-7" y2="14" stroke="#cf222e" stroke-width="0.8" opacity="0.7" />"##);
    // Ruled lines
    let line_ys = [-10.0, -6.0, -2.0, 2.0, 6.0, 10.0];
    for ly in line_ys {
        s.push_str(&format!(
            r##"<line x1="-5.0" y1="{ly:.1}" x2="10.0" y2="{ly:.1}" stroke="#afb8c1" stroke-width="0.8" />"##
        ));
    }
    // Wire spiral rings along the left binding
    let spiral_ys = [-13.0, -9.0, -5.0, -1.0, 3.0, 7.0, 11.0];
    for sy in spiral_ys {
        s.push_str(&format!(
            r##"<ellipse cx="-13" cy="{sy:.1}" rx="2.2" ry="1.2" fill="none" stroke="#111111" stroke-width="1.2" />"##
        ));
    }
    // Ballpoint pen resting on side
    s.push_str(r##"<rect x="8.5" y="-12" width="2" height="22" rx="0.8" fill="#f0883e" stroke="#111111" stroke-width="0.8" />"##);
    s.push_str(r##"<polygon points="8.5,10 10.5,10 9.5,13" fill="#24292e" stroke="#111111" stroke-width="0.6" />"##);
    s
}

