
use std::rc::Rc;

use crate::fragment::{Fragment, UnitId, Shard::*, self};
use crate::squares::Pos;
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
                    .with_flex_child(Align::centered(make_viewport_widget((0, 0))), 1.0),
                1.0,
            ),
    )
}

pub fn do_a_window(world: World) -> Result<(), PlatformError> {
    let colors = (
        Color::rgba8(11, 99, 120, 127),
        Color::rgba8(88, 22, 11, 127),
    );
    let world_view = Rc::new(WorldView::new(world));
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(world_view)?;
    Ok(())
}

fn make_viewport_widget(offset: (i64, i64)) -> Painter<Rc<WorldView>> {
    Painter::new(move |ctx, world_view: &Rc<WorldView>, env| {
        let world = &world_view.world;

        for u0_fragment in world.fragments.get_all("u0") {
            if let &UnitIsInZone(u0_zone) = &u0_fragment.shard {
                let zid = u0_zone.adjust(offset.0, offset.1);
                for fragment in world.fragments.get_all(&format!("{:?}", zid)) {
                    if let UnitIsInZone(zone) = fragment.shard {
                        paint_unit(ctx, world, &fragment.a);
                    }
                }
            }
        }
    })
}

fn paint_unit<'a, 'b, 'c>(ctx: &mut PaintCtx<'a, 'b, 'c>, world: &World, uid: &UnitId) {
    let bounds = ctx.size().to_rect().inset(-4.0);
    let rounded = bounds.to_rounded_rect(3.0);
    ctx.fill(rounded, &Color::LIME);
    ctx.stroke(rounded, &Color::BLUE, 2.0);
}
