use gdnative::prelude::*;

mod bat;
mod camera;
mod effect;
mod grass;
mod health_ui;
mod hitbox;
mod hurtbox;
mod player;
mod player_hurt_sound;
mod soft_collision;
mod stats;
mod utils;
mod wander_controller;
// mod sword_hitbox;
// mod player_detection_zone;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<bat::Bat>();
    handle.add_class::<camera::Camera>();
    handle.add_class::<effect::Effect>();
    handle.add_class::<grass::Grass>();
    handle.add_class::<health_ui::HealthUI>();
    handle.add_class::<hitbox::Hitbox>();
    handle.add_class::<hurtbox::Hurtbox>();
    handle.add_class::<player::Player>();
    handle.add_class::<player_hurt_sound::PlayerHurtSound>();
    handle.add_class::<soft_collision::SoftCollision>();
    handle.add_class::<stats::Stats>();
    handle.add_class::<wander_controller::WanderController>();
    // handle.add_class::<sword_hitbox::SwordHitbox>();
    // handle.add_class::<player_detection_zone::PlayerDetectionZone>();
}

// Macro that create the entry-points of the dynamic library.
godot_init!(init);
