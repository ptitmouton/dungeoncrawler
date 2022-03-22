pub use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);
    }

    if let Ok(entry) = ecs.entry_ref(want_move.entity) {
        let is_player = entry.get_component::<Player>().is_ok();
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(want_move.entity, fov.clone_dirty());

            if is_player {
                fov.visible_tiles.iter().for_each(|pos| {
                    map.revealed[map_idx(pos.x, pos.y)] = true;
                });
            }
        }

        if is_player {
            camera.move_camera(want_move.destination);
        }
    }

    commands.remove(*entity);
}
