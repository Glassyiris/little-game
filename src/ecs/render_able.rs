use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}