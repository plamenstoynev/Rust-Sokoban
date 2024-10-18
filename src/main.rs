#![warn(unused_imports)]
#![warn(unused_variables)]
#![allow(unused)]

use ggez::{
    conf,
    event::{self, KeyCode, KeyMods},
    Context, GameResult,
};
use input_queue::InputQueue;
use specs::{Builder, Component, RunNow, VecStorage, World, WorldExt};

use std::{path, sync::WaitTimeoutResult};

mod components;
mod input_queue;
mod input_system;
mod rendering_system;
struct Game {
    world: World,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let mut is = input_system::InputSystem {};
        is.run_now(&self.world);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = rendering_system::RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let mut input = self.world.write_resource::<InputQueue>();
        input.key_passed.push(keycode);
    }
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);
    register_resources(&mut world);
    rendering_system::initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    let game = Game { world };

    event::run(context, event_loop, game)
}
