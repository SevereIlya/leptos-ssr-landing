use leptos::prelude::*;

struct ReviewItem {
    name: &'static str,
    age: &'static str,
    request: &'static str,
    text: &'static str,
}

#[component]
pub fn ReviewsSection() -> impl IntoView {
    let reviews = vec![
        ReviewItem {
            name: "Марина",
            age: "34 года",
            request: "Постоянные долги и кредиты",
            text: "Пришла с темой финансового потолка. На разборе увидели причину — повторяющийся родовой сценарий, который блокировал деньги. Получила четкие инструкции, как это компенсировать. Через 4 месяца закрыла долг, который тянула три года.",
        },
        ReviewItem {
            name: "Алексей",
            age: "41 год",
            request: "Выбор ниши для бизнеса",
            text: "Долго выбирал между двумя направлениями. Разбор натальной карты показал, какое мое, а какое навязанное окружением. Рискнул пойти по 'своему' вектору. Прошло полгода — растем в выручке, а главное — я кайфую от процесса.",
        },
        ReviewItem {
            name: "Екатерина",
            age: "29 лет",
            request: "Кризис в отношениях",
            text: "Думала, что проблема в партнере. Но разбор совместимости подсветил мои собственные 'слепые зоны' и паттерны поведения. Муж заметил перемены уже через месяц, конфликтов стало в разы меньше.",
        },
        ReviewItem {
            name: "Игорь",
            age: "47 лет",
            request: "Потеря смысла жизни",
            text: "Я был жутким скептиком, пошел 'проверить'. Был в шоке, когда астролог описала мои внутренние конфликты точнее, чем психотерапевт. Ушел с конкретным планом действий на ближайший год.",
        },
    ];

    view! {
        <section id="reviews" class="py-20 md:py-32 bg-background border-t border-border/30 overflow-hidden">
            <div class="max-w-7xl mx-auto px-4 sm:px-6">
                <div class="mb-16 md:mb-20 text-center max-w-3xl mx-auto">
                    <p class="mb-4 inline-flex items-center gap-2 rounded-full border border-gold/30 bg-gold/5 px-4 py-1.5 text-xs uppercase tracking-[0.2em] text-gold">
                        <span class="text-[10px]">"✦"</span>
                        "Реальные истории"
                    </p>
                    <h2 class="text-3xl md:text-5xl font-serif text-foreground leading-[1.1] text-balance mb-6">
                        "Что говорят те, кто " <br class="hidden sm:block" />
                        <span class="text-gradient-gold italic pr-2">"уже прошел разбор"</span>
                    </h2>
                </div>
                <div class="grid md:grid-cols-2 gap-6 max-w-7xl mx-auto">
                    {reviews
                        .into_iter()
                        .map(|review| {
                            view! {
                                <div class="flex flex-col bg-card/20 border border-border/50 rounded-2xl p-6 sm:p-8 hover:border-gold/30 hover:bg-card/40 transition-colors">
                                    <div class="flex gap-1 text-gold text-sm mb-4">
                                        <span>"★"</span>
                                        <span>"★"</span>
                                        <span>"★"</span>
                                        <span>"★"</span>
                                        <span>"★"</span>
                                    </div>
                                    <p class="text-foreground/90 text-sm sm:text-base leading-relaxed mb-6 flex-grow italic">
                                        "«" {review.text} "»"
                                    </p>
                                    <div class="mt-auto pt-4 border-t border-border/30 flex justify-between items-center">
                                        <div>
                                            <p class="font-serif text-gold text-lg leading-none mb-1">{review.name}</p>
                                            <p class="text-[11px] text-muted-foreground uppercase tracking-widest">
                                                {review.age}
                                            </p>
                                        </div>
                                        <div class="bg-background border border-border px-3 py-1.5 rounded-full text-xs text-muted-foreground hidden sm:block">
                                            "Запрос: " {review.request}
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                <div class="mt-16 text-center">
                    <p class="text-muted-foreground mb-6">
                        "Хотите найти ответы на свои вопросы?"
                    </p>
                    <a
                        href="#hero"
                        class="inline-flex items-center justify-center rounded-lg gradient-gold px-8 py-3.5 text-sm font-semibold text-primary-foreground transition-transform hover:scale-[1.02] glow-gold"
                    >
                        "Обсудить вашу ситуацию"
                    </a>
                </div>
            </div>
        </section>
    }
}
