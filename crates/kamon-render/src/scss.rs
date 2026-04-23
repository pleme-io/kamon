//! SCSS renderer — emits `$kamon-...` variables. Mirror of CSS.

use kamon_tokens::TokenSet;

pub fn render(t: &TokenSet) -> String {
    let mut out = String::from("// kamon — pleme-io design tokens (generated)\n\n");
    for (k, v) in t.color.entries() {
        out.push_str(&format!("$kamon-{}: {};\n", k.replace('_', "-"), v.hex()));
    }
    for (role, palette_key) in t.roles.pairs() {
        let rgb = t.color.get(palette_key).expect("role resolves");
        out.push_str(&format!("$kamon-color-{role}: {};\n", rgb.hex()));
    }
    for (k, v) in t.spacing.pairs() {
        out.push_str(&format!("$kamon-space-{k}: {v}px;\n"));
    }
    for (k, v) in t.radius.pairs() {
        out.push_str(&format!("$kamon-radius-{k}: {v}px;\n"));
    }
    out
}
