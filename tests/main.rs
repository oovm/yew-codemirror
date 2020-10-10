use wasm_bindgen_test::*;
use syntect_yew::{render_to_string, KaTeXOptions};


#[wasm_bindgen_test]
fn test() {
    let d = KaTeXOptions::display_mode();
    let i = KaTeXOptions::inline_mode();

    assert_eq!(d.render("\\frac12"), i.render("\\frac12"));
}


#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1, 1);
}
