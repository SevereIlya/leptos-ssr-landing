use leptos::prelude::*;
use crate::adapters::leptos::components::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <Header />
        <main class="flex flex-col w-full text-white min-h-screen">
            <HeroSection />
            <PainsSection />
            <AboutMethodSection />
            <AboutExpertSection />
            <ReviewsSection />
            <FaqSection />
            <FinalCtaSection />
        </main>
        <Footer />
        <CookieBanner />
    }
}