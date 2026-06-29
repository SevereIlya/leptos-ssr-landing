use leptos::prelude::*;
use leptos::web_sys;

#[component]
pub fn CookieBanner() -> impl IntoView {
    let (show, set_show) = signal(false);

    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage(){
                if storage.get_item("cookie_consent").unwrap_or(None).is_none() {
                    set_show.set(true);
                }
            }
        }
    });

    let accept_cookies = move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("cookie_consent", "true");
            }
        }
        set_show.set(false);
    };

    view! {
        <Show when=move || show.get()>
            <div class="fixed bottom-4 left-4 right-4 md:left-auto md:right-4 md:max-w-sm z-[100] animate-in slide-in-from-bottom-5 duration-500">
                <div class="bg-card border border-border/80 p-4 rounded-2xl shadow-2xl flex flex-col gap-3">
                    <p class="text-xs text-muted-foreground leading-relaxed">
                        "Мы используем файлы cookie для аналитики и улучшения работы сайта. Оставаясь на сайте, вы соглашаетесь с нашей "
                        <a href="/privacy" class="text-gold hover:underline">"Политикой конфиденциальности"</a>"."
                    </p>
                    <button
                        on:click=accept_cookies
                        class="w-full py-2 bg-background border border-border text-foreground text-sm font-medium rounded-lg hover:border-gold/50 hover:text-gold transition-colors"
                    >
                        "Хорошо, понятно"
                    </button>
                </div>
            </div>
        </Show>
    }
}