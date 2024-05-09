use godot::engine::utilities::move_toward;
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, ProjectSettings};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    #[export]
    speed: f64,
    #[export]
    jump_force: f64,
    gravity: f64,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 300.0,
            jump_force: -400.0,
            gravity: f64::from_variant(&ProjectSettings::singleton().get_setting("physics/2d/default_gravity".into())),

            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut anim = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if !self.base().is_on_floor() {
            let mut velocity = self.base().get_velocity();
            velocity.y += (self.gravity * delta) as f32;
            self.base_mut().set_velocity(velocity);
        }

        if Input::singleton().is_action_just_pressed("jump".into()) && self.base().is_on_floor() {
            let mut velocity = self.base().get_velocity();
            velocity.y = self.jump_force as f32;
            self.base_mut().set_velocity(velocity);
        }

        if self.base().get_velocity().x > 0.0 {
            anim.set_flip_h(false);
        } else if self.base().get_velocity().x < 0.0 {
            anim.set_flip_h(true);
        }

        let direction = Input::singleton().get_axis("move_left".into(), "move_right".into());
        if direction != 0.0 {
            anim.set_animation("run".into());
            let mut velocity = self.base().get_velocity();
            velocity.x = direction * self.speed as f32;
            self.base_mut().set_velocity(velocity);
        } else {
            anim.set_animation("idle".into());
            let mut velocity = self.base().get_velocity();
            velocity.x = move_toward(velocity.x as f64, 0.0, self.speed) as f32;
            self.base_mut().set_velocity(velocity);
        }

        self.base_mut().move_and_slide();
    }
}