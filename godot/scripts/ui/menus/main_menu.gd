extends Control

signal play_game()
signal open_settings()
@onready var buttons_vbox = %ButtonsVBox

func _on_play_button_pressed() -> void:
	play_game.emit()
	hide()

func _on_settings_button_pressed() -> void:
	open_settings.emit()
	hide()

func _on_quit_button_pressed() -> void:
	get_tree().quit()

func _on_visibility_changed() -> void:
	if visible:
		focus_button()

func focus_button() -> void:
	if buttons_vbox:
		var button: Button = buttons_vbox.get_child(0)
		if button is Button:
			button.grab_focus()

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	focus_button()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
