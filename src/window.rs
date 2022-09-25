
use druid::kurbo::{BezPath, Point};
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::{Color, Env, PaintCtx,Rect, RenderContext};
use druid::widget::{Label, Painter, Flex, Padding, Align};

fn build_ui() -> impl Widget<Color> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::centered(make_painter()), 1.0),
                1.0))
}

pub fn do_a_window() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(Color::rgba8(11, 99, 120, 127))?;
    Ok(())
}

/*
pub fn draw_piet() {
    // Create an arbitrary bezier path
    // (ctx.size() returns the size of the layout rect we're painting in)
    let mut path = BezPath::new();
    path.move_to(Point::ORIGIN);
    path.quad_to(
        (80.0, 90.0),
        (ctx.size().width, ctx.size().height),
    );
    // Create a color
    let stroke_color = Color::rgb8(0x00, 0x80, 0x00);
    // Stroke the path with thickness 1.0
    ctx.stroke(path, &stroke_color, 1.0);

    // Rectangles: the path for practical people
    let rect = Rect::from_origin_size((10., 10.), (100., 100.));
    // Note the Color:rgba8 which includes an alpha channel (7F in this case)
    let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
    ctx.fill(rect, &fill_color);
}
*/

fn make_painter() -> Painter<Color> {
    const CORNER_RADIUS: f64 = 4.0;
    const STROKE_WIDTH: f64 = 2.0;

    Painter::new(|ctx, data: &Color, env| {
        // Shrink the bounds a little, to ensure that our stroke remains within
        // the paint bounds.
        let bounds = ctx.size().to_rect().inset(-STROKE_WIDTH / 2.0);
        let rounded = bounds.to_rounded_rect(CORNER_RADIUS);
        ctx.fill(rounded, data);
        ctx.stroke(rounded, &env.get(druid::theme::PRIMARY_DARK), STROKE_WIDTH);
    })
}
