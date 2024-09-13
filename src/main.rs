use druid::widget::{Flex, Label, List, TextBox, SizedBox};
use druid::{
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
    piet::{FontFamily, Text, TextLayout, TextLayoutBuilder},
    widget::prelude::*,
};
use font_kit::source::SystemSource;
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppState {
    input: String,
    fonts: Arc<Vec<String>>,
}

#[derive(Clone, Data, Lens)]
struct FontPreview {
    font: Arc<String>,
    text: String,
}

struct FontListLens;

impl Lens<AppState, Arc<Vec<FontPreview>>> for FontListLens {
    fn with<V, F: FnOnce(&Arc<Vec<FontPreview>>) -> V>(&self, data: &AppState, f: F) -> V {
        let font_previews = Arc::new(
            data.fonts
                .iter()
                .map(|font| FontPreview {
                    font: Arc::new(font.clone()),
                    text: data.input.clone(),
                })
                .collect(),
        );
        f(&font_previews)
    }

    fn with_mut<V, F: FnOnce(&mut Arc<Vec<FontPreview>>) -> V>(&self, data: &mut AppState, f: F) -> V {
        let mut font_previews = Arc::new(
            data.fonts
                .iter()
                .map(|font| FontPreview {
                    font: Arc::new(font.clone()),
                    text: data.input.clone(),
                })
                .collect(),
        );
        let result = f(&mut font_previews);
        result
    }
}

struct FontLabel;

impl Widget<FontPreview> for FontLabel {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut FontPreview, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &FontPreview, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &FontPreview, _data: &FontPreview, _env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &FontPreview, env: &Env) -> Size {
        let font = ctx.text().font_family(&data.font).unwrap_or_else(|| FontFamily::SYSTEM_UI);
        let layout = ctx.text().new_text_layout(data.text.clone())
            .font(font, 32.0)
            .text_color(env.get(druid::theme::TEXT_COLOR))
            .build()
            .unwrap();
        layout.size()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &FontPreview, env: &Env) {
        let font = ctx.text().font_family(&data.font).unwrap_or_else(|| FontFamily::SYSTEM_UI);
        let layout = ctx.text().new_text_layout(data.text.clone())
            .font(font, 32.0)
            .text_color(env.get(druid::theme::TEXT_COLOR))
            .build()
            .unwrap();
        ctx.draw_text(&layout, (0.0, 0.0));
    }
}

fn build_ui() -> impl Widget<AppState> {
    let input = TextBox::new()
        .with_placeholder("Enter text to preview")
        .lens(AppState::input)
        .expand_width();

    let font_list = List::new(|| {
        Flex::row()
            .with_flex_child(
                SizedBox::new(FontLabel)
                    .expand_width()
                    .padding((0., 0., 20., 0.)), // Add right padding to create more space
                1.0
            )
            .with_child(
                Label::new(|item: &FontPreview, _env: &_| item.font.to_string())
                    .with_text_size(14.0)
                    .fix_width(200.0) // Increased width for font names
            )
            .must_fill_main_axis(true)
            .padding(10.0)
    })
    .lens(FontListLens)
    .scroll()
    .vertical()
    .expand_width();

    Flex::column()
        .with_child(input)
        .with_spacer(20.0)
        .with_flex_child(font_list, 1.0)
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
        .window_size((1200.0, 900.0)); // Increased window width

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}