use gdnative::api::*;
use gdnative::prelude::*;

// Hitbox "class".
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Hitbox {
    #[property]
    knockback_vector: Vector2,
    #[property(default = 1)]
    pub damage: i64,
}

#[gdnative::methods]
impl Hitbox {
    pub fn new(_owner: &Area2D) -> Self {
        Hitbox {
            knockback_vector: Vector2::zero(),
            damage: 1,
        }
    }

    #[export]
    pub fn get_hitbox_damage(&self, _owner: &Area2D) -> i64 {
        self.damage
    }
}
