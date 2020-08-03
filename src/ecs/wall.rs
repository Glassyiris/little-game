use specs::{Component, NullStorage, VecStorage, World, WorldExt, Builder};


#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

