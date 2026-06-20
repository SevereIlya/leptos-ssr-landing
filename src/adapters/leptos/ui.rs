use super::pages::LandingPage;
use super::pages::NotFound;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Астрологический разбор | Путеводная звезда" />
        <Meta
            name="description"
            content="Узнай свое предназначение по натальной карте. Оставь заявку на разбор."
        />

        <Stylesheet id="leptos" href="/pkg/astrology_website.css" />

        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("") view=LandingPage />
            </Routes>
        </Router>
    }
}

#[cfg(feature = "ssr")]
use crate::constants::YANDEX_METRIKA_ID;
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />

                // Яндекс метрика
                <script type="text/javascript">
                    {format!(
                        r#"
                    (function(m,e,t,r,i,k,a){{
                        m[i]=m[i]||function(){{(m[i].a=m[i].a||[]).push(arguments)}};
                        m[i].l=1*new Date();
                        for (var j = 0; j < document.scripts.length; j++) {{if (document.scripts[j].src === r) {{ return; }}}}
                        k=e.createElement(t),a=e.getElementsByTagName(t)[0],k.async=1,k.src=r,a.parentNode.insertBefore(k,a)
                    }})(window, document,'script','https://mc.yandex.ru/metrika/tag.js?id={id}', 'ym');

                    ym({id}, 'init', {{
                        ssr:true,
                        webvisor:true,
                        clickmap:true,
                        ecommerce:"dataLayer",
                        referrer: document.referrer,
                        url: location.href,
                        accurateTrackBounce:true,
                        trackLinks:true
                    }});
                    "#,
                        id = YANDEX_METRIKA_ID,
                    )}
                </script>
                <noscript>
                    <div>
                        <img
                            src=format!("https://mc.yandex.ru/watch/{}", YANDEX_METRIKA_ID)
                            style="position:absolute; left:-9999px;"
                            alt=""
                        />
                    </div>
                </noscript>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
