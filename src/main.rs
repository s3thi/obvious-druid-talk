use druid::widget::{Button, Checkbox, Flex, Label, List, Padding, Scroll, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Env, Insets, Lens, LensWrap, Widget, WindowDesc};
use std::sync::Arc;

const PADDING_BASE: f64 = 8.0;

#[derive(Data, Clone, Debug, Lens)]
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
    let header = Flex::row()
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
        );

    let todo_list = Scroll::new(List::new(|| {
        Flex::row()
            .with_child(Checkbox::new().lens(TodoItem::is_completed), 0.0)
            .with_child(
                Padding::new(
                    Insets::new(PADDING_BASE, 0.0, 0.0, 0.0),
                    Label::new(|item: &TodoItem, _: &Env| item.task.to_string()),
                ),
                1.0,
            )
            .padding(PADDING_BASE)
    }))
    .vertical()
    .lens(AppState::todo_list);

    let footer = Flex::row()
        .with_child(
            Label::new(|data: &AppState, _: &Env| {
                format!(
                    "{} total, {} completed",
                    data.todo_list.len(),
                    data.todo_list.iter().filter(|i| i.is_completed).count()
                )
            })
            .padding(PADDING_BASE),
            1.0,
        )
        .with_child(
            Button::new("Clear Completed", |_, data: &mut AppState, _| {
                Arc::make_mut(&mut data.todo_list).retain(|item| !item.is_completed);
            })
            .padding(PADDING_BASE)
            .fix_width(PADDING_BASE * 24.0),
            0.0,
        );

    Flex::column()
        .with_child(header.fix_height(PADDING_BASE * 6.0), 0.0)
        .with_child(todo_list, 1.0)
        .with_child(footer.fix_height(PADDING_BASE * 6.0), 0.0)
}
