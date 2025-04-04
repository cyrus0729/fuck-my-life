
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

        egui::SidePanel::left("filler").min_width(500.0).show(ctx, |_ui| {});

        let screen_bounds = egui::vec2(500.0,500.0);
        let wawa_size = egui::vec2(25.0,25.0);
        egui::Window::new("wawa containment chamber")
            .fixed_size(screen_bounds)
            .fixed_pos(egui::pos2(1920.0/2.0,1080.0/2.0))
            .collapsible(false)
            .show(ctx, |ui| {

                let winpos = ctx.screen_rect().min;
                let distance = 1.0;
                let accessible_boundsx = screen_bounds.x-(screen_bounds.x-wawa_size.x)..screen_bounds.x-wawa_size.x;
                let accessible_boundsy = screen_bounds.y-(screen_bounds.y-wawa_size.y)..screen_bounds.y-wawa_size.y;
                let velocity_range =0.0..10.0;

                if self.wawapos.len() < self.max_wawas {
                    let randx = rand::random_range(accessible_boundsx.clone()) + winpos.x;
                    let randy = rand::random_range(accessible_boundsy.clone()) + winpos.y;
                    let velx = rand::random_range(velocity_range.clone());
                    let vely = rand::random_range(velocity_range.clone());
                    self.wawapos.push(((randx,randy),(velx,vely)));
                }

                // check if any click is active
                if ctx.input(|i| i.pointer.any_click()) {
                    if let Some(pos) = ctx.pointer_latest_pos() {
                        self.click_pos = Some((pos.x,pos.y)); // store the position of the click
                    }
                }

                // temporary variable to collect indices to remove
                let mut ind_to_rmv: Vec<usize> = Vec::new();

                // adjusting positions based on the window position
                for (i, ((x, y), (vx,vy))) in self.wawapos.iter_mut().enumerate() {
                    // Update x and y positions
                    let mut new_x = (*x + winpos.x + distance * *vx).clamp(
                        winpos.x + accessible_boundsx.start,
                        winpos.x + accessible_boundsx.end,
                    );
                    let mut new_y = (*y + winpos.y + distance * *vy).clamp(
                        winpos.y + accessible_boundsy.start,
                        winpos.y + accessible_boundsy.end,
                    );

                    if new_x < accessible_boundsx.start {new_x = accessible_boundsx.start; *vx = -*vx;}
                    if new_y < accessible_boundsx.start {new_y = accessible_boundsx.start; *vy = -*vy;} 
                    if new_x > accessible_boundsx.end {new_x = accessible_boundsx.end; *vx = -*vx;}
                    if new_y > accessible_boundsy.end {new_y = accessible_boundsy.end; *vy = -*vy;} // reflect the wawa
                    *x = new_x;
                    *y = new_y;

                    if let Some((x1, y1)) = self.click_pos {
                        // see if the clicked point is close enough to (x, y)
                        if (x1 - *x).abs() <= wawa_size.x/2.0 && (y1 - *y).abs() <= wawa_size.y/2.0 {
                            eprint!("wawa clicked");
                            ind_to_rmv.push(i);  // Collect the index for removal
                            self.cash += Decimal::new(1.0); 
                            self.wawas_clicked += Decimal::new(1.0);
                        }
                    }
                }

                for ((x,y),(_vx,_vy)) in self.wawapos.clone() {
                    let rect = egui::Rect::from_pos(egui::Pos2::new(x, y));
                    ui.put(rect,egui::Image::new(egui::include_image!("../assets/icon.png"))
                    .fit_to_exact_size(wawa_size));
                }
        
                // Step 4: Remove clicked images - iterate in reverse to avoid issues with shifting indices
                for index in ind_to_rmv.iter().rev() {
                    self.wawapos.remove(*index);
                }
                ui.set_height(screen_bounds.x);
                ui.set_width(screen_bounds.y);
            });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical(|ui| {
                ui.label(format!("cash: {}", self.cash));
                ui.label(format!("max wawas: {}", self.max_wawas));
                ui.label(format!("current wawas: {}", self.wawapos.len()));
                ui.label(format!("wawas clicked: {}", self.wawas_clicked))
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

