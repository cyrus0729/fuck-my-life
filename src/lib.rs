
// src/data.rs
use break_infinity::Decimal;
use egui::*;
mod window;
mod global_funcs;
mod wawa_containment;

pub const MENUS: [&str; 3] = [
    "challenges",
    "achievements",
    "options",
];

pub const CONTAINMENT_BOUNDS: Vec2 = vec2(500.,500.);
pub const WAWA_SIZE: egui::Vec2 = vec2(50.,50.);

#[allow(dead_code)]

pub struct Wawa {
    cash: Decimal,
    wawas_clicked: Decimal,
    max_wawas: usize,
    wawapos: Vec<((f32, f32),(f32,f32))>, //pos x,y and vel x,y
    click_pos: Option<(f32,f32)>,
    click_timestamp: Option<std::time::Instant>,
    upgrades: [&'static str; 4],
    upgradenames: [&'static str; 4],
    upgradeprices: [Decimal; 4],
    upgradecounts: [Decimal; 4],
    upgrade_buttons: Vec<Ui>,
    background: &'static str,
}

pub struct Save {
    
}

impl Default for Wawa {
    fn default() -> Self {
        Self { 
            cash: Decimal::new(0.0), 
            wawas_clicked: Decimal::new(0.0),
            max_wawas: 10, 
            click_pos: Default::default(),
            click_timestamp: Default::default(),
            wawapos: Default::default(),
            upgrades: [
                "click_radius","wawa_cash","",""
            ],
            upgradenames: [
                "Increase click radius (+0.05)",
                "Cash from wawa (+1)",
                "",
                ""
            ],
            upgradeprices: [Decimal::new(1.0); 4],
            upgradecounts: [Decimal::new(0.0); 4],
            upgrade_buttons: Default::default(),
            background: "bytes://../assets/backgrounds/space.jpg",
        }
    }
}
