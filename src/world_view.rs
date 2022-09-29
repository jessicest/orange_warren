
use std::rc::Rc;

use crate::squares::Pos;
use crate::fragment::{Fragment::*, UnitId};
use crate::world::World;
use druid::kurbo::Circle;
use druid::widget::{Align, Flex, Label, Padding, Painter};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc, PaintCtx};
use druid::{Color, RenderContext};

struct WorldView {
    world: World,
}

impl WorldView where {
    fn new(world: World) -> Self {
        WorldView {
            world,
        }
    }

}

fn build_ui() -> impl Widget<Rc<WorldView>> {
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
                    .with_flex_child(Align::centered(make_viewport_widget(Pos::new(0, 0))), 1.0),
                1.0,
            ),
    )
}

pub fn do_a_window(world: World) -> Result<(), PlatformError> {
    let colors = (
        Color::rgba8(11, 99, 120, 127),
        Color::rgba8(88, 22, 11, 127),
    );
    let world_view = WorldView::new(world);
    let world_view = Rc::new(world_view);
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(world_view)?;
    Ok(())
}

fn make_viewport_widget(offset: Pos<i64>) -> Painter<Rc<WorldView>> {
    Painter::new(move |ctx, world_view: &Rc<WorldView>, env| {
        let world = &world_view.world;

        for fragment in world.get_truths("u0") {
            if let &UnitZone(_, zid) = &fragment {
                let pos = world.zones.get(zid).unwrap().center();
                let pos = pos + offset;
                for fragment in world.get_truths(&pos.to_zone_id()) {
                    match fragment {
                        UnitZone(uid, _) => paint_unit(ctx, world, uid),
                        _ => (),
                    }
                }
            }
        }
    })
}

fn paint_unit<'a, 'b, 'c>(ctx: &mut PaintCtx<'a, 'b, 'c>, world: &World, uid: &UnitId) {
        /*
        //let radius = 4;
        //let breadth = radius * 2 + 1;

        let bounds = ctx.size().to_rect().inset(-1.0);
        //let cell_size = Size::new(ctx.size().width / breadth, ctx.size().height / breadth);

        let bounds = ctx.size().to_rect().inset(-STROKE_WIDTH / 2.0);
        let rounded = bounds.to_rounded_rect(CORNER_RADIUS);
        ctx.fill(rounded, &data.0);
        ctx.stroke(rounded, &env.get(druid::theme::PRIMARY_DARK), STROKE_WIDTH);
        let radius = f64::min(bounds.width().abs(), bounds.height().abs()) / 2.0;
        let circle = Circle::new(bounds.center(), radius);
        ctx.fill(circle, &data.1);
        ctx.stroke(circle, &env.get(druid::theme::PRIMARY_DARK), STROKE_WIDTH);
        */
}
