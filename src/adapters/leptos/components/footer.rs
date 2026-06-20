use crate::adapters::leptos::components::*;
use crate::constants::*;
use leptos::prelude::*;

struct LinkItem {
    href: &'static str,
    label: &'static str,
}

struct ContactItem {
    icon_emoji: &'static str,
    text: &'static str,
    href: Option<&'static str>,
}

#[component]
pub fn Footer() -> impl IntoView {
    let current_year = chrono::Utc::now().format("%Y").to_string();

    let nav_links = vec![
        LinkItem {
            href: "#about",
            label: "Обо мне",
        },
        LinkItem {
            href: "#method",
            label: "Метод работы",
        },
        LinkItem {
            href: "#reviews",
            label: "Отзывы клиентов",
        },
        LinkItem {
            href: "#faq",
            label: "Частые вопросы",
        },
    ];

    let doc_links = vec![
        LinkItem {
            href: "/privacy",
            label: "Политика конфиденциальности",
        },
        LinkItem {
            href: "/offer",
            label: "Публичная оферта",
        },
        LinkItem {
            href: "/disclaimer",
            label: "Отказ от ответственности",
        },
    ];

    let contacts = vec![
        ContactItem {
            icon_emoji: "✉",
            text: EMAIL_DISPLAY,
            href: Some(EMAIL_HREF),
        },
        ContactItem {
            icon_emoji: "➤",
            text: TELEGRAM_MANAGER_DISPLAY,
            href: Some(TELEGRAM_MANAGER_URL),
        },
        ContactItem {
            icon_emoji: "◷",
            text: "10:00 – 20:00 (Мск)",
            href: None,
        },
    ];

    view! {
        <footer class="bg-background pt-16 pb-8 border-t border-border/30">
            <Container class="relative w-full">
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-12 lg:gap-8 mb-16">
                    <div class="flex flex-col gap-5">
                        <a href="/" class="flex items-center gap-3 font-serif group w-fit">
                            <img
                                src="/images/logo.png"
                                alt="Логотип"
                                class="w-14 h-14 object-contain mix-blend-screen group-hover:scale-110 transition-transform duration-300"
                            />
                            <span class="text-gradient-gold font-bold text-2xl whitespace-nowrap">"Astra Regalis"</span>
                        </a>
                        <div>
                            <h3 class="text-lg font-serif text-foreground uppercase tracking-widest mb-2">
                                "Алла Валерьевна"
                            </h3>
                            <p class="text-sm text-muted-foreground leading-relaxed">
                                "Персональные разборы натальных карт. Перевожу язык звезд на язык конкретных действий."
                            </p>
                        </div>
                    </div>
                    <div class="flex flex-col gap-5 lg:pl-8">
                        <h4 class="text-lg font-serif text-foreground">"Навигация"</h4>
                        <nav class="flex flex-col gap-3 text-sm text-muted-foreground">
                            {nav_links
                                .into_iter()
                                .map(|link| {
                                    view! {
                                        <a href=link.href class="hover:text-gold transition-colors w-fit">
                                            {link.label}
                                        </a>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </nav>
                    </div>
                    <div class="flex flex-col gap-5">
                        <h4 class="text-lg font-serif text-foreground">"Контакты"</h4>
                        <div class="flex flex-col gap-3 text-sm text-muted-foreground">
                            {contacts
                                .into_iter()
                                .map(|item| {
                                    match item.href {
                                        Some(url) => {
                                            view! {
                                                <a
                                                    href=url
                                                    target="_blank"
                                                    rel="nofollow noopener noreferrer"
                                                    class="flex items-center gap-3 hover:text-gold transition-colors w-fit"
                                                >
                                                    <span class="text-gold text-base">{item.icon_emoji}</span>
                                                    <span>{item.text}</span>
                                                </a>
                                            }
                                                .into_any()
                                        }
                                        None => {
                                            view! {
                                                <p class="flex items-center gap-3">
                                                    <span class="text-gold text-base">{item.icon_emoji}</span>
                                                    <span>{item.text}</span>
                                                </p>
                                            }
                                                .into_any()
                                        }
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                        <div class="flex gap-3 mt-2">
                            <a
                                href=TELEGRAM_GROUP_URL
                                target="_blank"
                                rel="nofollow noopener noreferrer"
                                class="w-10 h-10 rounded-full bg-card border border-border flex items-center justify-center text-muted-foreground hover:border-gold hover:text-gold hover:bg-gold/5 transition-all duration-300"
                                title="Наш Telegram-канал"
                            >
                                <TelegramIcon class="w-7 h-7" />
                            </a>
                            <a
                                href=VK_GROUP_URL
                                target="_blank"
                                rel="nofollow noopener noreferrer"
                                class="w-10 h-10 rounded-full bg-card border border-border flex items-center justify-center text-muted-foreground hover:border-gold hover:text-gold hover:bg-gold/5 transition-all duration-300"
                                title="Наша группа ВКонтакте"
                            >
                                <VkIcon class="w-9 h-9" />
                            </a>
                        </div>
                    </div>
                    <div class="flex flex-col gap-5">
                        <h4 class="text-lg font-serif text-foreground">"Документы"</h4>
                        <div class="flex flex-col gap-3 text-sm text-muted-foreground">
                            {doc_links
                                .into_iter()
                                .map(|link| {
                                    view! {
                                        <a
                                            href=link.href
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="hover:text-gold transition-colors w-fit"
                                        >
                                            {link.label}
                                        </a>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                        <div class="flex items-center gap-2 mt-2">
                            <div class="w-[52px] h-8 bg-white rounded-md flex items-center justify-center shadow-sm select-none">
                                <MirIcon class="h-[14px] w-auto" />
                            </div>
                            <div class="w-[52px] h-8 bg-white rounded-md flex items-center justify-center shadow-sm select-none">
                                <MastercardIcon class="h-[24px] w-auto" />
                            </div>
                            <div class="w-[52px] h-8 bg-white rounded-md flex items-center justify-center shadow-sm select-none">
                                <VisaIcon class="h-[14px] w-auto" />
                            </div>
                        </div>
                    </div>
                </div>
                <div class="border-t border-border/40 pt-8 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 text-xs text-muted-foreground/60">
                    <div class="flex flex-col gap-1">
                        <span class="font-medium text-foreground/70">
                            "ИП ПАСТУХОВА АЛЛА ВАЛЕРЬЕВНА"
                        </span>
                        <span>"ИНН: 471605261679 | ОГРНИП: 325784700157443"</span>
                    </div>
                    <div class="flex flex-col md:items-end gap-1">
                        <span>"© " {current_year} " Все права защищены."</span>
                        <span>"Копирование материалов запрещено."</span>
                        <span class="mt-2 text-[10px] opacity-50 hover:opacity-100 transition-opacity duration-300">
                            "Developed by "
                            <a
                                href="https://t.me/SevereILya"
                                target="_blank"
                                rel="nofollow noopener noreferrer"
                                class="hover:text-gold transition-colors font-medium tracking-wide"
                            >
                                "SevereIlya"
                            </a>
                        </span>
                    </div>
                </div>
            </Container>
        </footer>
    }
}
