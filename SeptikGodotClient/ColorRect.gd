extends ColorRect


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

var clock = 0



# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	clock += 0.001	
	self.material.set_shader_param("lightColor", Vector3(clock,clock * 2, clock / 2))
