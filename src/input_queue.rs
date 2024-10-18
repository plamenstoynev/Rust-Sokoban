use ggez::event::{self, KeyCode, KeyMods};

#[derive(Default)]
pub struct InputQueue {
    pub key_passed: Vec<KeyCode>,
}
