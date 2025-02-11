use std::ffi::CStr;

use godot::{
  meta::{AsArg, ParamType},
  prelude::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputAction {
  MoveUp,
  MoveDown,
  MoveLeft,
  MoveRight,
}

impl InputAction {
  pub const fn cstr(self) -> &'static CStr {
    match self {
      Self::MoveUp => c"Move Up",
      Self::MoveDown => c"Move Down",
      Self::MoveLeft => c"Move Left",
      Self::MoveRight => c"Move Right",
    }
  }
}

impl AsArg<StringName> for InputAction {
  fn into_arg<'r>(self) -> <StringName as ParamType>::Arg<'r>
  where
    Self: 'r,
  {
    self.cstr().into_arg()
  }
}
