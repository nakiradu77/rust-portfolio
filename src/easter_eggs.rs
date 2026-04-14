// src/easter_eggs.rs
use leptos::prelude::*;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
// ─── CONSTANTES ────────────────────────────────────────────────────────
const KONAMI_SEQUENCE: [&str; 10] = [
    "ArrowUp", "ArrowUp", "ArrowDown", "ArrowDown",
    "ArrowLeft", "ArrowRight", "ArrowLeft", "ArrowRight",
    "KeyB", "KeyA"
];

const KONAMI_SEQUENCE_ALT: [&str; 10] = [
    "ArrowUp", "ArrowUp", "ArrowDown", "ArrowDown",
    "ArrowLeft", "ArrowRight", "ArrowLeft", "ArrowRight",
    "b", "a"
];

// Liste de tous les achievements (pour Completionist)
const ALL_ACHIEVEMENTS: [Achievement; 14] = [
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
];

// ─── TYPES ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Achievement {
    // Explorer
    ScrollMaster,
    SpeedReader,
    DepthExplorer,

    // Interactions
    ClickAddict,
    SocialButterfly,
    CuriousMind,

    // Hidden / Easter Eggs
    KonamiCode,
    SecretClick,
    ConsoleExplorer,
    TripleClick,
    DevMode,
    EasterDate,

    // Time based
    NightOwl,
    WeekendWarrior,

    // Completionist
    Completionist,
}

impl std::fmt::Display for Achievement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct AchievementInfo {
    pub id: Achievement,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub secret: bool,
}

impl AchievementInfo {
    pub fn get(ach: &Achievement) -> Self {
        match ach {
            Achievement::ScrollMaster => Self {
                id: ach.clone(),
                name: "Scroll Master",
                description: "Scrolled more than 1000 pixels",
                icon: "📜",
                secret: false,
            },
            Achievement::SpeedReader => Self {
                id: ach.clone(),
                name: "Speed Reader",
                description: "Visited all portfolio sections",
                icon: "📚",
                secret: false,
            },
            Achievement::DepthExplorer => Self {
                id: ach.clone(),
                name: "Depth Explorer",
                description: "Scrolled more than 3500 pixels",
                icon: "⛏️",
                secret: false,
            },
            Achievement::ClickAddict => Self {
                id: ach.clone(),
                name: "Click Addict",
                description: "Clicked 10 times on links",
                icon: "🖱️",
                secret: false,
            },
            Achievement::SocialButterfly => Self {
                id: ach.clone(),
                name: "Social Butterfly",
                description: "Opened all social links",
                icon: "🦋",
                secret: true,
            },
            Achievement::CuriousMind => Self {
                id: ach.clone(),
                name: "Curious Mind",
                description: "Hovered over 15 interactive elements",
                icon: "🔍",
                secret: true,
            },
            Achievement::KonamiCode => Self {
                id: ach.clone(),
                name: "Konami Legacy",
                description: "↑↑↓↓←→←→BA",
                icon: "🎮",
                secret: true,
            },
            Achievement::SecretClick => Self {
                id: ach.clone(),
                name: "Secret Spot",
                description: "Clicked 5 times on the logo",
                icon: "⭐",
                secret: true,
            },
            Achievement::ConsoleExplorer => Self {
                id: ach.clone(),
                name: "Console Explorer",
                description: "Opened developer tools",
                icon: "🔧",
                secret: true,
            },
            Achievement::TripleClick => Self {
                id: ach.clone(),
                name: "Triple Threat",
                description: "Triple-clicked on footer",
                icon: "👆",
                secret: true,
            },
            Achievement::DevMode => Self {
                id: ach.clone(),
                name: "Dev Mode",
                description: "Pressed F12",
                icon: "💻",
                secret: true,
            },
            Achievement::EasterDate => Self {
                id: ach.clone(),
                name: "Time Traveler",
                description: "Visited on April 1st",
                icon: "🐣",
                secret: true,
            },
            Achievement::NightOwl => Self {
                id: ach.clone(),
                name: "Night Owl",
                description: "Visited between midnight and 6 AM",
                icon: "🦉",
                secret: true,
            },
            Achievement::WeekendWarrior => Self {
                id: ach.clone(),
                name: "Weekend Warrior",
                description: "Visited on weekend",
                icon: "⚔️",
                secret: true,
            },
            Achievement::Completionist => Self {
                id: ach.clone(),
                name: "Completionist",
                description: "Unlocked all achievements!",
                icon: "🏆",
                secret: false,
            },
        }
    }
}

// ─── STORAGE MANAGER (via localStorage web_sys) ───────────────────────

#[derive(Clone)]
pub struct AchievementStorage {
    pub unlocked: RwSignal<HashSet<Achievement>>,
}

impl AchievementStorage {
    pub fn new() -> Self {
        let unlocked = RwSignal::new(Self::load_from_storage());
        Self { unlocked }
    }

    fn get_storage() -> Option<web_sys::Storage> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
    }

    fn load_from_storage() -> HashSet<Achievement> {
        if let Some(storage) = Self::get_storage() {
            if let Ok(Some(data)) = storage.get_item("portfolio_achievements") {
                let mut unlocked = HashSet::new();
                for name in data.split(',') {
                    match name.trim() {
                        "ScrollMaster" => { unlocked.insert(Achievement::ScrollMaster); }
                        "SpeedReader" => { unlocked.insert(Achievement::SpeedReader); }
                        "DepthExplorer" => { unlocked.insert(Achievement::DepthExplorer); }
                        "ClickAddict" => { unlocked.insert(Achievement::ClickAddict); }
                        "SocialButterfly" => { unlocked.insert(Achievement::SocialButterfly); }
                        "CuriousMind" => { unlocked.insert(Achievement::CuriousMind); }
                        "KonamiCode" => { unlocked.insert(Achievement::KonamiCode); }
                        "SecretClick" => { unlocked.insert(Achievement::SecretClick); }
                        "ConsoleExplorer" => { unlocked.insert(Achievement::ConsoleExplorer); }
                        "TripleClick" => { unlocked.insert(Achievement::TripleClick); }
                        "DevMode" => { unlocked.insert(Achievement::DevMode); }
                        "EasterDate" => { unlocked.insert(Achievement::EasterDate); }
                        "NightOwl" => { unlocked.insert(Achievement::NightOwl); }
                        "WeekendWarrior" => { unlocked.insert(Achievement::WeekendWarrior); }
                        "Completionist" => { unlocked.insert(Achievement::Completionist); }
                        _ => {}
                    }
                }
                return unlocked;
            }
        }
        HashSet::new()
    }

    pub fn save(&self) {
        if let Some(storage) = Self::get_storage() {
            let ids: Vec<String> = self.unlocked.get()
                .iter()
                .map(|a| format!("{:?}", a))
                .collect();
            let data = ids.join(",");
            let _ = storage.set_item("portfolio_achievements", &data);
        }
    }

    pub fn unlock(&self, ach: Achievement, notifier: &AchievementNotifier) {
        let mut unlocked = self.unlocked.get();
        if !unlocked.contains(&ach) {
            unlocked.insert(ach.clone());
            self.unlocked.set(unlocked);
            self.save();
            notifier.show(ach);

            if ALL_ACHIEVEMENTS.iter().all(|a| self.unlocked.get().contains(a)) {
                let mut completionist_unlocked = self.unlocked.get();
                if !completionist_unlocked.contains(&Achievement::Completionist) {
                    completionist_unlocked.insert(Achievement::Completionist);
                    self.unlocked.set(completionist_unlocked);
                    self.save();
                    notifier.show(Achievement::Completionist);
                }
            }
        }
    }
}

// ─── NOTIFICATION ──────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AchievementNotifier {
    pub current: RwSignal<Option<(Achievement, AchievementInfo)>>,
}

impl AchievementNotifier {
    pub fn new() -> Self {
        Self {
            current: RwSignal::new(None),
        }
    }

    pub fn show(&self, ach: Achievement) {
        let info = AchievementInfo::get(&ach);
        self.current.set(Some((ach, info)));

        // Auto-hide après 4 secondes - Version sans async
        let current = self.current.clone();
        let window = web_sys::window().unwrap();

        let callback = Closure::wrap(Box::new(move || {
            current.set(None);
        }) as Box<dyn FnMut()>);

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                4000,
            )
            .unwrap();

        callback.forget();
    }
}

// ─── EASTER EGG MANAGER ───────────────────────────────────────────────

#[derive(Clone)]
pub struct EasterEggManager {
    storage: AchievementStorage,
    notifier: AchievementNotifier,
    konami_index: RwSignal<usize>,

    // Counters
    logo_clicks: RwSignal<u32>,
    hovered_elements: RwSignal<HashSet<String>>,
    click_count: RwSignal<u32>,
    social_clicks: RwSignal<HashSet<String>>,
    sections_visited: RwSignal<HashSet<String>>,
}

impl EasterEggManager {
    pub fn new(storage: AchievementStorage, notifier: AchievementNotifier) -> Self {
        Self {
            storage,
            notifier,
            konami_index: RwSignal::new(0),
            logo_clicks: RwSignal::new(0),
            hovered_elements: RwSignal::new(HashSet::new()),
            click_count: RwSignal::new(0),
            social_clicks: RwSignal::new(HashSet::new()),
            sections_visited: RwSignal::new(HashSet::new()),
        }
    }

    pub fn init(&self) {
        self.check_time_achievements();
        self.init_konami_listener();
        self.init_dev_tools_listener();
    }

    fn check_time_achievements(&self) {
        let now = js_sys::Date::new_0();
        let hours = now.get_hours();

        if hours < 6 {
            self.storage.unlock(Achievement::NightOwl, &self.notifier);
        }

        let day = now.get_day();
        if day == 0 || day == 6 {
            self.storage.unlock(Achievement::WeekendWarrior, &self.notifier);
        }

        let month = now.get_month();
        let date = now.get_date();
        if month == 3 && date == 1 {
            self.storage.unlock(Achievement::EasterDate, &self.notifier);
        }
    }

    fn init_konami_listener(&self) {
        let manager = self.clone();
        let callback = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let code = e.code();
            let key = e.key();

            web_sys::console::log_1(&format!("code: {}, key: {}", code, key).into());

            // Accepter les deux formats
            let idx = manager.konami_index.get();

            let is_match = if idx < KONAMI_SEQUENCE.len() {
                code == KONAMI_SEQUENCE[idx] || key == KONAMI_SEQUENCE_ALT[idx]
            } else {
                false
            };

            if is_match {
                web_sys::console::log_1(&format!("Match at index {}", idx).into());
                if idx + 1 == KONAMI_SEQUENCE.len() {
                    web_sys::console::log_1(&"🎉 KONAMI CODE UNLOCKED! 🎉".into());
                    manager.storage.unlock(Achievement::KonamiCode, &manager.notifier);
                    manager.konami_index.set(0);
                } else {
                    manager.konami_index.set(idx + 1);
                }
            } else {
                // Ne reset que si la touche n'est pas une direction
                let is_direction = code == "ArrowUp" || code == "ArrowDown" || code == "ArrowLeft" || code == "ArrowRight";
                if !is_direction {
                    manager.konami_index.set(0);
                }
            }
        });

        let _ = web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());

        callback.forget();
    }

    fn init_dev_tools_listener(&self) {
        let manager = self.clone();

        let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            // CORRECTION : Stocker les valeurs dans des variables locales
            let code = e.code().to_string();
            let ctrl = e.ctrl_key();
            let shift = e.shift_key();

            if code == "F12" || (ctrl && shift && code == "KeyI") {
                manager.storage.unlock(Achievement::DevMode, &manager.notifier);
                manager.storage.unlock(Achievement::ConsoleExplorer, &manager.notifier);
            }
        }) as Box<dyn FnMut(_)>);

        let _ = web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());

        callback.forget();

        // Detect console via resize
        let manager2 = self.clone();
        let resize_callback = Closure::wrap(Box::new(move || {
            let window = web_sys::window().unwrap();
            let outer = window.outer_width().unwrap().as_f64().unwrap();
            let inner = window.inner_width().unwrap().as_f64().unwrap();
            if outer - inner > 100.0 {
                manager2.storage.unlock(Achievement::ConsoleExplorer, &manager2.notifier);
            }
        }) as Box<dyn FnMut()>);

        let _ = web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref());

        resize_callback.forget();
    }

    pub fn on_logo_click(&self) {
        let clicks = self.logo_clicks.get() + 1;
        self.logo_clicks.set(clicks);

        if clicks == 5 {
            self.storage.unlock(Achievement::SecretClick, &self.notifier);
        }
    }

    pub fn on_link_click(&self, link_type: &str) {
        let clicks = self.click_count.get() + 1;
        self.click_count.set(clicks);

        if clicks >= 10 {
            self.storage.unlock(Achievement::ClickAddict, &self.notifier);
        }

        if link_type == "social" {
            let mut socials = self.social_clicks.get();
            socials.insert(link_type.to_string());
            self.social_clicks.set(socials);

            if self.social_clicks.get().len() >= 3 {
                self.storage.unlock(Achievement::SocialButterfly, &self.notifier);
            }
        }
    }

    pub fn on_section_view(&self, section: &str) {
        let mut sections = self.sections_visited.get();
        sections.insert(section.to_string());
        self.sections_visited.set(sections);

        let required = ["work", "about", "contact"];
        if required.iter().all(|s| self.sections_visited.get().contains(*s)) {
            self.storage.unlock(Achievement::SpeedReader, &self.notifier);
        }
    }

    pub fn on_scroll(&self, scroll_y: f64) {
        if scroll_y > 1000.0 {
            self.storage.unlock(Achievement::ScrollMaster, &self.notifier);
        }
        if scroll_y > 3500.0 {
            self.storage.unlock(Achievement::DepthExplorer, &self.notifier);
        }
    }

    pub fn on_hover(&self, element_id: &str) {
        let mut elements = self.hovered_elements.get();
        elements.insert(element_id.to_string());

        // Vérifier la longueur avant de remettre la valeur
        if elements.len() >= 15 {
            self.storage.unlock(Achievement::CuriousMind, &self.notifier);
        }

        self.hovered_elements.set(elements);
    }

    pub fn on_triple_click(&self) {
        self.storage.unlock(Achievement::TripleClick, &self.notifier);
    }
}