use core::marker::PhantomData;
use specs::{prelude::*, Component};

pub mod nodes;

pub trait Node<const I: usize, const O: usize> {
    fn calculate_state(inputs: [bool; I]) -> [bool; O];
}

#[derive(Component)]
pub struct Connected<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    pub node: PhantomData<N>,
    pub inputs: [Option<Entity>; I],
    pub outputs: [Option<Entity>; O],
}

impl<N, const I: usize, const O: usize> Default for Connected<N, I, O>
where
    N: Node<I, O> + 'static,
{
    fn default() -> Self {
        Connected {
            node: PhantomData::<N>,
            inputs: [None; I],
            outputs: [None; O],
        }
    }
}

#[derive(Component, Default)]
pub struct Wire {
    pub input_state: bool,
    pub output_state: bool,
}

impl Node<1, 1> for Wire {
    fn calculate_state(i: [bool; 1]) -> [bool; 1] {
        i
    }
}