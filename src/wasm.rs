use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_comic(
    script: &str,
    title: Option<String>,
    layout: Option<String>,
) -> Result<String, JsValue> {
    let mut strip = crate::parser::parse_dialogue_script(script, title.as_deref())
        .map_err(|e| JsValue::from_str(&e))?;

    if let Some(ref l) = layout {
        strip.layout = crate::model::StripLayout::from_str_loose(l);
    }

    let svg = crate::renderer::render_comic_strip_to_svg(&strip);
    Ok(svg)
}

#[wasm_bindgen]
pub fn list_characters_json() -> String {
    let registry = crate::character::CharacterRegistry::default();
    let metadata = registry.list_metadata();
    serde_json::to_string(&metadata).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn get_character_avatar_svg(id: &str) -> Option<String> {
    let registry = crate::character::CharacterRegistry::default();
    if let Some(pack) = registry.get(id) {
        let pose = pack.get_pose_svg(&crate::model::Emotion::Neutral);
        let (w, h) = pack.dimensions();
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="100%" height="100%">{}</svg>"#,
            w, h, pose
        );
        Some(svg)
    } else {
        None
    }
}

#[wasm_bindgen]
pub fn get_character_pose_svg(id: &str, emotion: &str) -> Option<String> {
    let registry = crate::character::CharacterRegistry::default();
    if let Some(pack) = registry.get(id) {
        let parsed_emotion = crate::model::Emotion::from_str_loose(emotion);
        let pose = pack.get_pose_svg(&parsed_emotion);
        let (w, h) = pack.dimensions();
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="100%" height="100%">{}</svg>"#,
            w, h, pose
        );
        Some(svg)
    } else {
        None
    }
}

#[wasm_bindgen]
pub fn get_character_detail_json(id: &str) -> Option<String> {
    let registry = crate::character::CharacterRegistry::default();
    registry
        .get_detail(id)
        .and_then(|d| serde_json::to_string(&d).ok())
}

#[wasm_bindgen]
pub fn list_backdrops_json() -> String {
    let list = vec![
        "none",
        "ru",
        "hu_fachada",
        "hall_ccs",
        "sala_de_aula",
        "cafe_ccs",
        "laboratorio",
        "hu_enfermaria",
        "bu_ufsc",
        "pastoral",
        "room",
    ];
    serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn get_backdrop_svg(name: &str, width: f64, height: f64) -> Option<String> {
    if name == "none" || name.is_empty() {
        return None;
    }
    // Try vector backdrops first
    if let Some(svg) = crate::backdrop::get_vector_backdrop_svg(name, 0.0, 0.0, width, height) {
        return Some(svg);
    }
    // Try bitmap Comic Chat backdrops
    if let Some(b64) = crate::assets::get_backdrop_b64(name) {
        return Some(format!(
            r#"<image href="data:image/png;base64,{}" x="0" y="0" width="{:.1}" height="{:.1}" preserveAspectRatio="none" />"#,
            b64, width, height
        ));
    }
    None
}
