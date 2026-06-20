pub mod domain;
pub mod adapters;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub mod application;
        pub mod infrastructure;
    }
}

pub mod constants;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::logging::log!(
        "=========================================\n\
         | System Online. Hydration Complete.\n\
         | Powered by Rust, Leptos SSR & WASM.\n\
         | Hello, fellow developer! \n\
         ========================================="
    );
    leptos::mount::hydrate_body(crate::adapters::leptos::App);
}