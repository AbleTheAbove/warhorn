use rltk::{GameState, RGB, Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component, Debug)]
pub struct Player {}

pub fn register_components(mut ecs: World) -> World {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<LeftMover>();
    ecs.register::<Player>();

    ecs
}
