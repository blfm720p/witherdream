extends CharacterBody2D

@export var speed = 200.0

@onready var animated_sprite = $AnimatedSprite2D

var diagonal_timer = 0.0
var current_alt = false
var last_direction = "down"

func _ready():
	setup_sprite_sheet()
	animated_sprite.scale = Vector2(2, 2)
	animated_sprite.texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST
	animated_sprite.offset = Vector2(0, 1)

func setup_sprite_sheet():
	# Load the player sprite sheet
	var sprite_sheet = load("res://Assets/girl.png")
	
	# Create a SpriteFrames resource
	var sprite_frames = SpriteFrames.new()
	sprite_frames.clear_all()
	
	# Sprite sheet is 3 rows x 4 columns
	# Columns: 0=up, 1=right, 2=down, 3=left
	# Rows: 0=walk1, 1=idle, 2=walk2
	# Frame mapping:
	# 0: up walk1, 1: right walk1, 2: down walk1, 3: left walk1
	# 4: up idle, 5: right idle, 6: down idle, 7: left idle
	# 8: up walk2, 9: right walk2, 10: down walk2, 11: left walk2

	var sprite_positions = [
		# up
		["walk_up", [0, 8]],  # walk up uses frames 0 and 8
		["idle_up", [4]],     # idle up uses frame 4
		# right
		["walk_right", [1, 9]],  # walk right uses frames 1 and 9
		["idle_right", [5]],     # idle right uses frame 5
		# down
		["walk_down", [2, 10]],  # walk down uses frames 2 and 10
		["idle_down", [6]],     # idle down uses frame 6
		# left
		["walk_left", [3, 11]],  # walk left uses frames 3 and 11
		["idle_left", [7]]      # idle left uses frame 7
	]
	
	# Each sprite is 24x24 pixels
	var sprite_width = 24
	var sprite_height = 25
	
	# Create animations
	for anim_data in sprite_positions:
		var anim_name = anim_data[0]
		var frames = anim_data[1]
		
		sprite_frames.add_animation(anim_name)
		for frame_idx in frames:
			var atlas_texture = AtlasTexture.new()
			atlas_texture.atlas = sprite_sheet
			# Calculate position in 3x4 grid
			var row = frame_idx / 4
			var col = frame_idx % 4
			atlas_texture.region = Rect2(col * sprite_width, row * sprite_height, sprite_width, sprite_height)
			var duration = 1.0 if "walk" in anim_name else 1.2
			sprite_frames.add_frame(anim_name, atlas_texture, duration)
	
	# Apply sprite frames to the animated sprite
	animated_sprite.sprite_frames = sprite_frames
	animated_sprite.play("idle_down")  # Default to facing down

func _physics_process(delta):
	# Get the input direction for top-down movement
	var direction = Vector2(
		Input.get_axis("ui_left", "ui_right"),
		Input.get_axis("ui_up", "ui_down")
	)

	var anim_name = ""
	if direction.length() > 0:
		velocity = direction.normalized() * speed

		if abs(direction.x) > 0 and abs(direction.y) > 0:  # diagonal movement
			# Determine the two directions for alternation
			var dir1 = ""
			var dir2 = ""
			if direction.y < 0:  # up
				dir1 = "up"
				dir2 = "right" if direction.x > 0 else "left"
			else:  # down
				dir1 = "down"
				dir2 = "right" if direction.x > 0 else "left"

			# Alternate between the two directions
			diagonal_timer += delta
			if diagonal_timer >= 0.1:  # alternation interval
				current_alt = !current_alt
				diagonal_timer = 0
			anim_name = "walk_" + (dir2 if current_alt else dir1)
		else:
			# Cardinal movement
			anim_name = "walk_"
			if abs(direction.x) > abs(direction.y):
				# Horizontal movement
				anim_name += "right" if direction.x > 0 else "left"
			else:
				# Vertical movement
				anim_name += "down" if direction.y > 0 else "up"
	else:
		velocity = Vector2.ZERO
		diagonal_timer = 0  # reset timer when not moving

	move_and_slide()

	# Determine animation based on actual movement
	if velocity.length() > 0.1:  # actually moving
		if animated_sprite.animation != anim_name:
			animated_sprite.play(anim_name)
		# Update last direction
		if "walk_" in anim_name:
			last_direction = anim_name.replace("walk_", "")
	else:  # not moving
		var idle_anim = "idle_" + last_direction
		if animated_sprite.animation != idle_anim:
			animated_sprite.play(idle_anim)
