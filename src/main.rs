use druid::widget::{Flex, Label, List, Scroll, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use font_kit::source::SystemSource;
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppState {
    input: String,
    fonts: Arc<Vec<String>>,
}

fn build_ui() -> impl Widget<AppState> {
    let input = TextBox::new()
        .with_placeholder("Enter text to preview")
        .lens(AppState::input)
        .expand_width();

    let font_list = List::new(|| {
        Label::dynamic(|data: &String, _env| {
            format!("{}: Sample Text", data)
        })
        .with_text_size(16.0)
        .padding(10.0)
        .expand_width()
    })
    .lens(AppState::fonts);

    let scroll_font_list = Scroll::new(font_list).vertical();

    Flex::column()
        .with_child(input)
        .with_spacer(20.0)
        .with_child(scroll_font_list)
        .padding(20.0)
}

fn main() {
    let system_source = SystemSource::new();
    let font_names: Vec<String> = system_source
        .all_families()
        .unwrap()
        .into_iter()
        .map(|name| name.to_string())
        .collect();

    let initial_state = AppState {
        input: "Hello, World!".to_string(),
        fonts: Arc::new(font_names),
    };

    let main_window = WindowDesc::new(build_ui())
        .title("Font Preview App")
        .window_size((400.0, 600.0));

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}