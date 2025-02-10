use godot::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum Input {
  MoveUp,
  MoveDown,
  MoveLeft,
  MoveRight,
}