//! Small DOM helpers.
use leptos::prelude::*;

pub fn set_title(title: &str) {
    document().set_title(&format!("{title} · Loopseed"));
}

pub fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}

/// `1,024` style thousands grouping for counts.
pub fn grouped(value: u64) -> String {
    let digits = value.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}
