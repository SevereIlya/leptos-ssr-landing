use crate::adapters::leptos::components::*;
use crate::constants::*;
use leptos::prelude::*;
use leptos::{ev, web_sys};

struct NavItem {
    href: &'static str,
    label: &'static str,
    hide_class: &'static str,
}

/// Возвращает true, если юзер проскроллил больше `threshold` пикселей
fn use_scrolled(threshold: f64) -> ReadSignal<bool> {
    let (is_scrolled, set_scrolled) = signal(false);

    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let initial_scroll = window.scroll_y().unwrap_or(0.0);
            set_scrolled.set(initial_scroll > threshold);
            
            let handle_scroll = move |_| {
                if let Some(win) = web_sys::window() {
                    let scroll_y = win.scroll_y().unwrap_or(0.0);
                    set_scrolled.set(scroll_y > threshold);
                }
            };
            
            window_event_listener(ev::scroll, handle_scroll);
        }
    });
    
    is_scrolled
}

#[component]
pub fn Header() -> impl IntoView {
    let is_scrolled = use_scrolled(20.0);

    let nav_links = vec![
        NavItem {
            href: "#method",
            label: "Метод",
            hide_class: "max-[1100px]:hidden",
        },
        NavItem {
            href: "#about",
            label: "Обо мне",
            hide_class: "max-[920px]:hidden",
        },
        NavItem {
            href: "#reviews",
            label: "Отзывы",
            hide_class: "max-[840px]:hidden",
        },
        NavItem {
            href: "#faq",
            label: "Вопросы",
            hide_class: "max-[1000px]:hidden",
        },
    ];

    view! {
        <header class=move || {
            let base = "fixed top-0 left-0 w-full z-50 transition-all duration-300 h-20 flex items-center border-b border-transparent ";
            if is_scrolled.get() {
                base.to_owned() + "bg-background/95 backdrop-blur-md border-b border-border/50 shadow-lg"
            } else {
                base.to_owned() + "bg-transparent"
            }
        }>
            <Container class="relative w-full">
                <div class="flex justify-between items-center gap-4">
                    <div class="flex items-center shrink-0">
                        <a href="/" class="flex items-center gap-3 font-serif group">
                            <img
                                src="/images/logo.png"
                                alt="Astra Regalis Logo"
                                class="w-10 h-10 md:w-12 md:h-12 object-contain mix-blend-screen group-hover:scale-110 transition-transform duration-300"
                            />
                            <span class="max-[530px]:hidden text-gradient-gold font-bold text-xl md:text-2xl whitespace-nowrap">
                                "Astra Regalis"
                            </span>
                        </a>
                        <div class="hidden xl:flex flex-col ml-4 pl-4 border-l border-border/60 text-[10px] leading-tight text-muted-foreground uppercase tracking-widest font-medium whitespace-nowrap">
                            <span>"Персональные разборы"</span>
                            <span>"натальных карт"</span>
                        </div>
                    </div>
                    <nav class="flex items-center gap-2 sm:gap-4 lg:gap-6 text-sm font-medium shrink-0">
                        {nav_links
                            .into_iter()
                            .map(|link| {
                                view! {
                                    <a
                                        href=link.href
                                        class=format!(
                                            "{} text-foreground/80 hover:text-white transition-colors duration-300 whitespace-nowrap",
                                            link.hide_class,
                                        )
                                    >
                                        {link.label}
                                    </a>
                                }
                            })
                            .collect::<Vec<_>>()}
                        <a
                            href="#hero"
                            class="max-[330px]:text-[11px] inline-flex items-center justify-center px-4 py-1.5 rounded-full border border-white/20 bg-white/5 backdrop-blur-sm text-white font-medium whitespace-nowrap transition-all duration-300 hover:-translate-y-0.5 hover:bg-white/15 hover:border-white/40 hover:shadow-[0_0_15px_rgba(255,255,255,0.15)] [text-shadow:0_0_8px_rgba(255,255,255,0.6)]"
                        >
                            "Хочу разбор"
                        </a>
                    </nav>
                    <div class="flex items-center gap-4 lg:gap-5 shrink-0">
                        <div class="flex items-center max-[580px]:hidden gap-3 max-[710px]:gap-1 max-[660px]:gap-0">
                            <a
                                href=TELEGRAM_MANAGER_URL
                                target="_blank"
                                rel="nofollow noopener noreferrer"
                                class="text-muted-foreground hover:text-gold transition-colors duration-300"
                                title="Написать в Telegram"
                            >
                                <TelegramIcon class="w-8 h-8" />
                            </a>
                            <a
                                href=WHATSAPP_MANAGER_URL
                                target="_blank"
                                rel="nofollow noopener noreferrer"
                                class="max-[710px]:hidden text-muted-foreground hover:text-gold transition-colors duration-300"
                                title="Написать в WhatsApp"
                            >
                                <WhatsappIcon class="w-7 h-7" />
                            </a>
                            <a
                                href=VK_MANAGER_URL
                                target="_blank"
                                rel="nofollow noopener noreferrer"
                                class="max-[660px]:hidden text-muted-foreground hover:text-gold transition-colors duration-300"
                                title="Написать во ВКонтакте"
                            >
                                <VkIcon class="w-10 h-10" />
                            </a>
                        </div>
                        <div class="hidden min-[480px]:block w-px h-6 bg-border/60"></div>
                        <a href=PHONE_HREF class="flex items-center gap-2 md:gap-3 group">
                            <div class="hidden min-[380px]:flex w-8 h-8 md:w-9 md:h-9 rounded-full bg-gold/10 border border-gold/20 items-center justify-center text-gold group-hover:bg-gold/30 group-hover:text-background transition-colors duration-300 shrink-0">
                                <PhoneIcon class="w-5 h-5" />
                            </div>
                            <div class="flex flex-col text-right">
                                <span class="text-xs sm:text-sm md:text-base font-semibold text-foreground group-hover:text-gold transition-colors duration-300 whitespace-nowrap">
                                    {PHONE_DISPLAY}
                                </span>
                                <span class="text-[9px] uppercase text-gold/80 font-bold tracking-[0.15em] whitespace-nowrap">
                                    "Позвонить"
                                </span>
                            </div>
                        </a>
                    </div>
                </div>
            </Container>
        </header>
    }
}
