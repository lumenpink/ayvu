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
