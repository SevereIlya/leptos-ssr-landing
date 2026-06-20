use crate::adapters::leptos::actions::*;
use crate::adapters::leptos::analytics::*;
use crate::adapters::leptos::components::*;
use crate::constants::*;
use leptos::prelude::*;

#[component]
pub fn FinalCtaSection() -> impl IntoView {
    let create_lead = ServerAction::<CreateLeadServerFn>::new();
    let value = create_lead.value();

    let (show_modal, set_show_modal) = signal(false);
    let (consent, set_consent) = signal(false);
    let (contact_method, set_contact_method) = signal("telegram".to_string());

    Effect::new(move |_| {
        if let Some(Ok(_)) = value.get() {
            track_goal("lead_form_question");
        }
    });

    view! {
        <Section id="final" class="!pt-4 md:!pt-8 !border-none">
            <Container>
                <div class="relative bg-card/40 border border-border/50 rounded-3xl p-6 sm:p-10 md:p-12 overflow-hidden">
                    <div class="absolute -top-24 -right-24 w-96 h-96 bg-gold/10 rounded-full blur-[80px] pointer-events-none"></div>
                    <div class="grid lg:grid-cols-2 gap-12 lg:gap-8 items-center relative z-10">
                        <div>
                            <h2 class="text-3xl sm:text-4xl font-serif text-foreground leading-[1.1] mb-4">
                                "Остались вопросы? " <br />
                                <span class="text-gradient-gold">"Нужна помощь с форматом?"</span>
                            </h2>
                            <p class="text-muted-foreground text-sm sm:text-base leading-relaxed mb-8 max-w-md">
                                "Каждая ситуация уникальна. Напишите мне лично, и мы подберем комфортный формат разбора именно под ваш запрос. Без навязывания."
                            </p>
                            <div class="flex flex-col sm:flex-row gap-4 items-start sm:items-center mb-6">
                                <a
                                    href=TELEGRAM_MANAGER_URL
                                    target="_blank"
                                    rel="nofollow noopener noreferrer"
                                    class="inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl bg-[#2AABEE] hover:bg-[#2298D6] text-white font-medium transition-all duration-300 hover:scale-[1.02] shadow-lg shadow-[#2AABEE]/20 w-full sm:w-auto"
                                >
                                    <TelegramIcon class="w-5 h-5 fill-current" />
                                    "Написать в Telegram"
                                </a>
                                <button
                                    on:click=move |_| set_show_modal.set(true)
                                    class="inline-flex justify-center px-6 py-3.5 rounded-xl bg-background border border-border text-foreground font-medium hover:border-gold/50 hover:text-gold transition-colors duration-300 w-full sm:w-auto cursor-pointer"
                                >
                                    "Задать вопрос"
                                </button>
                            </div>
                            <div class="flex items-center gap-2 text-xs sm:text-sm text-emerald-400/90 font-medium bg-emerald-400/10 w-fit px-3 py-1.5 rounded-full border border-emerald-400/20">
                                <div class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></div>
                                "Служба заботы online (отвечаем за 15 минут)"
                            </div>
                        </div>
                        <div class="flex flex-col gap-4 max-w-md mx-auto lg:mx-0 lg:ml-auto w-full">
                            <div class="animate-float self-start bg-background border border-border/60 p-4 sm:p-5 rounded-2xl rounded-tl-sm shadow-sm max-w-[90%]">
                                <p class="text-sm text-muted-foreground leading-relaxed">
                                    "А если я не знаю точного времени рождения? Сможем ли мы сделать разбор?"
                                </p>
                            </div>
                            <div class="animate-float-delayed self-end bg-gradient-to-br from-gold/10 to-gold/5 border border-gold/30 p-4 sm:p-5 rounded-2xl rounded-br-sm shadow-lg shadow-gold/5 max-w-[90%] relative">
                                <div class="absolute -bottom-2 -right-2 w-6 h-6 rounded-full bg-gold border-2 border-background flex items-center justify-center text-[10px]">
                                    "✦"
                                </div>
                                <p class="text-sm text-foreground/90 leading-relaxed">
                                    "Конечно! ✨ Для этого существует процедура ректификации. По ключевым событиям из вашего прошлого я восстановлю время рождения до минут."
                                </p>
                            </div>
                            <div class="animate-float-fast self-start bg-background border border-border/60 p-4 rounded-2xl rounded-tl-sm shadow-sm opacity-80 max-w-[80%] mt-2">
                                <p class="text-sm text-muted-foreground">
                                    "Отлично, а возможна рассрочка?"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </Container>
        </Section>

        <Show when=move || show_modal.get()>
            <div
                class="fixed inset-0 z-[100] bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
                on:click=move |_| set_show_modal.set(false)
            >
                <div
                    class="relative bg-card border border-border/80 w-full max-w-3xl rounded-2xl p-6 sm:p-8 shadow-2xl"
                    on:click=move |ev| ev.stop_propagation()
                >
                    <button
                        on:click=move |_| set_show_modal.set(false)
                        class="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full bg-background border border-border text-muted-foreground hover:text-gold hover:border-gold/50 transition-colors cursor-pointer"
                    >
                        "✕"
                    </button>
                    <Show
                        when=move || value.get().is_some() && value.get().unwrap().is_ok()
                        fallback=move || {
                            view! {
                                <h3 class="text-2xl font-serif text-gold mb-6 text-center">
                                    "Задать вопрос"
                                </h3>
                                <ActionForm action=create_lead>
                                    <div class="flex flex-col gap-5 w-full">
                                        <Field label="Что вы хотели бы уточнить?">
                                            <textarea
                                                name="message"
                                                class=format!("{} resize-none min-h-[180px]", INPUT_CLASSES)
                                                placeholder="Опишите вашу ситуацию или задайте вопрос..."
                                            ></textarea>
                                        </Field>
                                        <Field label="Как с вами связаться?">
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
                                        <input type="hidden" name="name" value="Вопрос из FAQ" />
                                        <input type="hidden" name="source" value="FAQ | Модалка" />
                                        <input type="hidden" name="consent" value=move || consent.get().to_string() />
                                        <ConsentSubmit
                                            consent=consent
                                            set_consent=set_consent
                                            button_text="Отправить вопрос"
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
                            }
                        }
                    >
                        <div class="flex flex-col items-center justify-center text-center py-8 animate-in fade-in zoom-in duration-500">
                            <div class="w-20 h-20 bg-gold/10 rounded-full flex items-center justify-center text-gold text-4xl mb-6 shadow-[0_0_30px_rgba(212,175,55,0.2)]">
                                "✓"
                            </div>
                            <h3 class="text-3xl font-serif text-foreground mb-4">
                                "Вопрос успешно отправлен!"
                            </h3>
                            <p class="text-muted-foreground leading-relaxed max-w-md mb-8">
                                "Мой ассистент свяжется с вами в течение 15 минут в выбранном мессенджере. А пока можете подписаться на наш Telegram-канал."
                            </p>
                            <a
                                href=TELEGRAM_GROUP_URL
                                target="_blank"
                                class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-surface-1 border border-border hover:border-gold/50 hover:text-gold transition-colors"
                            >
                                <TelegramIcon class="w-5 h-5" />
                                "Перейти в канал"
                            </a>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}
