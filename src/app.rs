use std::sync::Arc;
use eframe::{egui, Frame};
use std::time::{Duration,Instant};
use egui::Context;

#[derive(PartialEq, Clone, Copy)]
enum RunningMode {
    Up,
    Down,
    Loop,
}

#[derive(PartialEq, Clone, Copy)]
enum TimerState {
    Focus,
    Break,
}
pub struct PomodoroTimer {
    is_running: bool,
    mode: RunningMode,
    focus_duration: Duration,
    break_duration : Duration,
    current_time: Duration,
    last_tick: Option<Instant>,
    state : TimerState,
}

impl Default for PomodoroTimer {
    fn default() -> Self {
        PomodoroTimer {
            is_running : false,
            mode : RunningMode::Down,
            focus_duration : Duration::from_secs(25 * 60),
            break_duration : Duration::from_secs(5 * 60),
            current_time : Duration::from_secs(25 * 60),
            last_tick : None,
            state : TimerState::Focus,
        }
    }
}

impl PomodoroTimer {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self{
        Self::set_font_custom(&cc.egui_ctx);
        Self::default()
    }

    fn set_font_custom(ctx : &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "my_chinese_font".to_owned(),
            Arc::from(egui::FontData::from_static(include_bytes!("../assets/SourceHanSansCN-Medium.otf"))),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0,"my_chinese_font".to_owned());

        ctx.set_fonts(fonts)
    }

    fn tick(&mut self) {
        if !self.is_running {
            self.last_tick = None;
            return;
        }

        let now = Instant::now();

        if let Some(last) = self.last_tick {
           let  elapsed = now.duration_since(last);

            match self.mode {
                RunningMode::Down => {
                    self.current_time = self.current_time.saturating_sub(elapsed);

                    if self.current_time.is_zero() {
                        self.is_running = false;
                        self.last_tick = None;
                        return;
                    }
                }

                RunningMode::Up => {
                    self.current_time = self.current_time.saturating_add(elapsed);
                }

                RunningMode::Loop => {
                    self.current_time = self.current_time.saturating_sub(elapsed);
                    if self.current_time.is_zero() {
                        match self.state {
                            TimerState::Focus => {
                                self.state = TimerState::Break;
                                self.current_time = self.break_duration;
                            }
                            TimerState::Break => {
                                self.state = TimerState::Focus;
                                self.current_time = self.focus_duration;
                            }
                        }
                    }
                }
            }
        }

        self.last_tick = Some(now);
    }
}

impl eframe::App for PomodoroTimer {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.tick();

        let total_ms = self.current_time.as_millis();

        let s = total_ms / 1000 % 60;
        let min = total_ms / 1000 / 60;
        let ms_part = total_ms % 1000;

        egui::CentralPanel::default().show(ctx , |ui| {

            ui.vertical_centered_justified(|ui| {
                ui.add_space(30.0);

                let time_str = format!("{}分{:02}秒{:03}毫秒", min, s, ms_part);

                let time_color = if self.is_running {
                    egui::Color32::from_rgb(50,200,255)
                } else {
                    egui::Color32::GRAY
                };

                ui.label(
                   egui::RichText::new(time_str)
                       .size(60.0)
                       .strong()
                       .color(time_color)
                );

                ui.add_space(30.0);

                let btn_text = if self.is_running { "暂停"} else { "开始" };

                if ui.add(egui::Button::new(egui::RichText::new(btn_text).size(24.0)).fill(
                    if self.is_running {
                        egui::Color32::from_rgb(60, 120, 200)
                    } else {
                        egui::Color32::from_white_alpha(40)
                    }
                )).clicked() {
                    self.is_running = !self.is_running;
                    if self.is_running {
                        self.last_tick = Some(Instant::now())
                    }
                }

                ui.add_space(20.0);

                ui.separator();

                ui.add_space(10.0);

                ui.columns(3, |columns| {

                    let is_down = self.mode == RunningMode::Down;
                    let btn_down = egui::Button::new(
                        egui::RichText::new("倒计时")
                            .size(16.0)
                            .color(if is_down { egui::Color32::WHITE } else { egui::Color32::GRAY })
                    )
                        .fill(if is_down { egui::Color32::from_rgb(60, 120, 200) } else { egui::Color32::TRANSPARENT })
                        .corner_radius(8.0);

                    if columns[0].add(btn_down).clicked() {
                        self.is_running = false;
                        self.current_time = self.focus_duration;
                        self.mode = RunningMode::Down;
                    }

                    let is_up = self.mode == RunningMode::Up;
                    let btn_up = egui::Button::new(
                        egui::RichText::new("正计时")
                            .size(16.0)
                            .color(if is_up { egui::Color32::WHITE } else { egui::Color32::GRAY })
                    )
                        .fill(if is_up { egui::Color32::from_rgb(60, 120, 200) } else { egui::Color32::TRANSPARENT })
                        .corner_radius(8.0);

                    if columns[1].add(btn_up).clicked() {
                        self.is_running = false;
                        self.current_time = Duration::ZERO;
                        self.mode = RunningMode::Up;
                    }

                    let is_loop = self.mode == RunningMode::Loop;
                    let btn_loop = egui::Button::new(
                        egui::RichText::new("番茄钟")
                            .size(16.0)
                            .color(if is_loop { egui::Color32::WHITE } else { egui::Color32::GRAY })
                    )
                        .fill(if is_loop { egui::Color32::from_rgb(60, 120, 200) } else { egui::Color32::TRANSPARENT })
                        .corner_radius(8.0);

                    if columns[2].add(btn_loop).clicked() {
                        self.is_running = false;
                        self.current_time = self.focus_duration;
                        self.mode = RunningMode::Loop;
                    }
                });

                ui.add_space(20.0);

                let mut temp_focus_mins = self.focus_duration.as_secs_f64() /60.0;
                let mut temp_break_mins = self.break_duration.as_secs_f64() / 60.0;

                ui.columns(2 , |columns| {
                    if columns[0].add(egui::Slider::new(&mut temp_focus_mins,0.0..=60.0)
                        .text("专注/分")
                        .step_by(0.5))
                        .changed()
                    {
                        self.focus_duration = Duration::from_secs_f64(temp_focus_mins * 60.0)     ;
                        if !self.is_running && self.mode != RunningMode::Up {
                           self.current_time = self.focus_duration;
                        }
                    }


                    if columns[1].add(egui::Slider::new(&mut temp_break_mins,0.0..=60.0)
                        .text("休息/分")
                        .step_by(0.5))
                        .changed()
                    {
                        self.break_duration = Duration::from_secs_f64(temp_break_mins * 60.0)     ;
                    }
                })

            })
        });


        if self.is_running {
            ctx.request_repaint();
        }
    }
}

