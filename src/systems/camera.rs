use crate::{
    CAMERA_HEIGHT, CAMERA_POSITION, CAMERA_WIDTH, WORLD,
    components::{generic::Position, player::Player},
    kill,
};

pub struct CameraSystem {}
impl CameraSystem {
    pub fn execute(world_width: u16, world_height: u16) -> anyhow::Result<()> {
        // TODO: Have the camera be an entity so it can be serialised?
        WORLD.with_borrow(|world| -> anyhow::Result<()> {
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
            let pos = player[0];
            // Clip camera to world bounds.
            CAMERA_POSITION.with_borrow_mut(|cam_pos| {
                let mut new_cam_pos = Position::new(pos.x, pos.y);
                if (cam_pos.x - CAMERA_WIDTH / 2 == 0 && pos.x < cam_pos.x)
                    || (cam_pos.x + CAMERA_WIDTH / 2 == world_width && pos.x > cam_pos.x)
                {
                    new_cam_pos.x = cam_pos.x;
                }
                if (cam_pos.y - CAMERA_HEIGHT / 2 == 0 && pos.y < cam_pos.y)
                    || (cam_pos.y + CAMERA_HEIGHT / 2 == world_height && pos.y > cam_pos.y)
                {
                    new_cam_pos.y = cam_pos.y;
                }
                *cam_pos = new_cam_pos;
            });
            Ok(())
        })
    }
}
