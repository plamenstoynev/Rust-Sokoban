use ggez::mint::Vector2;
use ggez::{
    conf,
    event::{self, KeyCode, KeyMods},
    graphics::{self, DrawParam, Image},
    Context, GameResult,
};
use specs::storage::GenericWriteStorage;
use specs::{
    join::Join, world::Index, Builder, Component, Entities, ReadStorage, RunNow, System,
    VecStorage, World, WorldExt, Write, WriteStorage,
};
use std::{collections::HashMap, io::Read};

use crate::components::{Player, Position};
use crate::{
    components::{Immovable, Movable},
    input_queue::InputQueue,
};

const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.key_passed.pop() {
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let imov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => match imov.get(&pos) {
                            Some(_id) => to_move.clear(),
                            None => break,
                        },
                    }
                }
            }
        }

        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}
