use leptos::prelude::*;

#[component]
pub fn AboutMethodSection() -> impl IntoView {
    view! {
        <section id="method" class="relative py-20 md:py-32 border-t border-border/30 overflow-hidden bg-background">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 relative z-10">
                <div class="grid lg:grid-cols-2 gap-12 lg:gap-20 items-center mb-24">
                    <div>
                        <p class="mb-4 inline-flex items-center gap-2 font-serif text-gold text-lg">
                            <span class="text-xl">"✦"</span>
                            "Королевское искусство"
                        </p>
                        <h2 class="text-3xl sm:text-4xl md:text-5xl font-serif text-foreground leading-[1.1] text-balance mb-6">
                            "Веками астрология была "
                            <span class="text-gradient-gold italic pr-2">"спутником монархов"</span>
                        </h2>
                        <div class="space-y-4 text-muted-foreground leading-relaxed text-sm sm:text-base">
                            <p>
                                "Елизавета I, Лоренцо Медичи, Джон Пирпонт Морган — люди, творившие историю, не принимали судьбоносных решений (будь то инвестиции, браки или войны) без анализа звездного неба."
                            </p>
                            <p>
                                "Это не развлекательный гороскоп из глянцевого журнала, где «все Скорпионы сегодня разбогатеют». Натальная карта — это сложная математическая модель, ваш персональный технический паспорт, рассчитанный по минутам вашего рождения."
                            </p>
                            <p class="font-medium text-foreground/90">
                                "Сегодня этот элитарный инструмент доступен вам. Я использую строгий, доказательный подход без мистики, чтобы дать вам конкретные ответы."
                            </p>
                        </div>
                    </div>
                    <div class="relative bg-background p-6 sm:p-8 rounded-2xl border border-border shadow-xl">
                        <div class="absolute -inset-0.5 bg-gradient-to-br from-gold/30 to-background rounded-2xl blur opacity-30 -z-10"></div>
                        <h3 class="text-xl font-serif text-foreground mb-6 pb-4 border-b border-border/50">
                            "Почувствуйте разницу"
                        </h3>
                        <div class="grid sm:grid-cols-2 gap-6 sm:gap-8">
                            <div>
                                <div class="text-muted-foreground/50 text-sm uppercase tracking-wider mb-3">
                                    "Гороскоп из сети"
                                </div>
                                <ul class="space-y-3">
                                    <li class="flex items-start gap-2 text-sm text-muted-foreground">
                                        <span class="text-red-900/70 mt-0.5">"✕"</span>
                                        <span>"Один прогноз на миллион человек"</span>
                                    </li>
                                    <li class="flex items-start gap-2 text-sm text-muted-foreground">
                                        <span class="text-red-900/70 mt-0.5">"✕"</span>
                                        <span>"Общие, размытые фразы"</span>
                                    </li>
                                    <li class="flex items-start gap-2 text-sm text-muted-foreground">
                                        <span class="text-red-900/70 mt-0.5">"✕"</span>
                                        <span>"Никак не решает вашу проблему"</span>
                                    </li>
                                </ul>
                            </div>
                            <div>
                                <div class="text-gold text-sm uppercase tracking-wider mb-3 font-medium">
                                    "Мой разбор"
                                </div>
                                <ul class="space-y-3">
                                    <li class="flex items-start gap-2 text-sm text-foreground/90">
                                        <span class="text-gold mt-0.5">"✓"</span>
                                        <span>
                                            "Расчет с точностью до минут рождения"
                                        </span>
                                    </li>
                                    <li class="flex items-start gap-2 text-sm text-foreground/90">
                                        <span class="text-gold mt-0.5">"✓"</span>
                                        <span>"Анализ конкретно вашей ситуации"</span>
                                    </li>
                                    <li class="flex items-start gap-2 text-sm text-foreground/90">
                                        <span class="text-gold mt-0.5">"✓"</span>
                                        <span>"Пошаговый план выхода из кризиса"</span>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="text-center mb-12">
                    <h2 class="text-3xl md:text-4xl font-serif text-foreground">
                        "Как мы можем поработать?"
                    </h2>
                    <p class="mt-4 text-muted-foreground">
                        "Два формата для вашего удобства. Содержание и глубина анализа одинаковы."
                    </p>
                </div>
                <div class="grid md:grid-cols-2 gap-6 lg:gap-8 max-w-7xl mx-auto">
                    <div class="bg-card/40 p-6 sm:p-8 rounded-2xl border border-border flex flex-col h-full hover:border-gold/30 transition-colors">
                        <div class="w-12 h-12 rounded-xl bg-background border border-border flex items-center justify-center text-gold mb-6 shrink-0">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.5"
                                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                />
                            </svg>
                        </div>
                        <h3 class="text-2xl font-serif text-foreground mb-3">"Письменный PDF-отчет"</h3>
                        <p class="text-muted-foreground text-sm leading-relaxed mb-6 flex-grow">
                            "Идеально для тех, кто любит вдумчиво изучать информацию. Вы получите подробный файл на 15–20 страниц с детальным разбором вашего запроса. К этому документу можно возвращаться годами, перечитывая важные моменты."
                        </p>
                        <a
                            href="#lead-form"
                            class="inline-flex items-center text-gold text-sm font-medium hover:text-gold-soft transition-colors group"
                        >
                            "Запросить условия"
                            <span class="ml-2 transition-transform group-hover:translate-x-1">"→"</span>
                        </a>
                    </div>
                    <div class="bg-gradient-to-br from-gold/10 to-transparent p-6 sm:p-8 rounded-2xl border border-gold/30 flex flex-col h-full relative overflow-hidden">
                        <div class="absolute -top-16 -right-16 w-48 h-48 bg-gold/20 rounded-full blur-[60px] pointer-events-none"></div>
                        <div class="w-12 h-12 rounded-xl bg-background border border-gold/30 flex items-center justify-center text-gold mb-6 shrink-0 relative z-10">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.5"
                                    d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
                                />
                            </svg>
                        </div>
                        <h3 class="text-2xl font-serif text-foreground mb-3 relative z-10">
                            "Живая онлайн-встреча"
                        </h3>
                        <p class="text-muted-foreground text-sm leading-relaxed mb-6 flex-grow relative z-10">
                            "Сессия в Telegram/WhatsApp на 60–90 минут. Мы вживую разберем вашу карту. Формат диалога позволяет сразу задавать уточняющие вопросы и копать глубже в суть проблемы. Аудиозапись встречи останется у вас."
                        </p>
                        <a
                            href="#lead-form"
                            class="inline-flex items-center text-gold text-sm font-medium hover:text-gold-soft transition-colors group relative z-10"
                        >
                            "Запросить условия"
                            <span class="ml-2 transition-transform group-hover:translate-x-1">"→"</span>
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}