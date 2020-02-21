use druid::widget::{Button, Flex, Label, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LensWrap, Widget, WindowDesc};
use std::sync::Arc;

const PADDING_BASE: f64 = 8.0;

#[derive(Clone, Debug)]
struct TodoItem {
    task: String,
    is_completed: bool,
}

#[derive(Data, Clone, Lens, Debug)]
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
            Button::new("Add", |_, data: &mut AppState, _| {
                // Create a new TodoItem.
                let todo_item = TodoItem {
                    task: data.todo_entry.to_string(),
                    is_completed: false,
                };

                // Push it into the todo list.
                Arc::make_mut(&mut data.todo_list).push(todo_item);
                // Clear the textbox.
                data.todo_entry = "".to_string();
                // Print out the current state of the app.
                println!("{:?}", data);
            })
            .padding(PADDING_BASE)
            .fix_width(PADDING_BASE * 8.0),
            0.0,
        )
}
