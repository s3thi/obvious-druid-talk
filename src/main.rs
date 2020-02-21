use druid::widget::TextBox;
use druid::{AppLauncher, Data, Lens, LensWrap, Widget, WindowDesc};
use std::sync::Arc;

struct TodoItem {
    task: String,
    is_completed: bool,
}

#[derive(Data, Clone, Lens)]
struct AppState {
    todo_list: Arc<Vec<TodoItem>>,
    todo_entry: String,
}

fn main() {
    let window = WindowDesc::new(ui_builder);
    let data = AppState {
        todo_list: Arc::new(vec![]),
        todo_entry: "".to_string(),
    };
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<AppState> {
    LensWrap::new(TextBox::new(), AppState::todo_entry)
}
