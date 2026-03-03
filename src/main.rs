use Pomodoro_Timer::PomodoroTimer;

fn main() -> eframe::Result<()>  {
    let options =  eframe::NativeOptions {
        viewport : eframe::egui::ViewportBuilder::default().with_inner_size([600.0,400.0]),
        multisampling : 4,
        ..Default::default()
    };

    eframe::run_native(
        "我的番茄钟",
        options,
        Box::new(|cc| Ok(Box::new(PomodoroTimer::new(cc)))),
    )
}
