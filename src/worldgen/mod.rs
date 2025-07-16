pub mod bsp;

use hecs::World;

pub trait GenerationStrategy {
    fn step(&mut self, current_step: u16);
    fn finish(&mut self) -> World;
}

pub struct DungeonGenerator {
    pub world: World,
    pub strategy: dyn GenerationStrategy,
}
