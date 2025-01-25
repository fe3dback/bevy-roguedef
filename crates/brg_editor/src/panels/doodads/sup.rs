use bevy::ecs::system::SystemParam;
use bevy::prelude::{Query, Res, ResMut, With};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_inspector_egui::egui::Window;
use brg_fundamental::prelude::ResCoords;
use strum::IntoEnumIterator;

use super::enum_bank::EBank;
use super::res_state::ResPanelState;

#[derive(SystemParam)]
pub struct SupPanel<'w, 's> {
    ctx:    Query<'w, 's, &'static mut EguiContext, With<PrimaryWindow>>,
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

        let Ok(mut bevy_ctx) = self.ctx.get_single_mut() else {
            return;
        };
        let ctx = bevy_ctx.get_mut();

        win.show(ctx, |ui| {
            egui::ComboBox::from_label("Bank")
                .selected_text(self.state.bank.to_string())
                .show_ui(ui, |ui| {
                    for bank in EBank::iter() {
                        ui.selectable_value(&mut self.state.bank, bank, bank.to_string());
                    }
                });
            ui.end_row();
        });
    }
}
