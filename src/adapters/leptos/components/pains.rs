use crate::adapters::leptos::components::*;
use leptos::prelude::*;

struct PainItem {
    id: &'static str,
    title: &'static str,
    desc: &'static str,
}

#[component]
pub fn PainsSection() -> impl IntoView {
    let pains = vec![
        PainItem {
            id: "01",
            title: "Финансовый потолок",
            desc: "Работаете 24/7, постоянно учитесь, но доход стоит на месте. Кредиты, долги и липкий страх, что так будет всегда.",
        },
        PainItem {
            id: "02",
            title: "Сценарий «Грабли»",
            desc: "Притягиваете один и тот же тип сложных партнеров. Одиночество вдвоем, абьюз или болезненные, повторяющиеся разрывы.",
        },
        PainItem {
            id: "03",
            title: "Потеря смыслов",
            desc: "Вроде все есть: семья, работа, быт. Но внутри пустота. Просыпаетесь без энергии и не понимаете, «кем станете, когда вырастете».",
        },
        PainItem {
            id: "04",
            title: "Жизнь на автопилоте",
            desc: "Годы идут, а декорации не меняются. Вы чувствуете свой огромный потенциал, но не знаете, с какой стороны к нему подступиться.",
        },
        PainItem {
            id: "05",
            title: "Страх ошибки",
            desc: "Стоите на распутье (развод, переезд, смена профессии) и боитесь сделать неверный шаг, который окончательно разрушит жизнь.",
        },
        PainItem {
            id: "06",
            title: "Хроническая усталость",
            desc: "Нет сил ни на новые проекты, ни на хобби. Только постоянное чувство вины за собственную лень и упущенное время.",
        },
    ];

    view! {
        <Section id="pains">
            <Container>
                <div class="mb-16 md:mb-20 text-center max-w-3xl mx-auto flex flex-col items-center">
                    <p class="mb-4 inline-flex items-center gap-2 rounded-full border border-gold/30 bg-gold/5 px-4 py-1.5 text-xs uppercase tracking-[0.2em] text-gold">
                        <span class="text-[10px]">"✦"</span>
                        "Узнаете себя?"
                    </p>
                    <h2 class="text-3xl md:text-5xl font-serif text-foreground leading-[1.1] text-balance">
                        "6 состояний, с которыми " <br class="hidden md:block" />
                        <span class="text-gradient-gold italic pr-2">
                            "ко мне приходят чаще всего"
                        </span>
                    </h2>
                </div>
                <div class="grid gap-4 sm:gap-6 md:grid-cols-2 lg:grid-cols-3">
                    {pains
                        .into_iter()
                        .map(|pain| {
                            view! {
                                <div class="group relative p-6 sm:p-8 rounded-2xl bg-card/30 border border-border/50 backdrop-blur-sm transition-all duration-300 hover:-translate-y-1 hover:bg-card/60 hover:border-gold/40 hover:shadow-[0_8px_30px_rgb(0,0,0,0.12)]">
                                    <div class="absolute top-6 right-6 font-serif text-4xl text-border/40 transition-colors duration-300 group-hover:text-gold/20 select-none">
                                        {pain.id}
                                    </div>

                                    <div class="relative z-10">
                                        <h3 class="text-xl md:text-2xl font-serif text-foreground mb-3 group-hover:text-gold transition-colors duration-300 pr-10">
                                            {pain.title}
                                        </h3>
                                        <p class="text-sm md:text-base leading-relaxed text-muted-foreground">
                                            {pain.desc}
                                        </p>
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
