pub mod facing;
pub mod input;

use std::collections::HashMap;

use godot::{
  classes::{CharacterBody2D, ICharacterBody2D},
  prelude::*,
};

use self::facing::PlayerFacingDirection;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
  #[export]
  speed: f32,
  #[export]
  facing: PlayerFacingDirection,
  #[export]
  model: Option<Gd<Node2D>>,
  base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
  fn init(base: Base<CharacterBody2D>) -> Self {
    Self {
      speed: 15.0,
      facing: Default::default(),
      base,
      model: None,
    }
  }

  fn physics_process(&mut self, _delta: f64) {
    let input = Input::singleton();

    let dir = input.get_vector("Move Left", "Move Right", "Move Up", "Move Down");

    if !dir.is_zero_approx() {
      self.facing = PlayerFacingDirection::from_angle(dir.angle_to(self.base().get_up_direction()));
    }

    let velocity = dir * self.speed;

    self.base_mut().set_velocity(velocity);
    self.base_mut().move_and_slide();
  }
}
