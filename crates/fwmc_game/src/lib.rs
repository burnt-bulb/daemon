use godot::prelude::*;

pub mod player;

struct FWMCExtension;

#[gdextension]
unsafe impl ExtensionLibrary for FWMCExtension {
  fn on_level_init(_: InitLevel) {
    godot_print!("BAU BAU!");
  }
}
