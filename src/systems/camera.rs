use crate::{
    CAMERA_HEIGHT, CAMERA_POSITION, CAMERA_WIDTH, WORLD,
    components::{
        camera::Camera,
        generic::{Position, Size},
        player::Player,
    },
    kill,
};

pub struct CameraSystem {}
impl CameraSystem {
    pub fn execute(world_width: u16, world_height: u16) -> anyhow::Result<()> {
        // TODO: Have the camera be an entity so it can be serialised?
        WORLD.with_borrow_mut(|world| -> anyhow::Result<()> {
            // Find player to center the camera on.
            let player = world
                .query::<(&Player, &Position)>()
                .iter()
                .map(|(_id, (&_pl, &pos))| pos)
                .collect::<Vec<_>>();
            if player.len() == 0 {
                eprintln!("Player not found! This shouldn't be able to happen...");
                kill();
                return Ok(());
            }
            let player_pos = player[0];
            for (_id, (_camera, position, size)) in
                world.query_mut::<(&Camera, &mut Position, &Size)>()
            {
                let mut new_position = Position::new(player_pos.x, player_pos.y);
                if (position.x - size.width / 2 == 0 && player_pos.x < position.x)
                    || (position.x + size.width / 2 == world_width && player_pos.x > position.x)
                {
                    new_position.x = position.x;
                }
                if (position.y - size.height / 2 == 0 && player_pos.y < position.y)
                    || (position.y + size.height / 2 == world_height && player_pos.y > position.y)
                {
                    new_position.y = position.y;
                }
                *position = new_position;
            }
            Ok(())
        })
    }
}

// let mut new_cam_pos = Position::new(pos.x, pos.y);
// if (cam_pos.x - CAMERA_WIDTH / 2 == 0 && pos.x < cam_pos.x)
//     || (cam_pos.x + CAMERA_WIDTH / 2 == world_width && pos.x > cam_pos.x)
// {
//     new_cam_pos.x = cam_pos.x;
// }
// if (cam_pos.y - CAMERA_HEIGHT / 2 == 0 && pos.y < cam_pos.y)
//     || (cam_pos.y + CAMERA_HEIGHT / 2 == world_height && pos.y > cam_pos.y)
// {
//     new_cam_pos.y = cam_pos.y;
// }
// *cam_pos = new_cam_pos;
