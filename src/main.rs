use crate::components::*;
use crate::player::player_input;
use rltk::{GameState, RGB, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};


pub mod components;
pub mod map;
pub mod player;
pub mod renderer;
pub mod shapes;

use crate::renderer::draw_map;
use crate::map::TileType;
use crate::map::new_map_test;
use crate::map::new_map_rooms_and_corridors;

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

pub struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // SCREEN CLEAR
        ctx.cls();
        // GET INPUT
        player_input(self, ctx);

        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        // SCREEN FINISHED
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Warhorn").build()?;

    let mut world = World::new();
    let world = components::register_components(world);

    let mut gs = State { ecs: world };

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }
    gs.ecs.insert(new_map_rooms_and_corridors());
    //gs.ecs.insert(new_map_test());
    rltk::main_loop(context, gs)
}
