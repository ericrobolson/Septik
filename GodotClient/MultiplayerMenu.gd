extends VBoxContainer


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var previousMenu

# Called when the node enters the scene tree for the first time.
func _ready():
	hide()
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func activate(prev):
	print("multiplayer!")
	prev.hide()
	previousMenu = prev
	show()
	
	pass


func _on_Back_pressed():
	hide()
	previousMenu.show()
