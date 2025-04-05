use break_infinity::Decimal;
use eframe::App;
use egui::*;
use std::time::*;
use crate::global_funcs;

impl crate::Wawa {
    pub fn new() -> Self {
        Self::default()
    }
}

impl App for crate::Wawa {

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        ctx.request_repaint();
        ctx.add_font(epaint::text::FontInsert::new(
            "font",
            FontData::from_static(include_bytes!(
                "../assets/8514oem.ttf"
            )),
            vec![
                epaint::text::InsertFontFamily {
                    family: FontFamily::Proportional,
                    priority: epaint::text::FontPriority::Highest,
                },
                epaint::text::InsertFontFamily {
                    family: FontFamily::Monospace,
                    priority: epaint::text::FontPriority::Lowest,
                },
            ],
        ));

        Window::new("wawa containment chamber")
            .fixed_size(crate::CONTAINMENT_BOUNDS)
            .fixed_pos(pos2(0.,0.))
            .collapsible(false)
            .movable(false)
            .show(ctx, |ui| {
                // temporary variable to collect indices to remove
                let mut ind_to_rmv: Vec<usize> = Vec::new();
                for (i,((x, y), (_vx,_vy))) in self.wawapos.iter_mut().enumerate() {
                    // Check if any click is active
                    if ctx.input(|i| i.pointer.any_click()) {
                        if let Some(pos) = ctx.pointer_latest_pos() {
                            // Store the position of the click and the current timestamp
                            self.click_pos = Some((pos.x, pos.y));
                            self.click_timestamp = Some(std::time::Instant::now()); // Record the time of the click
                        }
                    }
                    if let Some(timestamp) = self.click_timestamp {
                        if Instant::now().duration_since(timestamp) >= Duration::from_secs_f32(0.1) {
                            // Clear the click position if 0.1 seconds have passed
                            self.click_pos = None;
                            self.click_timestamp = None; // Reset timestamp
                        }
                    } else {
                        // Set default pos if no click is active
                        // click_pos = Some((f32::MAX, f32::MAX));
                    }

                    
                    if let Some((x1, y1)) = self.click_pos {
                        // see if the clicked point is close enough to (x, y)
                        if (x1 - *x).abs() <= crate::WAWA_SIZE.x/1.5 && (y1 - *y).abs() <= crate::WAWA_SIZE.y/1.5 {
                            ind_to_rmv.push(i);  // Collect the index for removal
                            self.cash += self.upgradecounts[1]; 
                            self.wawas_clicked += Decimal::new(1.);
                        }
                    }

                    let rect = Rect::from_pos(Pos2::new(*x, *y));
                    ui.put(rect,Image::new(include_image!("../assets/icon.png"))
                    .fit_to_exact_size(crate::WAWA_SIZE));
                } // this handles the clicking and rendering
        
                // remove clicked wawas
                for index in ind_to_rmv.iter().rev() {
                    self.wawapos.remove(*index);
                    global_funcs::play_sound("../assets/owSfx.ogg");
                }
                ui.allocate_space(ui.available_size());
        });
        

        SidePanel::left("filler").exact_width(525.0).show(ctx, |ui| {
            ui.add_space(525. + 25.);
            ui.style_mut().text_styles.insert(
                TextStyle::Body,
                FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.vertical(|ui| {
                ui.label(format!("cash: {:.2}", self.cash));
                ui.label(format!("max wawas: {:.2}", self.max_wawas));
                ui.label(format!("current wawas: {:.2}", self.wawapos.len()));
                ui.label(format!("wawas clicked: {:.2}", self.wawas_clicked));
            });
        });
        SidePanel::right("menus").exact_width(225.0).show(ctx, |ui| {
            Image::new(self.background).paint_at(ui, ui.ctx().screen_rect());
            ui.style_mut().text_styles.insert(
                TextStyle::Body,
                FontId::new(36.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.style_mut().text_styles.insert(
                TextStyle::Button,
                FontId::new(36.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.label("menus");
            let _menu0 = ui.button(crate::MENUS[0]);
            let _menu1 = ui.button(crate::MENUS[1]);
            let _menu2 = ui.button(crate::MENUS[2]);
        });

        CentralPanel::default().show(ctx, |ui| {
            Grid::new("upgrade_grid").show(ui, |ui| {
                ui.style_mut().text_styles.insert(
                    TextStyle::Body,
                    FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
                );
                ui.style_mut().text_styles.insert(
                    TextStyle::Button,
                    FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
                );

                ui.label(format!("{},[+{}]",self.upgradenames[0],Decimal::new(0.05) * self.upgradecounts[0] ));
                let _ = ui.button(format!("Increase: {:?}",self.upgradeprices[0].to_exponential(1)));
                ui.end_row();
                ui.label(self.upgradenames[1]);
                let _ = ui.button(format!("Increase: {:?}",self.upgradeprices[1].to_exponential(1)));
                ui.end_row();
            
                ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
                ui.label("Third row, second column");
                ui.end_row();
            }); 

            // let wawa = ImageButton::new(include_image!("../assets/icon.png"));
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
            //ui.add(Separator::default().vertical().spacing(400.0));
        });
        crate::wawa_containment::update(self);
    }
}

