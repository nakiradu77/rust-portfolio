// src/components/achievements_panel.rs
use leptos::prelude::*;
use crate::easter_eggs::{AchievementStorage, AchievementInfo, Achievement};

#[component]
pub fn AchievementsPanel(open: RwSignal<bool>) -> impl IntoView {
    let storage = expect_context::<AchievementStorage>();
    let all_achievements = vec![
        Achievement::ScrollMaster,
        Achievement::SpeedReader,
        Achievement::DepthExplorer,
        Achievement::ClickAddict,
        Achievement::SocialButterfly,
        Achievement::CuriousMind,
        Achievement::KonamiCode,
        Achievement::SecretClick,
        Achievement::ConsoleExplorer,
        Achievement::TripleClick,
        Achievement::DevMode,
        Achievement::EasterDate,
        Achievement::NightOwl,
        Achievement::WeekendWarrior,
        Achievement::Completionist,
    ];

    let unlocked_count = move || storage.unlocked.get().len();
    let total_count = all_achievements.len();

    // Effet pour bloquer le scroll du body quand le panel est ouvert
    Effect::new(move || {
        let is_open = open.get();
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let body = document.body().unwrap();
            if is_open {
                // Sauvegarder le scroll position et bloquer
                let _ = body.style().set_property("overflow", "hidden");
                let _ = body.style().set_property("position", "fixed");
                let _ = body.style().set_property("width", "100%");
            } else {
                // Restaurer le scroll
                let _ = body.style().set_property("overflow", "");
                let _ = body.style().set_property("position", "");
                let _ = body.style().set_property("width", "");
            }
        }
    });

    view! {
        {move || {
            if open.get() {
                Some(view! {
                    <div
                        class="achievements-panel fixed inset-0 bg-black/80 z-50 flex items-center justify-center"
                        on:click=move |_| open.set(false)
                    >
                        <div
                            class="achievements-panel-container bg-bg text-text w-11/12 max-w-2xl max-h-[85vh] md:max-h-[80vh] overflow-y-auto rounded-t-2xl md:rounded-xl shadow-2xl"
                            on:click=|e| e.stop_propagation()
                        >
                            {/* Header sticky avec la croix toujours visible */}
                            <div class="achievements-panel-header sticky top-0 bg-bg-dark text-text-inv p-4 md:p-5 flex justify-between items-center border-b border-border z-10">
                                <div>
                                    <h2 class="font-serif text-xl md:text-2xl font-bold">
                                        "🏆 Achievements"
                                    </h2>
                                    <p class="text-xs md:text-sm text-muted">
                                        {unlocked_count} / {total_count} unlocked
                                    </p>
                                </div>
                                <button
                                    class="achievements-close-btn text-2xl md:text-2xl hover:text-red transition-colors w-10 h-10 flex items-center justify-center rounded-full hover:bg-white/10"
                                    on:click=move |_| open.set(false)
                                >
                                    "✕"
                                </button>
                            </div>

                            {/* Grille des achievements */}
                            <div class="achievements-grid p-4 md:p-5 grid grid-cols-1 gap-3 md:gap-3">
                                {all_achievements.iter().map(|ach| {
                                    let is_unlocked = storage.unlocked.get().contains(ach);
                                    let info = AchievementInfo::get(ach);
                                    let container_class = if is_unlocked {
                                        "achievement-card unlocked p-3 md:p-4 rounded-lg border transition-all bg-bg-dark/20 border-red/50"
                                    } else {
                                        "achievement-card locked p-3 md:p-4 rounded-lg border transition-all bg-bg-dark/5 border-border opacity-60"
                                    };

                                    view! {
                                        <div class=container_class>
                                            <div class="flex items-center gap-3">
                                                <span class="achievement-icon text-2xl md:text-3xl">
                                                    {info.icon}
                                                </span>
                                                <div class="achievement-info flex-1">
                                                    <div class="achievement-title font-bold text-sm md:text-base flex flex-wrap items-center gap-2">
                                                        {info.name}
                                                       <div style="font-size: 0.75rem; color: var(--color-muted, #8a8577);">
                                                        {if info.secret && !is_unlocked {
                                                            view! {
                                                                <span style="filter: blur(4px); display: inline-block; cursor: help;" title="Débloquez ce succès pour révéler la description">
                                                                    {info.description}
                                                                </span>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <span>{info.description}</span>
                                                            }.into_any()
                                                        }}
                                                        </div>
                                                    </div>
                                                </div>
                                                <div class="achievement-status">
                                                    {if is_unlocked {
                                                        view! {
                                                            <span class="unlocked-badge text-xl md:text-2xl">
                                                                "✅"
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <span class="locked-badge text-xl md:text-2xl opacity-30">
                                                                "❓"
                                                            </span>
                                                        }.into_any()
                                                    }}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                })
            } else {
                None
            }
        }}
    }
}