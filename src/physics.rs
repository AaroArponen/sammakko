use crate::na::{Point2, Vector2};
use specs::prelude::ParallelIterator;
use specs::{Component, VecStorage, WriteStorage, System, ParJoin};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2<f64>);

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, mut positions: Self::SystemData) {
        (&mut positions,)
            .par_join()
            .for_each(|pos| {
                pos.0 += Vector2::new(1,1);
            });
    }
}