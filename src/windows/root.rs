use crate::egui_multiwin_dynamic::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};
use egui_multiwin::egui::{Align2, Color32, DragValue, FontFamily, FontId, menu, Rect, Rounding, Stroke, Ui};
use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::winit::dpi::{PhysicalSize};
use scoreboard_lib::{Game, get_match};
use crate::ui::widget::{Padding, TextWidget, Widget};

use crate::AppCommon;
use crate::ui::score_grid::ScoreGrid;
use crate::windows::HandleInput;

use super::score_window::ScoreWindow;

pub struct RootWindow {
    is_fullscreen: bool,
    first_run: bool,
    scale: f32,
    widgets: Vec<Widget>,
    window_size: PhysicalSize<u32>,
}

impl RootWindow {
    pub fn request() -> NewWindowRequest {
        let window_size = PhysicalSize{ width: 800, height: 600};

        NewWindowRequest {

            window_state: super::MyWindows::Root(RootWindow { is_fullscreen: false, first_run: true, scale: 2.0, widgets: vec![], window_size }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(window_size)
                .with_title("BJJ Scoreboard"),
            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
            id: egui_multiwin::multi_window::new_id(),
        }
    }
    pub fn grid_config_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::Window::new("Grid Config")
            .open(&mut c.show_grid_config)
            .vscroll(true)
            .min_width(700.0)
            .show(&egui.egui_ctx, |ui| {
                egui_multiwin::egui::Grid::new("Grid Settings")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Bottom");
                        ui.add(
                            DragValue::new(&mut c.grid_config.bottom)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Name");
                        ui.add(
                            DragValue::new(&mut c.grid_config.name)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Stalling");
                        ui.add(
                            DragValue::new(&mut c.grid_config.stalling)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Adv");
                        ui.add(
                            DragValue::new(&mut c.grid_config.adv)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Time");
                        ui.add(
                            DragValue::new(&mut c.grid_config.time)
                                .speed(0.01)
                                .clamp_range(0.05..=0.50),
                        );
                        ui.end_row();

                        ui.label("Bracket");
                        ui.add(
                            DragValue::new(&mut c.grid_config.bracket)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Match Type");
                        ui.add(
                            DragValue::new(&mut c.grid_config.match_type)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Points");
                        ui.add(
                            DragValue::new(&mut c.grid_config.points)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Flag");
                        ui.add(
                            DragValue::new(&mut c.grid_config.flag)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();
                    });
            });
    }
    fn on_resize(&self, c:&mut AppCommon, rect: Rect) {
        c.first_run = false;
        c.scoring_grid = ScoreGrid::calc(rect, &c.grid_config);
    }

    fn draw_grid(&self, c:&mut AppCommon, ui: &mut Ui) {
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.name_grid.name, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.name_grid.flag, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.name_grid.team, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.name_grid.country, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.stalling_grid.stalling, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.points_grid.advantage, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.points_grid.penalty, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_one.points_grid.points, Rounding::default(), Stroke::new(2.0, Color32::GREEN));

        ui.painter().rect_stroke(c.scoring_grid.competitor_two.name_grid.name, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_two.name_grid.flag, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_two.name_grid.team, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_two.name_grid.country, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_two.stalling_grid.stalling, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_two.points_grid.advantage, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_two.points_grid.penalty, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.competitor_two.points_grid.points, Rounding::default(), Stroke::new(2.0, Color32::GREEN));

        ui.painter().rect_stroke(c.scoring_grid.match_info.time, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.match_info.match_type, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.match_info.match_info, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.scoring_grid.match_info.bracket_info, Rounding::default(), Stroke::new(2.0, Color32::GREEN));

    }

    fn draw_menu(&self, windows_to_create: &mut Vec<NewWindowRequest>, c:&mut AppCommon, egui: &mut EguiGlow) -> bool {
        let mut quit = false;

        egui_multiwin::egui::TopBottomPanel::top("menu_bar").show(&egui.egui_ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Sample Match").clicked() {
                        c.bjj_match = Some(get_match());
                        println!("{:?}", c.bjj_match);
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        quit = true;
                    }
                });
                ui.menu_button("Window", |ui| {
                    if ui.selectable_label(c.show_score_window, "Score window").clicked() {
                        c.show_score_window = !c.show_score_window;

                        if c.show_score_window {
                            windows_to_create.push(ScoreWindow::request("Score".to_string()));
                        }
                        ui.close_menu();
                    }

                    if ui.selectable_label(c.show_grid_config, "Grid Config").clicked() {
                        c.show_grid_config = !c.show_grid_config;
                        ui.close_menu();
                    }
                });
            });
        });

        quit
    }

    fn create_widgets(&mut self, c: &AppCommon, game: &Game) {
        self.widgets.clear();

        self.widgets.push(
            Widget::Text(
                TextWidget {
                    text: game.competitor_one.display_name(),
                    alignment: Align2::LEFT_CENTER,
                    rect: c.scoring_grid.competitor_one.name_grid.name,
                    font: FontId::new(c.font_config.competitor,FontFamily::Name("score_font".into())),
                    padding: Padding::left(4.0),
                    color: c.color_scheme.competitor_one_name
                }
            )
        );

        self.widgets.push(
            Widget::Text(
                TextWidget {
                    text: game.competitor_one.team.name.to_string(),
                    alignment: Align2::LEFT_CENTER,
                    rect: c.scoring_grid.competitor_one.name_grid.team,
                    font: FontId::new(c.font_config.team, FontFamily::Name("score_font".into())),
                    padding: Padding::left(4.0),
                    color: c.color_scheme.competitor_one_team
                }
            )
        );

        self.widgets.push(
            Widget::Text(
                TextWidget {
                    text: game.competitor_one.country.to_string(),
                    alignment: Align2::CENTER_CENTER,
                    rect: c.scoring_grid.competitor_one.name_grid.country,
                    font: FontId::new(c.font_config.country, FontFamily::Name("score_font".into())),
                    padding: Padding::none(),
                    color: c.color_scheme.competitor_one_country
                }
            )
        );
    }

    fn draw_widgets(&mut self, ui: &mut Ui, scale: f32) {
        for widget in &mut self.widgets {
            widget.draw(ui, scale);
        }
    }

    fn draw_match(&mut self, c:&mut AppCommon, window: &egui_multiwin::winit::window::Window, egui: &mut EguiGlow) {
        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            self.draw_grid(c, ui);

            if self.first_run {
                self.on_resize(c, ui.available_rect_before_wrap());
            }

            if self.window_size != window.inner_size() {
                self.scale = window.inner_size().width as f32 / 400.0 * c.font_config.scale;
                self.window_size = window.inner_size();
                self.first_run = true;
            }

            if let Some(bjj_match) = &c.bjj_match {
                self.create_widgets(c, bjj_match);

                self.draw_widgets(ui, self.scale);
            }


        });
    }
}

impl HandleInput for RootWindow {
    fn set_fullscreen(&mut self, fullscreen: bool) {
        self.is_fullscreen = fullscreen;
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}

impl TrackedWindow for RootWindow {
    fn is_root(&self) -> bool {
        true
    }

    fn set_root(&mut self, _root: bool) {}

    fn redraw(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
        window: &egui_multiwin::winit::window::Window,
        _clipboard: &mut egui_multiwin::arboard::Clipboard,
    ) -> RedrawResponse {
        let mut quit = false;

        egui.egui_ctx.request_repaint();

        let mut windows_to_create = vec![];

        self.handle_input(c, window, egui);
        self.grid_config_ui(c, egui);
        quit = self.draw_menu(&mut windows_to_create, c, egui);
        self.draw_match(c, window, egui);

        RedrawResponse {
            quit,
            new_windows: windows_to_create,
        }
    }
}