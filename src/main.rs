use druid::widget::{Flex, Label, TextBox};
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

    let font_list = Label::dynamic(|data: &AppState, _env| {
        data.fonts
            .iter()
            .map(|font| format!("{}: {}", font, data.input))
            .collect::<Vec<_>>()
            .join("\n")
    })
    .with_text_size(16.0)
    .padding(10.0)
    .expand_width();

    Flex::column()
        .with_child(input)
        .with_spacer(20.0)
        .with_child(font_list)
        .scroll()
        .vertical()
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
        input: "Sample Text".to_string(),
        fonts: Arc::new(font_names),
    };

    let main_window = WindowDesc::new(build_ui())
        .title("Font Preview App")
        .window_size((400.0, 600.0));

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}