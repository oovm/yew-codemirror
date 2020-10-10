use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::panic::RefUnwindSafe;
use serde_derive::{Serialize,Deserialize};
mod options;

/// Output type from KaTeX.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum OutputType {
    /// Outputs KaTeX in HTML only.
    Html,
    /// Outputs KaTeX in MathML only.
    Mathml,
    /// Outputs HTML for visual rendering and includes MathML for accessibility.
    HtmlAndMathml,
}

#[wasm_bindgen]
#[derive(Clone,Deserialize, Serialize)]
/// Read <https://katex.org/docs/options.html> for more information.
pub struct KaTeXOptions {
    /// Whether to render the math in the display mode.
    displayMode: bool,
    /// KaTeX output type.
    output_type: Option<OutputType>,
    /// Whether to have `\tags` rendered on the left instead of the right.
    leqno: Option<bool>,
    /// Whether to make display math flush left.
    fleqn: Option<bool>,
    /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
    throw_on_error: Option<bool>,
    /// Color used for invalid LaTeX.
    error_color: Option<String>,
    /// Collection of custom macros.
    /// Read <https://katex.org/docs/options.html> for more information.
    macros: HashMap<String, String>,
    /// Specifies a minimum thickness, in ems.
    /// Read <https://katex.org/docs/options.html> for more information.
    min_rule_thickness: Option<f64>,
    /// Max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    /// Read <https://katex.org/docs/options.html> for more information.
    max_size: Option<Option<f64>>,
    /// Limit the number of macro expansions to the specified number.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    /// Read <https://katex.org/docs/options.html> for more information.
    max_expand: Option<Option<i32>>,
    /// Whether to trust users' input.
    /// Cannot be assigned at the same time with [`OptsBuilder::trust_callback`].
    /// Read <https://katex.org/docs/options.html> for more information.
    trust: Option<bool>,

    global_group: Option<bool>,
}

impl Default for KaTeXOptions {
    fn default() -> Self {
        Self {
            displayMode: true,
            output_type: Some(OutputType::Html),
            leqno: None,
            fleqn: None,
            throw_on_error: Some(false),
            error_color: None,
            macros: Default::default(),
            min_rule_thickness: None,
            max_size: None,
            max_expand: None,
            trust: None,
            global_group: None,
        }
    }
}


#[wasm_bindgen(module = "/src/katex.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = "renderToString")]
    pub fn render_to_string(expr: &str, args: &JsValue) -> String;
}

impl KaTeXOptions {
    pub fn display_mode() -> KaTeXOptions {
        let mut o = KaTeXOptions::default();
        o.displayMode = true;
        return o;
    }
    pub fn inline_mode() -> KaTeXOptions {
        let mut o = KaTeXOptions::default();
        o.displayMode = false;
        return o;
    }

    pub fn render(&self, input: &str) -> String {
        render_to_string(input, &JsValue::from_serde(self).unwrap())
    }
}


