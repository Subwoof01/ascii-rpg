use hecs::Entity;

use crate::{WORLD, components::player::Player};

pub mod draw;
pub mod input;
pub mod movement;
pub mod save_load;

pub fn get_player_entity() -> Option<Entity> {
    WORLD.with_borrow(|world| {
        let mut player = None;
        for (entity, _player) in world.query::<&Player>().iter() {
            player = Some(entity);
        }
        player
    })
}
