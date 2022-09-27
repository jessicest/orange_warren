
use druid::kurbo::Circle;
use druid::widget::{Align, Flex, Label, Padding, Painter};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};
use druid::{Color, RenderContext};

struct WorldView {
}

impl WorldView where {
    fn paint(world: &World) {
    }
}

type ColorPair = (Color, Color);

fn build_ui() -> impl Widget<(Color, Color)> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::centered(make_painter()), 1.0),
                1.0,
            ),
    )
}

pub fn do_a_window() -> Result<(), PlatformError> {
    let colors = (
        Color::rgba8(11, 99, 120, 127),
        Color::rgba8(88, 22, 11, 127),
    );
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(colors)?;
    Ok(())
}

#[derive(Data)]
struct Viewport {
    #[data(ignore)]
    visible_fragments: HashMap<Vec<Fragment>>,
}

fn make_viewport_widget() -> Painter<Viewport> {
    Painter::new(|ctx, viewport: &(), env| {
        let bounds = ctx.size().to_rect().inset(-1.0);
        let size = ctx.size();

        /*
        let bounds = ctx.size().to_rect().inset(-STROKE_WIDTH / 2.0);
        let rounded = bounds.to_rounded_rect(CORNER_RADIUS);
        ctx.fill(rounded, &data.0);
        ctx.stroke(rounded, &env.get(druid::theme::PRIMARY_DARK), STROKE_WIDTH);
        let radius = f64::min(bounds.width().abs(), bounds.height().abs()) / 2.0;
        let circle = Circle::new(bounds.center(), radius);
        ctx.fill(circle, &data.1);
        ctx.stroke(circle, &env.get(druid::theme::PRIMARY_DARK), STROKE_WIDTH);
        */
    })
}
