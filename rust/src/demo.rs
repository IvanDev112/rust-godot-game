use godot::prelude::*;
use godot::engine::{Sprite2D, ISprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Demo {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Demo {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,

            base,
        }
    }

    fn ready(&mut self) {
        let screen_size = self.base().get_viewport_rect().size;
        let new_position = Vector2 { x: screen_size.x / 2.0, y: screen_size.y / 2.0 };
        self.base_mut().set_position(new_position);
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}