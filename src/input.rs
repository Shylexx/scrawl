pub mod events;
pub mod key;

use crate::input::key::Key;

pub enum InputEvent {
    Input(Key),
    Tick,
}
