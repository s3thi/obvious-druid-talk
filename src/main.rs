use druid::widget::{Align, Label};
use druid::{AppLauncher, Widget, WindowDesc};

fn main() {
    let window = WindowDesc::new(ui_builder);
    let data: u32 = 0;
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<u32> {
    Align::centered(Label::new("Hello, world!"))
}
