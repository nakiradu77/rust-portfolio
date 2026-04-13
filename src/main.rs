// src/main.rs — Maxime Loukhal Portfolio × Leptos 0.7

mod content;
mod gsap;
mod components;
mod easter_eggs;

use leptos::prelude::*;
use leptos_meta::*;
use wasm_bindgen::JsCast;
use easter_eggs::{AchievementStorage, AchievementNotifier, EasterEggManager};
use components::achievements_panel::AchievementsPanel;

use components::{
    splash::Splash,
    header::Header,
    ticker::Ticker,
    hero::Hero,
    projects::Projects,
    philosophy::Philosophy,
    experience::Experience,
    skills::Skills,
    recognition::Recognition,
    insights::Insights,
    contact::Contact,
};

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    // ─── EASTER EGGS & ACHIEVEMENTS INITIALIZATION ─────────────────────────────
    let storage = AchievementStorage::new();
    let notifier = AchievementNotifier::new();
    let egg_manager = EasterEggManager::new(storage.clone(), notifier.clone());

    provide_context(storage.clone());
    provide_context(notifier.clone());
    provide_context(egg_manager.clone());

    let show_achievements = RwSignal::new(false);
    egg_manager.init();

    // ─── SCROLL LISTENER ─────────────────────────────────────────────────────
    let egg_manager_scroll = egg_manager.clone();
    let scroll_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let scroll_y = web_sys::window()
            .unwrap()
            .scroll_y()
            .unwrap_or(0.0);
        egg_manager_scroll.on_scroll(scroll_y);
    }) as Box<dyn FnMut()>);

    let _ = web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("scroll", scroll_callback.as_ref().unchecked_ref());
    scroll_callback.forget();

    // ─── HOVER LISTENER FOR CuriousMind ACHIEVEMENT ──────────────────────────
    let egg_manager_hover = egg_manager.clone();
    let hover_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
        let target = e.target().unwrap();
        let element = target.unchecked_into::<web_sys::Element>();

        // Correction : element.id() retourne directement un String, pas un Option
        let id = element.id();
        if !id.is_empty() {
            egg_manager_hover.on_hover(&id);
        }
    }) as Box<dyn FnMut(_)>);

    if let Some(document) = web_sys::window().unwrap().document() {
        if let Ok(elements) = document.query_selector_all(".hover-target") {
            for i in 0..elements.length() {
                if let Some(el) = elements.get(i) {
                    let _ = el.add_event_listener_with_callback("mouseenter", hover_callback.as_ref().unchecked_ref());
                }
            }
        }
    }
    hover_callback.forget();

    // ─── BOOT ANIMATIONS ─────────────────────────────────────────────────────
    Effect::new(|_| {
        let cb = wasm_bindgen::closure::Closure::once(move || {
            gsap::init_lenis();
            gsap::init_cursor();
            gsap::play_splash();
        });
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                200,
            )
            .unwrap();
        cb.forget();
    });

    view! {
        <Title text="Maxime Loukhal — Embedded Software Engineer & Open Source Developer" />
        <Meta name="description" content="Maxime Loukhal — Polyglot Embedded Software Programmer specialize in high-throughput, memory-safe architectures." />

        <AchievementsPanel open=show_achievements />

        <button
            class="fixed bottom-5 left-5 z-50 bg-bg-dark text-text-inv p-3 rounded-full shadow-lg hover:scale-110 transition-transform"
            on:click=move |_| show_achievements.set(true)
        >
            "🏆 " {move || storage.unlocked.get().len()}
        </button>

        <Splash />

        <main id="smooth-wrapper">
            <Header />
            <Ticker />
            <Hero />
            <Projects />
            <Ticker reverse=true />
            <Philosophy />
            <Experience />
            <Skills />
            <Recognition />
            <Insights />
            <Contact />
        </main>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Warn);
    console_error_panic_hook::set_once();
    mount_to_body(App);
}