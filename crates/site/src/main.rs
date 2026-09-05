//! loopseed.io — the Loopseed research program. Client-side Leptos application.
mod app;
mod components;
mod pages;
mod util;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
