use crate::adapters::leptos::actions::*;
use crate::adapters::leptos::analytics::*;
use crate::adapters::leptos::components::*;
use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    let create_lead = ServerAction::<CreateLeadServerFn>::new();
    let value = create_lead.value();

    let (consent, set_consent) = signal(false);
    let (contact_method, set_contact_method) = signal("telegram".to_string());

    Effect::new(move |_| {
        if let Some(Ok(_)) = value.get() {
            track_goal("lead_form_hero");
        }
    });

    view! {
        <Section id="hero" class="!py-0 !border-none min-h-dvh flex items-center">
            <img
                src="/images/hero-cosmos.jpg"
                alt=""
                class="absolute inset-0 h-full w-full object-cover opacity-60 z-0"
            />
            <div class="starfield absolute inset-0 z-[1]" />
            <div class="absolute inset-0 z-[5] pointer-events-none bg-[linear-gradient(to_bottom,transparent_40%,var(--color-surface-1)_100%)]" />

            <Container class="pb-20 pt-24 md:pb-32 md:pt-32">
                <div class="grid items-center gap-12 lg:gap-16 lg:grid-cols-12">
                    <div class="lg:col-span-7">
                        <p class="mb-5 inline-block rounded-full border border-gold/30 bg-gold/5 px-4 py-1.5 text-xs uppercase tracking-[0.2em] text-gold">
                            "Королевское искусство"
                        </p>
                        <h1 class="text-2xl sm:text-4xl md:text-5xl xl:text-6xl leading-[1.1] text-balance">
                            "Чувствуете, что жизнь "
                            <span class="text-gradient-gold italic">"зашла в тупик?"</span>
                        </h1>
                        <p class="mt-6 max-w-xl text-base sm:text-lg md:text-xl leading-relaxed text-muted-foreground">
                            "Финансовые трудности, повторяющиеся кризисы или непонимание своего предназначения. Разбор натальной карты поможет расшифровать ваш «личный код» и найти выход."
                        </p>
                        <div class="mt-10 flex flex-col gap-4 text-sm md:text-base text-foreground/90">
                            <div class="flex items-center gap-3">
                                <span class="flex items-center justify-center w-6 h-6 rounded-full bg-gold/20 text-gold text-xs">
                                    "✓"
                                </span>
                                <span>
                                    <span class="font-semibold text-gold-soft">"Авторская"</span>
                                    " методика разбора"
                                </span>
                            </div>
                            <div class="flex items-center gap-3">
                                <span class="flex items-center justify-center w-6 h-6 rounded-full bg-gold/20 text-gold text-xs">
                                    "✓"
                                </span>
                                <span>
                                    "Разбор финансов, профориентации и отношений"
                                </span>
                            </div>
                            <div class="flex items-center gap-3">
                                <span class="flex items-center justify-center w-6 h-6 rounded-full bg-gold/20 text-gold text-xs">
                                    "✓"
                                </span>
                                <span>
                                    "Удобный формат: письменный отчет или аудио-разбор"
                                </span>
                            </div>
                        </div>
                    </div>
                    <div id="lead-form" class="lg:col-span-5 w-full">
                        <Form>
                            <Show
                                when=move || matches!(value.get(), Some(Ok(_)))
                                fallback=move || {
                                    view! {
                                        <div class="p-4 sm:p-6 md:p-8">
                                            <h3 class="text-lg sm:text-2xl lg:text-lg xl:text-2xl text-gold mb-2 font-medium leading-tight">
                                                "Получить условия разбора"
                                            </h3>
                                            <p class="text-sm text-muted-foreground mb-6 leading-relaxed">
                                                "Оставьте контакт, мы расскажем о форматах и ответим на вопросы."
                                            </p>
                                            <ActionForm action=create_lead>
                                                <div class="flex flex-col gap-4 sm:gap-5 w-full">

                                                    <Field label="Как к вам обращаться?">
                                                        <input
                                                            class=INPUT_CLASSES
                                                            name="name"
                                                            placeholder="Ваше имя"
                                                        />
                                                    </Field>
                                                    <Field label="Способ связи">
                                                        <div class="flex flex-col gap-2 min-[430px]:flex-row">
                                                            <select
                                                                name="contact_method"
                                                                class=format!(
                                                                    "{} min-[430px]:w-[130px] cursor-pointer",
                                                                    INPUT_CLASSES,
                                                                )
                                                                on:change=move |ev| {
                                                                    set_contact_method.set(event_target_value(&ev));
                                                                }
                                                            >
                                                                <option value="telegram">"Telegram"</option>
                                                                <option value="whatsapp">"WhatsApp"</option>
                                                                <option value="vk">"ВКонтакте"</option>
                                                                <option value="phone">"Телефон"</option>
                                                                <option value="email">"Почта"</option>
                                                            </select>
                                                            <input
                                                                name="contact_value"
                                                                class=format!("{} w-full", INPUT_CLASSES)
                                                                required
                                                                type=move || {
                                                                    if contact_method.get() == "phone" { "tel" } else { "text" }
                                                                }
                                                                placeholder=move || match contact_method.get().as_str() {
                                                                    "telegram" => "@username или номер",
                                                                    "whatsapp" | "phone" => "+7 (999) 000-00-00",
                                                                    "vk" => "vk.com/id...",
                                                                    "email" => "user@email",
                                                                    _ => "Ваш контакт",
                                                                }
                                                            />
                                                        </div>
                                                    </Field>
                                                    <input
                                                        type="hidden"
                                                        name="source"
                                                        value="Главный экран (Hero)"
                                                    />
                                                    <input type="hidden" name="message" value="" />
                                                    <input
                                                        type="hidden"
                                                        name="consent"
                                                        value=move || consent.get().to_string()
                                                    />
                                                    <ConsentSubmit
                                                        consent=consent
                                                        set_consent=set_consent
                                                        pending=create_lead.pending().into()
                                                        button_text="Хочу разбор"
                                                    />
                                                    <Show when=move || matches!(value.get(), Some(Err(_)))>
                                                        <p class="mt-4 text-red-400 font-medium text-center text-sm p-3 bg-red-400/10 border border-red-400/20 rounded-lg animate-in fade-in slide-in-from-bottom-2 duration-300">
                                                            {move || match value.get() {
                                                                Some(Err(ServerFnError::ServerError(msg))) => msg,
                                                                Some(Err(e)) => e.to_string(),
                                                                _ => String::new(),
                                                            }}
                                                        </p>
                                                    </Show>
                                                </div>
                                            </ActionForm>
                                        </div>
                                    }
                                }
                            >
                                <div class="p-8 md:p-12 flex flex-col items-center justify-center text-center animate-in fade-in zoom-in duration-500 h-full min-h-[400px]">
                                    <div class="w-20 h-20 bg-gold/10 rounded-full flex items-center justify-center text-gold text-4xl mb-6 shadow-[0_0_30px_rgba(212,175,55,0.2)]">
                                        "✦"
                                    </div>
                                    <h3 class="text-2xl sm:text-3xl font-serif text-foreground mb-4">
                                        "Заявка получена"
                                    </h3>
                                    <p class="text-muted-foreground leading-relaxed mb-8">
                                        "Мы свяжемся с вами в течение 15 минут в выбранном мессенджере, чтобы обсудить ваш запрос и форматы работы."
                                    </p>
                                    <div class="w-full h-px bg-border/50 mb-6"></div>
                                    <p class="text-xs text-muted-foreground uppercase tracking-widest">
                                        "Астра Регалис"
                                    </p>
                                </div>
                            </Show>
                        </Form>
                    </div>
                </div>
            </Container>
        </Section>
    }
}
