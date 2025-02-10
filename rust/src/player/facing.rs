use core::f32;

use godot::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum PlayerFacingDirection {
  North,
  NorthEast,
  East,
  SouthEast,
  #[default]
  South,
  SouthWest,
  West,
  NorthWest,
}

impl PlayerFacingDirection {
  const FRAC_2_PI_3: f32 = 2.0 * f32::consts::FRAC_PI_3;
  const FRAC_5_PI_6: f32 = 5.0 * f32::consts::FRAC_PI_6;
  const NEG_PI: f32 = -f32::consts::PI;
  const NEG_FRAC_PI_3: f32 = -f32::consts::FRAC_PI_3;
  const NEG_FRAC_PI_6: f32 = -f32::consts::FRAC_PI_6;
  const NEG_FRAC_2_PI_3: f32 = -Self::FRAC_2_PI_3;
  const NEG_FRAC_5_PI_6: f32 = -Self::FRAC_5_PI_6;

  const FRAC_3_PI_4: f32 = 3.0 * f32::consts::FRAC_PI_4;
  const NEG_PI_4: f32 = -f32::consts::FRAC_PI_4;
  const NEG_FRAC_PI_2: f32 = -f32::consts::FRAC_PI_2;
  const NEG_FRAC_3_PI_4: f32 = -Self::FRAC_3_PI_4;

  pub fn from_angle(radians: f32) -> Self {
    match radians {
      0.0..=f32::consts::FRAC_PI_6 => PlayerFacingDirection::South,
      f32::consts::FRAC_PI_6..=f32::consts::FRAC_PI_3 => PlayerFacingDirection::SouthEast,
      f32::consts::FRAC_PI_3..Self::FRAC_2_PI_3 => PlayerFacingDirection::East,
      Self::FRAC_2_PI_3..=Self::FRAC_5_PI_6 => PlayerFacingDirection::NorthEast,
      Self::FRAC_5_PI_6..=f32::consts::PI => PlayerFacingDirection::North,
      Self::NEG_PI..=Self::NEG_FRAC_5_PI_6 => PlayerFacingDirection::North,
      Self::NEG_FRAC_5_PI_6..=Self::NEG_FRAC_2_PI_3 => PlayerFacingDirection::NorthWest,
      Self::NEG_FRAC_2_PI_3..=Self::NEG_FRAC_PI_3 => PlayerFacingDirection::West,
      Self::NEG_FRAC_PI_3..=Self::NEG_FRAC_PI_6 => PlayerFacingDirection::SouthWest,
      Self::NEG_FRAC_PI_6..0.0 => PlayerFacingDirection::South,
      _ => PlayerFacingDirection::default(),
    }
  }

  pub const fn as_angle(&self) -> f32 {
    match self {
      PlayerFacingDirection::North => f32::consts::PI,
      PlayerFacingDirection::NorthEast => Self::NEG_FRAC_3_PI_4,
      PlayerFacingDirection::East => Self::NEG_FRAC_PI_2,
      PlayerFacingDirection::SouthEast => Self::NEG_PI_4,
      PlayerFacingDirection::South => 0.0,
      PlayerFacingDirection::SouthWest => f32::consts::FRAC_PI_4,
      PlayerFacingDirection::West => f32::consts::FRAC_PI_2,
      PlayerFacingDirection::NorthWest => Self::FRAC_3_PI_4,
    }
  }
}
