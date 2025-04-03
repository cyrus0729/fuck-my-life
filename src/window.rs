// src/window.rs
use crate::data::Wawa;
use eframe::{egui, App};
use break_infinity::{self, Decimal};


impl Wawa {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_cash(&mut self, amount: Decimal) {
        self.cash += amount;
    }

    pub fn increment_wawas_clicked(&mut self) {
        self.wawas_clicked += Decimal::new(1.0);
    }

}

impl App for Wawa {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        let mut _index_to_remove: Option<usize> = None;

    
            // if self.wawa_count< self.max_wawas {
            //     let new_x = /*rand::random::<f32>() * */ 100.0; // Example for random x
            //     let new_y = /*rand::random::<f32>() * */ 100.0; // Example for random y
            //     self.wawapos.push((new_x, new_y));
            //     self.wawa_count += 1;
            // }

            // for (i, (x, y)) in self.wawapos.iter().enumerate() {
            //     // Create a button at the specific position
            //     if ui.put(
            //     egui::Rect::from_pos(egui::Pos2::new(*x, *y)),
            //     egui::ImageButton::new("../assets/icon.png")).clicked()
            //     { index_to_remove = Some(i);}
            // }
            // // Remove the clicked button
            // if let Some(i) = index_to_remove {
            //     self.add_cash(Decimal::new(1.0));
            //     self.increment_wawas_clicked();
            //     self.wawapos.remove(i);
            // }

        egui::SidePanel::left("filler").min_width(300.0).show(ctx, |_ui| {

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(format!("cash: {}", self.cash));
                ui.label(format!("max wawas: {}", self.max_wawas));
                ui.label(format!("current wawas: {}", self.wawa_count));
                ui.label(format!("wawas clicked: {}", self.wawas_clicked))
            });

            egui::Window::new("wawa containment chamber").fixed_size(egui::vec2(500.0,500.0)).show(ctx, |ui| {
                let wawa = egui::Image::new(egui::include_image!("../assets/icon.png"));//.max_size(egui::Vec2::new(25.0,25.0));
                while self.wawa_count < self.max_wawas {
                    let randx = rand::random_range(25.0..475.0);
                    let randy = rand::random_range(25.0..475.0);
                    // ui.put(egui::Rect::from_pos(egui::Pos2::new(randx, randy)),wawa.clone()); 
                    //ui.put(egui::Rect::from_pos(egui::Pos2::new(100.0,100.0)),wawa2.clone()); 
                    let wawa2 = egui::ImageButton::new(wawa.clone());
                    ui.add(wawa2); // why isnt this working?
                    self.wawa_count += 1
                }
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());
            });

            // let wawa = egui::ImageButton::new(egui::include_image!("../assets/icon.png"));
            // ui.add(wawa.clone())

            // ui.label("Welcome to Wawa App!");
            // ui.label(format!("Current cash: {}", self.get_cash()));
            // ui.label(format!("Wawas clicked: {}", self.get_wawas_clicked()));

            // if ui.button("Add Cash").clicked() {
            //     self.add_cash(Decimal::new(10.0));
            // }
            // if ui.button("Increment Wawas Clicked").clicked() {
            //     self.increment_wawas_clicked();
            // }
            //ui.add(egui::Separator::default().vertical().spacing(400.0));
        });
    }
}

