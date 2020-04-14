extends Button

export(NodePath) var pathToMultiplayerMenu

# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _pressed():
	var multiplayer_menu = get_node(pathToMultiplayerMenu)
		
	var parent = get_node("..")
	
	multiplayer_menu.activate(parent)
	
