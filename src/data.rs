
// src/data.rs
use break_infinity::Decimal;

pub struct Wawa {
    pub cash: Decimal,
    pub wawas_clicked: Decimal,
    pub max_wawas: usize,
    pub abswawapos: Vec<((f32, f32),(f32,f32))>, //pos x,y and vel x,y
    pub relwawapos: Vec<((f32, f32),(f32,f32))>, //pos x,y and vel x,y
    pub window_pos: egui::Pos2,
    pub dragging_start: Option<std::time::Instant>,
    pub upgrades: [&'static str; 4],
    pub upgradenames: [&'static str; 4],
    pub upgradeprices: [Decimal; 4],
    pub background: &'static str,
}

impl Default for Wawa {
    fn default() -> Self {
        Self { 
            cash: Decimal::new(0.0), 
            wawas_clicked: Decimal::new(0.0),
            max_wawas: 10, 
            abswawapos: Default::default(),
            relwawapos: Default::default(),
            window_pos: egui::pos2(0.,0.),
            dragging_start: Default::default(),
            upgrades: Default::default(),
            upgradenames: Default::default(),
            upgradeprices: Default::default(),
            background: "../assets/backgrounds/mainbg.png",
        }
    }
}
