use crate::adapters::leptos::components::*;
use leptos::prelude::*;

struct FaqItem {
    question: &'static str,
    answer: &'static str,
}

#[component]
pub fn FaqSection() -> impl IntoView {
    let faqs = vec![
        FaqItem {
            question: "Это не гадание и не приворот?",
            answer: "Абсолютно нет. Натальная карта — это точная астрономическая модель положения планет в минуту вашего рождения. Я не предсказываю судьбу и не снимаю порчу. Я анализирую алгоритмы вашей психики и сценарии событий, давая конкретный, логичный план действий.",
        },
        FaqItem {
            question: "А если я не помню точное время своего рождения?",
            answer: "Без точного времени построить карту невозможно. Но это не проблема. Существует процедура «ректификация» — восстановление времени рождения по ключевым событиям вашей жизни (брак, переезды, операции). Обсудим это на предварительной консультации.",
        },
        FaqItem {
            question: "Сколько стоит разбор?",
            answer: "Стоимость зависит от глубины вашего запроса и выбранного формата (письменный отчет или живая онлайн-сессия). Оставьте заявку, я свяжусь с вами, выслушаю вашу ситуацию и предложу оптимальный вариант без навязывания лишних услуг.",
        },
        FaqItem {
            question: "Как быстро я получу результаты?",
            answer: "Если мы работаем онлайн, инсайты придут прямо на встрече. Письменный разбор занимает от 3 до 7 дней подготовки. Скорость реальных изменений в жизни зависит только от вас — карта дает маршрут, но идти по нему придется вам.",
        },
        FaqItem {
            question: "Безопасно ли оставлять вам свои личные данные?",
            answer: "Полностью. Дата, время и место рождения, а также все детали вашей ситуации остаются строго между нами. Я гарантирую полную конфиденциальность и не передаю информацию третьим лицам.",
        },
    ];
    
    let (open_idx, set_open_idx) = signal::<Option<usize>>(Some(0));

    view! {
        <Section id="faq" class="!pb-0 md:!pb-0">
            <Container>
                <div class="mb-12 text-center">
                    <p class="mb-4 inline-flex items-center gap-2 rounded-full border border-gold/30 bg-gold/5 px-4 py-1.5 text-xs uppercase tracking-[0.2em] text-gold">
                        <span class="text-[10px]">"✦"</span>
                        "FAQ"
                    </p>
                    <h2 class="text-3xl md:text-5xl font-serif text-foreground leading-[1.1] text-balance">
                        "Отвечаю заранее"
                    </h2>
                </div>
                <div class="space-y-6">
                    {faqs
                        .into_iter()
                        .enumerate()
                        .map(|(i, faq)| {
                            let is_open = move || open_idx.get() == Some(i);
                            view! {
                                <div class="border border-border/50 bg-background rounded-2xl overflow-hidden transition-colors hover:border-gold/30">
                                    <button
                                        on:click=move |_| {
                                            if is_open() { set_open_idx.set(None) } else { set_open_idx.set(Some(i)) }
                                        }
                                        class="w-full px-6 py-5 flex items-center justify-between text-left focus:outline-none cursor-pointer group"
                                    >
                                        <span class=move || {
                                            let base = "font-serif text-lg sm:text-xl transition-colors pr-4 ";
                                            if is_open() {
                                                base.to_owned() + "text-gold"
                                            } else {
                                                base.to_owned() + "text-foreground group-hover:text-gold"
                                            }
                                        }>{faq.question}</span>
                                        <div class=move || {
                                            let base = "w-8 h-8 rounded-full border flex items-center justify-center transition-all duration-300 shrink-0 ";
                                            if is_open() {
                                                base.to_owned() + "border-gold bg-gold text-background rotate-180"
                                            } else {
                                                base.to_owned() + "border-border text-gold group-hover:border-gold/50"
                                            }
                                        }>
                                            {move || {
                                                if is_open() {
                                                    view! { <span class="text-lg leading-none mb-1">"−"</span> }
                                                        .into_any()
                                                } else {
                                                    view! { <span class="text-lg leading-none mb-0.5">"+"</span> }
                                                        .into_any()
                                                }
                                            }}
                                        </div>
                                    </button>
                                    <div class=move || {
                                        let base = "grid transition-all duration-300 ease-in-out ";
                                        if is_open() {
                                            base.to_owned() + "grid-rows-[1fr] opacity-100"
                                        } else {
                                            base.to_owned() + "grid-rows-[0fr] opacity-0"
                                        }
                                    }>
                                        <div class="overflow-hidden">
                                            <p class="px-6 pb-6 text-sm sm:text-base text-muted-foreground leading-relaxed border-t border-border/10 pt-4 mt-2 mx-2">
                                                {faq.answer}
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </Container>
        </Section>
    }
}
