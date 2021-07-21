use gdnative::api::*;
use gdnative::prelude::*;

// PlayerDetecionZone "class".
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct PlayerDetectionZone {
    #[property]
    player: Ref<Node>,
}

#[gdnative::methods]
impl PlayerDetectionZone {
    pub fn new(_owner: &Area2D) -> Self {
        PlayerDetectionZone {
            player: Node::new().into_shared(),
        }
    }

    #[export]
    fn can_see_player(&self, _owner: &Area2D) -> bool {
        let player = unsafe { self.player.assume_safe() };
        player.name() == GodotString::from_str("Player")
    }

    #[export]
    fn _on_player_detection_zone_body_entered(&mut self, _owner: &Area2D, body: Ref<Node>) {
        self.player = body;
    }

    #[export]
    fn _on_player_detection_zone_body_exited(&mut self, _owner: &Area2D, _body: Ref<Node>) {
        self.player = Node::new().into_shared()
    }
}
