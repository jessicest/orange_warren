
use std::cell::RefCell;
use std::rc::Rc;

use crate::fragment::{UnitId, Shard::*, Zone, Fragment};
use crate::world::World;
use druid::widget::{Align, Flex, Label, Padding, Painter, Controller, Scroll};
use druid::{AppLauncher, Color, RenderContext, PlatformError, Widget, WindowDesc, PaintCtx, WidgetExt, Env, EventCtx, Event};

type WorldData = Rc<RefCell<WorldView>>;

struct WorldView {
    world: World,
}

impl WorldView where {
    fn new(world: World) -> Self {
        WorldView {
            world,
        }
    }

    fn step(&mut self, unit_id: &str, x: i64, y: i64) {
        let fragment = self.world.fragments
            .get_all(unit_id, "UnitIsInZone")
            .find(|f| matches!(f.shard, UnitIsInZone(_)))
            .expect("avatar should exist")
            .clone();
        self.world.fragments.remove(&fragment);
        if let UnitIsInZone(Zone(zx, zy, 1)) = fragment.shard {
            let zone = Zone(zx + x, zy + y, 1);
            self.world.fragments.add(Fragment::new(
                unit_id,
                &format!("{:?}", zone),
                &fragment.shard_name,
                UnitIsInZone(zone)));
        }
    }
}

struct KeyController {
}

impl KeyController {
    fn new() -> Self {
        KeyController {
        }
    }
}

impl <Child: Widget<WorldData>> Controller<WorldData, Child> for KeyController {
    fn event(&mut self, child: &mut Child, ctx: &mut EventCtx, event: &Event, data: &mut WorldData, env: &Env) {
        use druid::Code::*;

        match &event {
            Event::WindowConnected => ctx.request_focus(),
            Event::KeyUp(key_event) => {
                let mut world_view = data.borrow_mut();
                match key_event.code {
                    Numpad1 => world_view.step("player", -1, 1),
                    Numpad2 => world_view.step("player", 0, 1),
                    Numpad3 => world_view.step("player", 1, 1),
                    Numpad4 => world_view.step("player", -1, 0),
                    Numpad6 => world_view.step("player", 1, 0),
                    Numpad7 => world_view.step("player", -1, -1),
                    Numpad8 => world_view.step("player", 0, -1),
                    Numpad9 => world_view.step("player", 1, -1),
                    _ => {},
                }
                ctx.request_paint();
            },
            _ => child.event(ctx, event, data, env),
        }
    }
}

fn build_ui() -> impl Widget<WorldData> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(make_viewport_widget(), 1.0)
            .with_flex_child(Scroll::new(Label::new("no unit selected")).vertical(), 1.0),
    )
}

pub fn do_a_window(world: World) -> Result<(), PlatformError> {
    let world_view = Rc::new(RefCell::new(WorldView::new(world)));
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(world_view)?;
    Ok(())
}

fn make_viewport_widget() -> impl Widget<WorldData> {
    let mut grid = Flex::column();
    for y in (-4)..=(4) {
        let mut row = Flex::row();
        for x in (-4)..=(4) {
            row.add_flex_child(make_cell_widget((x, y)), 1.0);
        }
        grid.add_flex_child(row, 1.0);
    }
    grid
        .border(Color::PURPLE, 2.0)
        .controller(KeyController::new())
}

fn make_cell_widget(offset: (i64, i64)) -> impl Widget<WorldData> {
    Painter::new(move |ctx, world_view: &WorldData, env| {
        let world = &world_view.borrow().world;

        for player_fragment in world.fragments.get_all("player", "UnitIsInZone") {
            if let &UnitIsInZone(player_zone) = &player_fragment.shard {
                let zid = player_zone.adjust(offset.0, offset.1);
                for fragment in world.fragments.get_all(&format!("{:?}", zid), "UnitIsInZone") {
                    if let UnitIsInZone(_) = fragment.shard {
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
