use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
pub use visibility_system::VisibilitySystem;

pub struct State {
    pub ecs: World,
}

impl GameState for State {

    // Specs uses a hierarchical bitset to implement searching for entities
    // filling desired properties quickly. The join() lookup should be O(n), 
    // where n is the number of entities, I think. The memory is also all in
    // the same location. (again, I think)

    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();
        let mut map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);


        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join(){
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

    }
}

impl State {
    fn run_systems(&mut self) {
        // let mut lw = Walker{};  // Remember, this is a _system_, not entity
        // lw.run_now(&self.ecs);
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // gs.ecs.register::<Mover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs
        .create_entity()
        .with(Position{ x: player_x, y: player_y })
        .with(Renderable{ 
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8 })
        .build();
    rltk::main_loop(context, gs)
}
