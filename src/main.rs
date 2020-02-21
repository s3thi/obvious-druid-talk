use druid::widget::{Button, Flex, Label, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LensWrap, Widget, WindowDesc};
use std::sync::Arc;

const PADDING_BASE: f64 = 8.0;

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
    Flex::row()
        .with_child(Label::new("Add item:").padding(PADDING_BASE), 0.0)
        .with_child(
            LensWrap::new(TextBox::new().padding(PADDING_BASE), AppState::todo_entry),
            1.0,
        )
        .with_child(
            Button::new("Add", Button::noop)
                .padding(PADDING_BASE)
                .fix_width(PADDING_BASE * 8.0),
            0.0,
        )
}
