use fwmc_derive::ToStringName;

#[derive(ToStringName, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputAction {
  MoveUp,
  MoveDown,
  MoveLeft,
  MoveRight,
}
