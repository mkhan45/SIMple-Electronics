use crate::components::{Node, Orientation};
use crate::Connected;
use crate::Pos;
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;

#[derive(Default)]
pub struct PlaceNodeSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
}

impl<'a, N, const I: usize, const O: usize> System<'a> for PlaceNodeSys<N, I, O>
where
    N: Node<I, O> + 'static,
{
    type SystemData = (
        WriteStorage<'a, Connected<N, I, O>>,
        WriteStorage<'a, Pos>,
        Entities<'a>,
    );

    fn run(&mut self, (mut node_storage, mut position_storage, entities): Self::SystemData) {
        let mp = mouse_position();

        entities
            .build_entity()
            .with(Connected::<N, I, O>::default(), &mut node_storage)
            .with(
                Pos {
                    orientation: Orientation::Right,
                    pos: mp.into(),
                },
                &mut position_storage,
            )
            .build();
    }
}
