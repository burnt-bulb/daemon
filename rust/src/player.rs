pub mod facing;
pub mod input;

use std::collections::HashMap;

use godot::{
  classes::{CharacterBody3D, ICharacterBody3D},
  prelude::*,
};

use self::facing::PlayerFacingDirection;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
  #[export]
  speed: f32,
  #[export]
  facing: PlayerFacingDirection,
  #[export]
  model: Option<Gd<Node3D>>,
  #[export]
  input: HashMap<Input, GString>,
  base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
  fn init(base: Base<CharacterBody3D>) -> Self {
    Self {
      speed: 15.0,
      facing: Default::default(),
      base,
      model: None,
    }
  }

  fn physics_process(&mut self, _delta: f64) {
    let input = Input::singleton();

    let vec2 = input
      .get_vector("Move Left".into(), "Move Right".into(), "Move Up".into(), "Move Down".into())
      .to_3d(0.0);
    let dir = swizzle!(vec2 => x, z, y);

    if !dir.is_zero_approx() {
      self.facing =
        PlayerFacingDirection::from_angle(dir.signed_angle_to(Vector3::BACK, self.base().get_up_direction()));
    }

    if let Some(model) = &mut self.model {
      model.set_rotation(Vector3::new(0.0, self.facing.as_angle(), 0.0));
    }

    let velocity = Vector3::new(dir.x * self.speed, dir.y, dir.z * self.speed);

    self.base_mut().set_velocity(velocity);
    self.base_mut().move_and_slide();
  }
}
