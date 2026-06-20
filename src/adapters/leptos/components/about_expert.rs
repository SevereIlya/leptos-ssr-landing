use leptos::prelude::*;

#[component]
pub fn AboutExpertSection() -> impl IntoView {
    view! {
        <section id="about" class="py-20 md:py-32 bg-background border-t border-border/30 overflow-hidden">
            <div class="max-w-7xl mx-auto px-4 sm:px-6">
                <div class="grid lg:grid-cols-2 gap-12 lg:gap-20 items-center">
                    <div class="relative order-2 lg:order-1">
                        <div class="absolute -inset-4 bg-gradient-to-tr from-gold/20 to-transparent rounded-[2rem] blur-2xl -z-10 opacity-50"></div>
                        <div class="relative rounded-2xl p-2 border border-gold/20 bg-background/50 backdrop-blur-sm">
                            <div class="aspect-[4/5] sm:aspect-square lg:aspect-[4/5] rounded-xl overflow-hidden relative">
                                <img
                                    src="/images/expert-portrait.jpg"
                                    alt="Астролог"
                                    class="w-full h-full object-cover object-top filter contrast-125 transition-all duration-700"
                                />
                                <div class="absolute inset-0 bg-gradient-to-t from-background via-transparent to-transparent opacity-80"></div>
                            </div>
                        </div>
                        <div class="absolute -bottom-6 -right-4 sm:right-8 bg-card border border-gold/30 rounded-xl p-4 sm:p-5 shadow-2xl backdrop-blur-md max-w-[200px]">
                            <p class="font-serif text-sm sm:text-base text-gold leading-tight mb-1">
                                "Без мистики и хрустальных шаров"
                            </p>
                        </div>
                    </div>
                    <div class="order-1 lg:order-2">
                        <p class="mb-4 inline-flex items-center gap-2 rounded-full border border-gold/30 bg-gold/5 px-4 py-1.5 text-xs uppercase tracking-[0.2em] text-gold">
                            <span class="text-[10px]">"✦"</span>
                            "Давайте знакомиться"
                        </p>
                        <h2 class="text-3xl sm:text-4xl md:text-5xl font-serif text-foreground leading-[1.1] text-balance mb-6">
                            "Астрология — это "
                            <span class="text-gradient-gold italic pr-2">"инструмент расчетов,"</span>
                            <br class="hidden sm:block" /> "а не магия."
                        </h2>
                        <div class="space-y-4 text-muted-foreground leading-relaxed text-sm sm:text-base mb-8">
                            <p>
                                "Многие боятся идти к астрологам, ожидая услышать фаталистичные предсказания или пространные рассуждения о «ретроградном Меркурии». Я работаю иначе."
                            </p>
                            <p>
                                "В моем арсенале нет гаданий. Только строгий математический анализ вашей натальной карты, основанный на методике "
                                <span class="text-foreground font-medium">
                                    "Высшей Школы Астрологии Павла Андреева"
                                </span>
                                ". Эта школа славится логичным, доказательным подходом, который легко переложить на реальную жизнь."
                            </p>
                            <p>
                                "Моя задача — перевести язык звезд на понятный вам язык действий. Показать, где ваш ресурс, почему буксуют финансы и как именно вам нужно действовать, чтобы выйти из тупика."
                            </p>
                        </div>
                        <div class="grid sm:grid-cols-2 gap-4 border-t border-border/50 pt-8">
                            <div class="flex flex-col gap-1">
                                <span class="text-gold font-serif text-xl">"Высшая Школа"</span>
                                <span class="text-xs uppercase tracking-wider text-muted-foreground font-medium">
                                    "Фундаментальное образование"
                                </span>
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-gold font-serif text-xl">"Логика"</span>
                                <span class="text-xs uppercase tracking-wider text-muted-foreground font-medium">
                                    "Доказательный подход"
                                </span>
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-gold font-serif text-xl">"Практика"</span>
                                <span class="text-xs uppercase tracking-wider text-muted-foreground font-medium">
                                    "Применимо в реальной жизни"
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
