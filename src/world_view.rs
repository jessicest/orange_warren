
use std::cell::{RefCell, Ref};
use std::rc::Rc;

use crate::fragment::{Shard::*, Zone, Fragment, IdType, UnitId};
use crate::world::World;
use druid::widget::{Flex, Label, Padding, Painter, Controller, Scroll};
use druid::{AppLauncher, Color, RenderContext, PlatformError, Widget, WindowDesc, PaintCtx, WidgetExt, Env, EventCtx, Event, MouseButton, Data, lens, UnitPoint};

type TheWorld = Rc<RefCell<World>>;

impl Data for IdType {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Clone)]
struct WorldView {
    world: TheWorld,
    selected_unit_id: Option<UnitId>,
}

impl Data for WorldView {
    fn same(&self, other: &Self) -> bool {
        self.selected_unit_id == other.selected_unit_id
            && self.world.borrow().next_tick() == other.world.borrow().next_tick()
            && self.world.borrow().next_unit() == other.world.borrow().next_unit()
    }
}

impl WorldView where {
    fn new(world: TheWorld) -> Self {
        WorldView {
            world,
            selected_unit_id: None,
        }
    }

    fn move_avatar(&mut self, x: i64, y: i64) {
        let mut world = self.world.borrow_mut();
        world.queued_move = (x, y);
        world.advance();
        while world.next_unit() != "player" {
            world.advance();
        }
    }

    fn unit_is_selected(&self, a: &IdType) -> bool {
        if let Some(selected_uid) = &self.selected_unit_id {
            &a.to_string() == selected_uid
        } else {
            false
        }
    }

    fn player_zone(&self) -> Zone {
        let world = self.world.borrow();
        for player_fragment in world.fragments.get("UnitIsInZone", &IdType::from("player")) {
            if let &UnitIsInZone(player_zone) = &player_fragment.shard {
                return player_zone;
            }
        }

        panic!("zoneless player just isn't what we do here")
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

impl <Child: Widget<WorldView>> Controller<WorldView, Child> for KeyController {
    fn event(&mut self, child: &mut Child, ctx: &mut EventCtx, event: &Event, world_view: &mut WorldView, env: &Env) {
        use druid::Code::*;

        match &event {
            Event::WindowConnected => ctx.request_focus(),
            Event::KeyUp(key_event) => {
                match key_event.code {
                    Numpad1 => world_view.move_avatar(-1, 1),
                    Numpad2 => world_view.move_avatar(0, 1),
                    Numpad3 => world_view.move_avatar(1, 1),
                    Numpad4 => world_view.move_avatar(-1, 0),
                    Numpad5 => world_view.move_avatar(0, 0),
                    Numpad6 => world_view.move_avatar(1, 0),
                    Numpad7 => world_view.move_avatar(-1, -1),
                    Numpad8 => world_view.move_avatar(0, -1),
                    Numpad9 => world_view.move_avatar(1, -1),
                    _ => {},
                };
                ctx.request_update();
                ctx.request_paint();
            },
            _ => child.event(ctx, event, world_view, env),
        }
    }
}

struct RepaintOnClick {
}

impl RepaintOnClick {
    fn new() -> Self {
        RepaintOnClick {
        }
    }
}

impl <Child: Widget<WorldView>> Controller<WorldView, Child> for RepaintOnClick {
    fn event(&mut self, child: &mut Child, ctx: &mut EventCtx, event: &Event, data: &mut WorldView, env: &Env) {
        match &event {
            Event::MouseUp(_) | Event::MouseDown(_) => { ctx.request_update(); ctx.request_paint(); },
            _ => (),
        }
        child.event(ctx, event, data, env);
    }
}

struct ClickSelector {
    offset: (i64, i64),
}

impl ClickSelector {
    fn new(offset: (i64, i64)) -> Self {
        ClickSelector {
            offset,
        }
    }
}

impl <Child: Widget<WorldView>> Controller<WorldView, Child> for ClickSelector {
    fn event(&mut self, child: &mut Child, ctx: &mut EventCtx, event: &Event, world_view: &mut WorldView, env: &Env) {
        match &event {
            Event::MouseUp(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    let zone = world_view.player_zone().adjust(self.offset.0, self.offset.1);
                    let world = world_view.world.borrow();

                    let new_unit_id = world.fragments.get("UnitIsInZone", &IdType::from(zone))
                        .next()
                        .map(|f| f.a.to_string());

                    world_view.selected_unit_id = new_unit_id;
                    ctx.request_paint();
                }
            },
            _ => child.event(ctx, event, world_view, env),
        }
    }
}

fn build_ui() -> impl Widget<WorldView> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(make_viewport_widget(), 1.0)
            .with_flex_child(Flex::column()
                .with_flex_child(Scroll::new(make_fragments_panel()).vertical().border(Color::PURPLE, 2.0), 1.0)
                .with_flex_child(Scroll::new(make_tasks_panel()).align_vertical(UnitPoint::BOTTOM).border(Color::PURPLE, 2.0), 0.1)
            , 1.0)
            .controller(RepaintOnClick::new())
            .controller(KeyController::new())
    )
}

fn make_fragments_panel() -> impl Widget<WorldView> {
    Label::new(|world_view: &WorldView, _env: &_| {
        if let Some(uid) = &world_view.selected_unit_id {
            let mut result = String::new();

            for fragment in world_view.world.borrow_mut().fragments.get_all(&IdType::from(uid)) {
                result.push_str(&format!("{:#?}", fragment));
            }

            result
        } else {
            String::from("no unit selected")
        }
    })
}

fn make_tasks_panel() -> impl Widget<WorldView> {
    Label::new(|world_view: &WorldView, _env: &_| {
        if let Some(uid) = &world_view.selected_unit_id {
            let mut result = String::new();
            let world = world_view.world.borrow();

            for entry in &world.next_moves {
                if &entry.1 == uid {
                    result.push_str(&format!("{}\n", entry.0.0));
                    break;
                }
            }

            if let Some(task) = world.tasks.get(uid) {
                result.push_str(&format!("{:?}\n", task));
            }

            result
        } else {
            String::from("no unit selected")
        }
    })
}

pub fn do_a_window(world: World) -> Result<(), PlatformError> {
    let world_view = WorldView::new(Rc::new(RefCell::new(world)));
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(world_view)?;
    Ok(())
}

fn make_viewport_widget() -> impl Widget<WorldView> {
    let mut grid = Flex::column();
    for y in (-12)..=(12) {
        let mut row = Flex::row();
        for x in (-12)..=(12) {
            row.add_flex_child(make_cell_widget((x, y)), 1.0);
        }
        grid.add_flex_child(row, 1.0);
    }
    grid.border(Color::PURPLE, 2.0)
}

fn make_cell_widget(offset: (i64, i64)) -> impl Widget<WorldView> {
    Painter::new(move |ctx, world_view: &WorldView, _env| {
        let zone = world_view.player_zone().adjust(offset.0, offset.1);
        let world = world_view.world.borrow();

        if let Some(_) = world.fragments.get_one("ObjectTypeOccupiesZone", &IdType::from("tree"), &IdType::from(zone)) {
            paint_rect(ctx, &Color::LIME);
        }

        for fragment in world.fragments.get("UnitIsInZone", &IdType::from(zone)) {
            paint_rect(ctx, &Color::BLUE);
            if world_view.unit_is_selected(&fragment.a) {
                paint_border(ctx, &Color::PURPLE);
            }
        }
    }).controller(ClickSelector::new(offset))
}

fn paint_rect<'a, 'b, 'c>(ctx: &mut PaintCtx<'a, 'b, 'c>, color: &Color) {
    let bounds = ctx.size().to_rect().inset(-4.0);
    let rounded = bounds.to_rounded_rect(3.0);
    ctx.fill(rounded, color);
}

fn paint_border<'a, 'b, 'c>(ctx: &mut PaintCtx<'a, 'b, 'c>, color: &Color) {
    let bounds = ctx.size().to_rect().inset(-4.0);
    let rounded = bounds.to_rounded_rect(3.0);
    ctx.stroke(rounded, color, 2.0);
}

fn paint_unit<'a, 'b, 'c>(ctx: &mut PaintCtx<'a, 'b, 'c>, world: &World, uid: &IdType, selected_unit_id: &Option<UnitId>) {
    paint_rect(ctx, &Color::BLUE);
    if let Some(selected_unit_id) = selected_unit_id {
        if &IdType::from(selected_unit_id) == uid {
            paint_border(ctx, &Color::PURPLE);
        }
    }
}
