use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ym)]
    pub fn ym(id: u32, action: &str, target: &str);
}

#[allow(unused_variables)]
#[allow(clippy::missing_const_for_fn)]
pub fn track_goal(goal_name: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use crate::constants::YANDEX_METRIKA_ID;
        ym(YANDEX_METRIKA_ID, "reachGoal", goal_name);
        leptos::logging::log!("Метрика: цель '{}' успешно отправлена", goal_name);
    }
}