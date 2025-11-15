extends Node2D

@onready var background = $DreamEnvironment/Background

func _input(event):
	if event is InputEventKey and event.pressed and event.keycode == KEY_E:
		background.color = Color(randf(), randf(), randf(), 1)