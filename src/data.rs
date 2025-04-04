
// src/data.rs
use break_infinity::Decimal;

pub struct Wawa {
    pub cash: Decimal,
    pub wawas_clicked: Decimal,
    pub max_wawas: usize,
    pub wawapos: Vec<((f32, f32),(f32,f32))>, //pos x,y and vel x,y
    pub click_pos: Option<(f32, f32)>,
}

impl Default for Wawa {
    fn default() -> Self {
        Self { 
            cash: Decimal::new(0.0), 
            wawas_clicked: Decimal::new(0.0),
            max_wawas: 10, 
            wawapos: Default::default(),
            click_pos: Default::default(),
        }
    }
}
