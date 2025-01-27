use bevy::ecs::system::SystemParam;
use bevy::prelude::{Query, Res, ResMut, With};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_inspector_egui::egui::Window;
use brg_core::prelude::{ICDoodadsCategory, IdOf};
use brg_fundamental::prelude::ResCoords;
use brg_scene::prelude::{AssetMGADoodadCategory, SupAssets};

use super::res_state::ResPanelState;

#[derive(SystemParam)]
pub struct SupPanel<'w, 's> {
    ctx:    Query<'w, 's, &'static mut EguiContext, With<PrimaryWindow>>,
    assets: SupAssets<'w>,
    coords: Res<'w, ResCoords>,
    state:  ResMut<'w, ResPanelState>,
}

impl<'w, 's> SupPanel<'w, 's> {
    fn window(&mut self) -> Option<Window<'s>> {
        let (w, h) = (320.0, 480.0);
        let padding = 20.0;

        Some(
            Window::new("Doodads")
                .vscroll(true)
                .default_width(w)
                .default_height(h)
                .resizable(false)
                .default_pos(egui::pos2(
                    self.coords.screen_ui_width as f32 - w - (padding + 10.0),
                    padding,
                )),
        )
    }

    pub fn draw(&mut self) {
        let Some(win) = self.window() else {
            return;
        };

        // todo: need some research about better mapping (data -> view)
        // - can`t read category titles and mutate selected category
        // - this bad code: we copy strings every frame
        let categories: Vec<(IdOf<ICDoodadsCategory>, String)> = self
            .state
            .available_assets
            .keys()
            .map(|c| (*c, self.get_category_title(*c)))
            .collect();

        let category_title = match self.state.selected_category {
            Some(c) => self.get_category_title(c),
            None => "?".to_string(),
        };

        let Ok(mut bevy_ctx) = self.ctx.get_single_mut() else {
            return;
        };
        let ctx = bevy_ctx.get_mut();

        win.show(ctx, |ui| {
            egui::ComboBox::from_label("Categories")
                .selected_text(category_title)
                .show_ui(ui, |ui| {
                    for (cat, title) in categories {
                        ui.selectable_value(&mut self.state.selected_category, Some(cat), title);
                    }
                });
            ui.end_row();
        });
    }

    fn get_category_title(&self, id: IdOf<ICDoodadsCategory>) -> String {
        match self.assets.get::<AssetMGADoodadCategory, _>(id) {
            Some(category) => category.title.clone(),
            None => "?".to_string(),
        }
    }
}
