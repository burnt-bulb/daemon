extends CharacterBody2D

const SPEED = 300.0

func _physics_process(delta: float) -> void:

	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	var vert_direction := Input.get_axis("Move Up", "Move Down")
	var hor_direction := Input.get_axis("Move Left", "Move Right")
	
	var direction := Vector2(hor_direction, vert_direction).normalized()
	
	if direction.y:
		velocity.y = direction.y * SPEED
	else:
		velocity.y = move_toward(velocity.y, 0, SPEED)
		
	if direction.x:
		velocity.x = direction.x * SPEED
	else:
		velocity.x = move_toward(velocity.x, 0, SPEED)

	move_and_slide()
