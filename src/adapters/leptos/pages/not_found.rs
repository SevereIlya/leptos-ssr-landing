use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-4">
            <h1 class="text-6xl font-bold text-red-500 mb-4">"404"</h1>
            <p class="text-xl">"Страница не найдена"</p>
            <a href="/" class="mt-8 text-purple-400 hover:text-purple-300 underline">
                "Вернуться на главную страницу"
            </a>
        </main>
    }
}