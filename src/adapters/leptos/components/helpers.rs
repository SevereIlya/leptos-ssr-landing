use leptos::prelude::*;

pub const INPUT_CLASSES: &str = "p-3.5 rounded-lg bg-surface-1 text-foreground border border-border transition-colors duration-150 placeholder:text-muted-foreground focus:outline-none focus:border-gold/50";

#[component]
pub fn Section(
    #[prop(optional)] id: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let base_classes = "py-20 md:py-32 relative overflow-hidden border-t border-border/30 bg-surface-1";
    view! {
        <section id=id class=format!("{} {}", base_classes, class)>
            {children()}
        </section>
    }
}

#[component]
pub fn Container(
    #[prop(optional)] class: &'static str,
    children: Children
) -> impl IntoView {
    let base_classes = "mx-auto max-w-7xl px-4 sm:px-6 relative z-10 w-full";
    view! { <div class=format!("{} {}", base_classes, class)>{children()}</div> }
}

#[component]
pub fn Form(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let base_classes = "bg-card/60 rounded-2xl border border-border shadow-2xl backdrop-blur-md";
    view! { <div class=format!("{} {}", base_classes, class)>{children()}</div> }
}

#[component]
pub fn Field(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1.5">
            <label class="text-[10px] uppercase tracking-[0.1em] font-medium text-muted-foreground">{label}</label>
            {children()}
        </div>
    }
}

#[component]
pub fn Button(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] disabled: Signal<bool>,
    #[prop(optional)] pending: Signal<bool>,
    #[prop(optional)] r#type: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type=r#type
            disabled=move || disabled.get() || pending.get()
            class=move || {
                let base = format!(
                    "w-full mt-2 p-4 rounded-lg font-semibold transition-all duration-200 active:scale-[0.98] flex items-center justify-center gap-2 {class}",
                );
                if pending.get() {
                    base + " bg-gold/70 text-background cursor-wait animate-pulse"
                } else if disabled.get() {
                    base + " bg-background border border-border text-muted-foreground opacity-70 cursor-not-allowed"
                } else {
                    base
                        + " gradient-gold border border-transparent text-foreground hover:scale-[1.02] glow-gold cursor-pointer"
                }
            }
        >
            <span class=move || {
                if pending.get() { "hidden" } else { "inline-flex items-center gap-2" }
            }>{children()}</span>
        
            <span class=move || {
                if pending.get() { "inline-flex items-center gap-2" } else { "hidden" }
            }>"Обработка..."</span>
        </button>
    }
}

#[component]
pub fn ConsentSubmit(
    consent: ReadSignal<bool>,
    set_consent: WriteSignal<bool>,
    #[prop(optional)] pending: Signal<bool>,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] button_text: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("flex flex-col gap-3 {}", class)>
            <Button r#type="submit" disabled=Signal::derive(move || !consent.get()) pending=pending>
                {button_text}
            </Button>

            <label class="mt-1 flex place-items-center gap-3 mt-2 cursor-pointer group">
                <input
                    type="checkbox"
                    class="w-4 h-4 shrink-0 accent-gold cursor-pointer"
                    on:change=move |ev| {
                        set_consent.set(event_target_checked(&ev));
                    }
                />
                <span class="text-[11px] leading-tight text-muted-foreground">
                    "Нажимая на кнопку, я даю согласие на обработку персональных данных и соглашаюсь с "
                    <a href="/privacy" target="_blank" class="text-gold hover:underline">
                        "Политикой конфиденциальности"
                    </a> "."
                </span>
            </label>
        </div>
    }
}
