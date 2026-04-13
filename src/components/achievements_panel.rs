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

    view! {
        {move || {
            if open.get() {
                Some(view! {
                    <div 
                        class="fixed inset-0 bg-black bg-opacity-80 z-50 flex items-center justify-center" 
                        on:click=move |_| open.set(false)
                    >
                        <div 
                            class="bg-bg text-text w-11/12 max-w-2xl max-h-screen overflow-y-auto rounded-xl shadow-2xl" 
                            on:click=|e| e.stop_propagation()
                        >
                            <div class="sticky top-0 bg-bg-dark text-text-inv p-5 flex justify-between items-center border-b border-border">
                                <div>
                                    <h2 class="font-serif text-2xl font-bold">
                                        "🏆 Achievements"
                                    </h2>
                                    <p class="text-sm text-muted">
                                        {unlocked_count} / {total_count} unlocked
                                    </p>
                                </div>
                                <button 
                                    class="text-2xl hover:text-red transition-colors" 
                                    on:click=move |_| open.set(false)
                                >
                                    "✕"
                                </button>
                            </div>

                            <div class="p-5 grid grid-cols-1 md:grid-cols-2 gap-3">
                                {all_achievements.iter().map(|ach| {
                                    let is_unlocked = storage.unlocked.get().contains(ach);
                                    let info = AchievementInfo::get(ach);
                                    let container_class = if is_unlocked {
                                        "p-4 rounded-lg border transition-all bg-bg-dark bg-opacity-20 border-red border-opacity-50"
                                    } else {
                                        "p-4 rounded-lg border transition-all bg-bg-dark bg-opacity-5 border-border opacity-60"
                                    };
                                    
                                    view! {
                                        <div class=container_class>
                                            <div class="flex items-center gap-3">
                                                <span class="text-3xl">
                                                    {info.icon}
                                                </span>
                                                <div class="flex-1">
                                                    <div class="font-bold flex items-center gap-2">
                                                        {info.name}
                                                        {if info.secret && !is_unlocked {
                                                            view! { 
                                                                <span class="text-xs text-muted">
                                                                    "🔒 SECRET"
                                                                </span> 
                                                            }.into_any()
                                                        } else {
                                                            view! {}.into_any()
                                                        }}
                                                    </div>
                                                    <div class="text-xs text-muted">
                                                        {info.description}
                                                    </div>
                                                </div>
                                                {if is_unlocked {
                                                    view! { 
                                                        <span class="text-2xl">
                                                            "✅"
                                                        </span> 
                                                    }.into_any()
                                                } else {
                                                    view! { 
                                                        <span class="text-2xl opacity-30">
                                                            "❓"
                                                        </span> 
                                                    }.into_any()
                                                }}
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