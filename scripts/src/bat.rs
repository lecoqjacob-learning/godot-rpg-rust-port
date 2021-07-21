use gdnative::api::*;
use gdnative::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::utils::load_scene;

// Bat "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat {
    #[property(default = 300.0)]
    acceleration: f32,
    #[property(default = 50.0)]
    max_speed: f32,
    #[property(default = 200.0)]
    friction: f32,
    #[property(default = 4)]
    wander_target_range: i32,

    velocity: Vector2,
    knockback: Vector2,
    stats: Ref<Node>,
    effect_scene_load: Ref<PackedScene>,
    state: BatState,
    // player_detecion_zone: Ref<Node>,
    sprite: Ref<Node>,
    player: Ref<Node>,
    hurtbox: Ref<Node>,
    soft_collision: Ref<Node>,
    wander_controller: Ref<Node>,
    animation_player: Ref<Node>,
}

enum BatState {
    Idle,
    Wander,
    Chase,
}

// Bat Implementation
#[gdnative::methods]
impl Bat {
    // The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            acceleration: 300.0,
            max_speed: 50.0,
            friction: 200.0,
            wander_target_range: 4,

            velocity: Vector2::zero(),
            knockback: Vector2::zero(),

            stats: Node::new().into_shared(),
            effect_scene_load: PackedScene::new().into_shared(),
            state: BatState::Idle,
            // player_detecion_zone: Node::new().into_shared(),
            sprite: Node::new().into_shared(),
            player: Node::new().into_shared(),
            hurtbox: Node::new().into_shared(),
            soft_collision: Node::new().into_shared(),
            wander_controller: Node::new().into_shared(),
            animation_player: Node::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        // Loading scene
        let effect_scene_load = load_scene("res://Effects/EnemyDeathEffect.tscn");
        match effect_scene_load {
            Some(_scene) => self.effect_scene_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }

        // Access to `Stats` node
        self.stats = owner.get_node("Stats").expect("Stats node should exist");
        let stats = unsafe { self.stats.assume_safe() };

        // Connecting to signal
        stats
            .connect(
                "no_health",
                owner,
                "_on_stats_no_health",
                VariantArray::new_shared(),
                1,
            )
            .unwrap();

        // Set `max_health` and `health` variable in `Stats` node
        // stats.set("max_health", 2);
        stats.set("health", stats.get("max_health"));

        // Access to `PlayerDetectionZone` node
        // self.player_detecion_zone = owner
        //     .get_node("PlayerDetectionZone")
        //     .expect("Stats node should exist");

        // Access to `AnimatedSprite` node
        self.sprite = owner
            .get_node("AnimatedSprite")
            .expect("AnimatedSprite node should exist");

        let sprite = unsafe { self.sprite.assume_safe() };
        let sprite = sprite
            .cast::<AnimatedSprite>()
            .expect("Node should cast to AnimatedSprite");

        let mut rng = Pcg64::from_rng(thread_rng()).unwrap();
        sprite.set_frame(rng.gen_range(0..4));

        // Access to `Hurtbox` node
        self.hurtbox = owner
            .get_node("Hurtbox")
            .expect("Hurtbox node should exist");

        // Access to `SoftCollision` node
        self.soft_collision = owner
            .get_node("SoftCollision")
            .expect("SoftCollision node should exist");

        // Access to `WanderController` node
        self.wander_controller = owner
            .get_node("WanderController")
            .expect("WanderController node should exist");

        self.state = self.pick_random_state(&mut vec![BatState::Idle, BatState::Wander]);

        // Access `AnimationPlayer` node
        self.animation_player = owner
            .get_node("AnimationPlayer")
            .expect("AnimationPlayer node Should Exist");
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        self.knockback = self
            .knockback
            .move_towards(Vector2::zero(), 200.0 * delta as f32);

        self.knockback = owner.move_and_slide(
            self.knockback,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );

        match self.state {
            BatState::Idle => {
                self.velocity = self
                    .velocity
                    .move_towards(Vector2::zero(), self.friction * delta as f32);

                let wander_controller = unsafe { self.wander_controller.assume_safe() };
                if unsafe { wander_controller.call("get_time_left", &[]).to_f64() } == 0.0 {
                    self.update_wander();
                }
            }
            BatState::Wander => {
                let wander_controller = unsafe { self.wander_controller.assume_safe() };
                if unsafe { wander_controller.call("get_time_left", &[]).to_f64() } == 0.0 {
                    self.update_wander();
                }

                // self.accelerate_towards_point(
                //     owner,
                //     unsafe {
                //         wander_controller
                //             .call("get_target_position", &[])
                //             .to_vector2()
                //     },
                //     delta,
                // );

                let pos = unsafe {
                    wander_controller
                        .call("get_target_position", &[])
                        .to_vector2()
                };

                self.velocity = self.velocity.move_towards(
                    owner.global_position().direction_to(pos) * self.max_speed,
                    owner.global_position().distance_to(pos) * delta as f32,
                );

                if owner.global_position().distance_to(unsafe {
                    wander_controller
                        .call("get_target_position", &[])
                        .to_vector2()
                }) <= self.wander_target_range as f32
                {
                    self.update_wander();
                }
            }
            BatState::Chase => {
                let player = unsafe { self.player.assume_safe() };
                let player = player.cast::<Node2D>().expect("Node should cast to Node2D");

                self.accelerate_towards_point(owner, player.global_position(), delta);
            }
        }

        let soft_collision = unsafe { self.soft_collision.assume_safe() };
        if unsafe { soft_collision.call("is_colliding", &[]).to_bool() } {
            self.velocity += unsafe { soft_collision.call("get_push_vector", &[]).to_vector2() }
                * delta as f32
                * 400.0;
        }

        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }

    // Accepting signal
    #[export]
    fn _on_hurtbox_area_entered(&mut self, _owner: &KinematicBody2D, area: Ref<Area2D>) {
        let stats = unsafe { self.stats.assume_safe() };
        let stats = stats.cast::<Node>().expect("Node should cast to Node");

        let area = unsafe { area.assume_safe() };

        // Update `health` variable in `Stats` node
        let health = (stats.get("health").to_i64() - area.get("damage").to_i64()).to_variant();

        unsafe {
            stats.call("set_health", &[health]);
        }

        self.knockback = area.get("knockback_vector").to_vector2() * 120.0;

        let hurtbox = unsafe { self.hurtbox.assume_safe() };
        unsafe { hurtbox.call("create_hit_effect", &[]) };
        unsafe { hurtbox.call("start_invincibility", &[(0.4).to_variant()]) };
    }

    // Accepting signal
    #[export]
    fn _on_stats_no_health(&self, owner: &KinematicBody2D) {
        //Deleting Bat node
        owner.queue_free();

        let enemy_death_effect = unsafe { self.effect_scene_load.assume_safe() };
        let enemy_death_effect = enemy_death_effect
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .expect("should be able to instance scene");

        let parent = owner.get_parent().unwrap();
        let parent = unsafe { parent.assume_safe() };
        parent.add_child(enemy_death_effect, false);

        // Accessing to DeathEffect node
        let enemy_death_effect = enemy_death_effect.to_variant();
        let enemy_death_effect = enemy_death_effect
            .try_to_object::<Node2D>()
            .expect("Should cast to Node2D");
        let enemy_death_effect = unsafe { enemy_death_effect.assume_safe() };

        // Moving position of DeathEffect
        enemy_death_effect.set_global_position(owner.global_position());
    }

    #[export]
    fn _on_player_detection_zone_body_entered(
        &mut self,
        _owner: &KinematicBody2D,
        body: Ref<Node>,
    ) {
        self.state = BatState::Chase;
        self.player = body;
    }

    #[export]
    fn _on_player_detection_zone_body_exited(
        &mut self,
        _owner: &KinematicBody2D,
        _body: Ref<Node>,
    ) {
        self.state = BatState::Idle;
        self.player = Node::new().into_shared()
    }

    #[export]
    fn _on_hurtbox_invincibility_started(&self, _owner: &KinematicBody2D) {
        let animation_player = unsafe { self.animation_player.assume_safe() };
        let animation_player = animation_player.cast::<AnimationPlayer>().unwrap();

        animation_player.play("start", -1.0, 1.0, false)
    }
    #[export]
    fn _on_hurtbox_invincibility_ended(&self, _owner: &KinematicBody2D) {
        let animation_player = unsafe { self.animation_player.assume_safe() };
        let animation_player = animation_player.cast::<AnimationPlayer>().unwrap();

        animation_player.play("stop", -1.0, 1.0, false)
    }
}

impl Bat {
    fn pick_random_state(&self, state_list: &mut Vec<BatState>) -> BatState {
        state_list.shuffle(&mut thread_rng());
        state_list.remove(0)
    }

    fn update_wander(&mut self) {
        let wander_controller = unsafe { self.wander_controller.assume_safe() };
        self.state = self.pick_random_state(&mut vec![BatState::Idle, BatState::Wander]);

        unsafe {
            wander_controller.call(
                "start_wander_timer",
                &[RandomNumberGenerator::new()
                    .randf_range(1.0, 3.0)
                    .to_variant()],
            )
        };
    }

    fn accelerate_towards_point(&mut self, owner: &KinematicBody2D, point: Vector2, delta: f64) {
        let direction = owner.global_position().direction_to(point);

        self.velocity = self
            .velocity
            .move_towards(direction * self.max_speed, self.acceleration * delta as f32);

        let sprite = unsafe { self.sprite.assume_safe() };
        let sprite = sprite
            .cast::<AnimatedSprite>()
            .expect("Node should cast to AnimatedSprite");

        sprite.set_flip_h(self.velocity.x < 0.0);
    }
}
