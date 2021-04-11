use crate::components::Connection;
use crate::components::{round_to_snap, Pos};
use crate::resources::MousePos;
use crate::resources::UIState;
use crate::systems::place_node_sys::PlaceNodeSys;
use macroquad::prelude::*;
use specs::prelude::*;

use crate::nodes;
pub fn handle_mouse_click(world: &mut World) {
    crate::systems::ui_systems::SwitchClickSys.run_now(world);

    let ui_state = *world.fetch::<UIState>();

    match ui_state {
        UIState::AddingWire { wire_entity, .. } => {
            // clear UIState including removing the wire entity
            let wire_placed = {
                let position_storage = world.read_storage::<Pos>();
                position_storage.get(wire_entity).is_some()
            };

            if !wire_placed {
                world.delete_entity(wire_entity).unwrap();

                crate::systems::cleanup_sys::CleanupWires.run_now(world);
            }

            world.insert(UIState::Nothing);
        }
        UIState::AddingNode(n) => {
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

            use crate::all_nodes;
            all_nodes!(place_node_systems);

            world.insert(UIState::Nothing);
        }
        UIState::Deleting => {
            let positions = world.read_storage::<Pos>();
            let entities = world.entities();
            let mouse_pos = world.fetch::<MousePos>().0;
            let connections = world.read_storage::<Connection>();
            let target = (&positions, &entities).join().find(|(pos, e)| {
                (connections.get(*e).is_none()) && (pos.pos - mouse_pos).length() < 35.0
            });
            std::mem::drop(connections);

            if let Some((_, entity)) = target {
                entities.delete(entity).unwrap();
                std::mem::drop(positions);
                std::mem::drop(entities);
                crate::systems::cleanup_sys::run_cleanup_systems(entity, world);
                world.maintain();
                crate::systems::cleanup_sys::CleanupWires.run_now(world);
            }
        }
        UIState::Nothing => {}
    }
}

pub fn handle_mouse_right_click(world: &mut World) {
    let ui_state = *world.fetch::<UIState>();

    match ui_state {
        UIState::AddingWire {
            connection_entity,
            wire_entity,
            x_pos: None,
            y_pos: Some(y_pos),
        } => {
            let mouse_pos = world.fetch::<MousePos>().0;
            world.insert(UIState::AddingWire {
                connection_entity,
                wire_entity,
                x_pos: Some(mouse_pos.x),
                y_pos: Some(y_pos),
            });
            world
                .write_storage::<Pos>()
                .insert(
                    wire_entity,
                    Pos::from_vec(Vec2::new(mouse_pos.x, round_to_snap(y_pos))),
                )
                .unwrap();
        }
        _ => {
            crate::systems::place_wire_sys::WirePlaceSys.run_now(world);
        }
    }
}
