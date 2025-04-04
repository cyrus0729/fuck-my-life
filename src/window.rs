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
                let window_rect = ctx.screen_rect();
                let distance = 1.0;
                let accessible_bounds_x  = screen_bounds.x - (screen_bounds.x - wawa_size.x/2.0)..screen_bounds.x - wawa_size.x/2.0;
                let accessible_bounds_y =  screen_bounds.y - (screen_bounds.y - wawa_size.y/2.0)..screen_bounds.y - wawa_size.x/2.0;
                let velocity_range =-2.0..2.0;

                if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary)) {
                    if let Some(pointer_pos) = ctx.pointer_latest_pos() {
                        if window_rect.contains(pointer_pos) {
                            if self.window_pos.x > window_rect.min.x && self.window_pos.y > window_rect.min.y && 
                            self.window_pos.x < window_rect.max.x && self.window_pos.y < window_rect.max.y {
                                let delta = pointer_pos - self.window_pos; // movement difference
                                self.window_pos += delta;
                            }
                        }
                    }
                }

                if self.abswawapos.len() < self.max_wawas {
                    let randx = rand::random_range(accessible_bounds_x.clone());
                    let randy = rand::random_range(accessible_bounds_y.clone());
                    let velx = rand::random_range(velocity_range.clone());
                    let vely = rand::random_range(velocity_range.clone());
                    self.abswawapos.push(((randx,randy),(velx,vely)));
                }

                // check if any click is active
                if ctx.input(|i| i.pointer.any_click()) {
                    if let Some(pos) = ctx.pointer_latest_pos() {
                        self.click_pos = Some((pos.x,pos.y)); // store the position of the click
                    }
                }

                // temporary variable to collect indices to remove
                let mut ind_to_rmv: Vec<usize> = Vec::new();
                self.relwawapos.clear();

                // adjusting positions based on the window position
                for ((x, y), (vx,vy)) in self.abswawapos.iter_mut() {
                    // Update x and y positions
                    let  new_x = *x + distance * *vx;
                    let new_y = *y + distance * *vy;

                    if new_x < accessible_bounds_x.start { *vx = -*vx;}
                    else if new_x > accessible_bounds_x.end { *vx = -*vx;}
                    if new_y < accessible_bounds_y.start { *vy = -*vy;}
                    else if new_y > accessible_bounds_y.end { *vy = -*vy;} // reflect the wawa
                    *x = new_x;
                    *y = new_y;

                    self.relwawapos.push(((*x + self.window_pos.x,*y + self.window_pos.y),(*vx,*vy)));
                } // this only handles the physics

                for (i,((x, y), (_vx,_vy))) in self.relwawapos.iter_mut().enumerate() {
                    // eprintln!("{:?}: {:?},{:?} {:?},{:?}",i,x,y,vx,vy);

                    if let Some((x1, y1)) = self.click_pos {
                        // see if the clicked point is close enough to (x, y)
                        if (x1 - *x).abs() <= wawa_size.x/2.0 && (y1 - *y).abs() <= wawa_size.y/2.0 {
                            eprint!("wawa clicked");
                            ind_to_rmv.push(i);  // Collect the index for removal
                            self.cash += Decimal::new(1.0); 
                            self.wawas_clicked += Decimal::new(1.0);
                        }
                    }

                    let rect = egui::Rect::from_pos(egui::Pos2::new(*x, *y));
                    ui.put(rect,egui::Image::new(egui::include_image!("../assets/icon.png"))
                    .fit_to_exact_size(wawa_size));
                } // this handles the clicking and rendering
        


                // remove clicked wawas
                for index in ind_to_rmv.iter().rev() {
                    self.abswawapos.remove(*index);
                }
                ui.allocate_space(ui.available_size());
                eprintln!("cycle completed");
            });


        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical(|ui| {
                ui.label(format!("cash: {:1}", self.cash));
                ui.label(format!("max wawas: {:1}", self.max_wawas));
                ui.label(format!("current wawas: {:1}", self.abswawapos.len()));
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

