use crate::systems::place_node_sys::PlaceNodeSys;
use crate::systems::place_wire_sys::WirePlaceSys;
use crate::{
    components::{Orientation, Pos},
    resources::AddingWire,
};
use macroquad::prelude::*;
use specs::prelude::*;

use crate::nodes;
use crate::resources;

pub fn handle_mouse_click(world: &mut World) {
    let node = world.fetch::<resources::AddingNode>();

    if let resources::AddingNode(Some(n)) = &*node {
        macro_rules! place_node_systems {
                    ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
                        #[allow(unreachable_patterns)]
                        match n {
                            $(nodes::NodeTy::$node => {
                                PlaceNodeSys::<nodes::$node, $i, $o>::default().run_now(&world)
                            })*
                            _ => todo!(),
                        }
                    };
                }

        place_node_systems!(
            [OnNode, 0, 1],
            [OffNode, 0, 1],
            [NotNode, 1, 1],
            [AndNode, 2, 1],
            [OrNode, 2, 1],
            [NandNode, 2, 1],
            [NorNode, 2, 1],
            [XorNode, 2, 1],
            [XnorNode, 2, 1]
        );

        std::mem::drop(node);
        world.insert(resources::AddingNode(None));
    }
}

pub fn handle_mouse_right_click(world: &mut World) {
    let adding_wire = world.fetch::<AddingWire>().0;
    match adding_wire {
        Some((wire_entity, None)) => {
            let (mx, my) = mouse_position();
            world.insert(AddingWire(Some((wire_entity, Some(Vec2::new(mx, my))))));
            world
                .write_storage::<Pos>()
                .insert(
                    wire_entity,
                    Pos {
                        pos: Vec2::new(mx, my),
                        orientation: Orientation::Right,
                    },
                )
                .unwrap();
        }
        _ => {
            macro_rules! place_wire_systems {
                ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
                    let initial_state = world.fetch::<AddingWire>().0.is_some();

                    $(
                        WirePlaceSys::<nodes::$node, $i, $o>::default().run_now(&world);
                        let new_state = world.fetch::<AddingWire>().0.is_some();
                        if new_state != initial_state {
                            return
                        }
                    )*
                };
            }

            place_wire_systems!(
                [OnNode, 0, 1],
                [OffNode, 0, 1],
                [NotNode, 1, 1],
                [AndNode, 2, 1],
                [OrNode, 2, 1],
                [NandNode, 2, 1],
                [NorNode, 2, 1],
                [XorNode, 2, 1],
                [XnorNode, 2, 1]
            );
        }
    };
}
