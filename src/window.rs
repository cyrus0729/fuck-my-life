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

        ctx.add_font(egui::epaint::text::FontInsert::new(
            "neko",
            egui::FontData::from_static(include_bytes!(
                "../assets/Neko.ttf"
            )),
            vec![
                egui::epaint::text::InsertFontFamily {
                    family: egui::FontFamily::Proportional,
                    priority: egui::epaint::text::FontPriority::Highest,
                },
                egui::epaint::text::InsertFontFamily {
                    family: egui::FontFamily::Monospace,
                    priority: egui::epaint::text::FontPriority::Lowest,
                },
            ],
        ));

        egui::SidePanel::left("filler").min_width(525.0).show(ctx, |_ui| {});
        egui::SidePanel::right("menus").min_width(125.0).show(ctx, |ui| {
            ui.add_sized([120., 40.], egui::Label::new("menus"));
            ui.add_sized([120., 40.], egui::Button::new("test menu"));
        });
        let screen_bounds = egui::vec2(500.,500.);
        let wawa_size = egui::vec2(25.0,25.0);
        egui::Window::new("wawa containment chamber")
            .fixed_size(screen_bounds)
            .collapsible(false)
            .show(ctx, |ui| {

                let _window_rect = ctx.screen_rect();
                let distance = 1.0;
                // let accessible_bounds_x  = window_rect.min.x + screen_bounds.x - (screen_bounds.x - wawa_size.x/2.0)..window_rect.min.x + screen_bounds.x - wawa_size.x/2.0;
                // let accessible_bounds_y =  window_rect.min.y + screen_bounds.y - (screen_bounds.y - wawa_size.y/2.0)..window_rect.min.y + screen_bounds.y - wawa_size.x/2.0;
                let accessible_bounds_x  = self.window_pos.x + screen_bounds.x - (screen_bounds.x - wawa_size.x/2.0)..self.window_pos.x + screen_bounds.x - wawa_size.x/2.0;
                let accessible_bounds_y =  self.window_pos.y + screen_bounds.y - (screen_bounds.y - wawa_size.y/2.0)..self.window_pos.y + screen_bounds.y - wawa_size.x/2.0;
               
                let velocity_range =-2.0..2.0;

                // Update position when dragging
                if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary)) {
                    if let Some(pointer_pos) = ctx.pointer_latest_pos() {
                        // If window is being dragged, update window position
                        self.window_pos = pointer_pos; // Update the position to the pointer
                    }
                }

                if self.wawapos.len() < self.max_wawas {
                    let randx = rand::random_range(accessible_bounds_x.clone());
                    let randy = rand::random_range(accessible_bounds_y.clone());
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
                    let  new_x = *x + distance * *vx;
                    let new_y = *y + distance * *vy;

                    if new_x < accessible_bounds_x.start { *vx = -*vx;}
                    else if new_x > accessible_bounds_x.end { *vx = -*vx;}
                    if new_y < accessible_bounds_y.start { *vy = -*vy;}
                    else if new_y > accessible_bounds_y.end { *vy = -*vy;} // reflect the wawa
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
                    // eprintln!("{:?}: {:?},{:?} {:?},{:?}",i,x,y,vx,vy);
                    let rect = egui::Rect::from_pos(egui::Pos2::new(*x, *y));
                    ui.put(rect,egui::Image::new(egui::include_image!("../assets/icon.png"))
                    .fit_to_exact_size(wawa_size));

                }
        
                // Step 4: Remove clicked images - iterate in reverse to avoid issues with shifting indices
                for index in ind_to_rmv.iter().rev() {
                    self.wawapos.remove(*index);
                }

                ui.allocate_space(ui.available_size());
            });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical(|ui| {
                ui.label(format!("cash: {:1}", self.cash));
                ui.label(format!("max wawas: {:1}", self.max_wawas));
                ui.label(format!("current wawas: {:1}", self.wawapos.len()));
                ui.label(format!("wawas clicked: {:1}", self.wawas_clicked))
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

