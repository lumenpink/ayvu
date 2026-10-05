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
