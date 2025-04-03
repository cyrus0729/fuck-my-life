// src/data.rs
use break_infinity::Decimal;

pub struct Wawa {
    pub cash: Decimal,
    pub wawas_clicked: Decimal,
    pub max_wawas: usize,
    pub wawa_count: usize,
    pub wawapos: Vec<(f32, f32)>,
}

impl Default for Wawa {
    fn default() -> Self {
        Self { 
            cash: Decimal::new(0.0), 
            wawas_clicked: Decimal::new(0.0),
            max_wawas: 1, 
            wawa_count: 0,
            wawapos: Default::default()}
    }
}
