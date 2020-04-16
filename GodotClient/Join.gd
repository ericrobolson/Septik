extends Button

var IP_SERVER = "127.0.0.1" #"34.202.96.244"
var PORT_SERVER = 3400
var PORT_CLIENT = 1509
var socketUDP = PacketPeerUDP.new()
var cron_send = 0

# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


func start_client():
	if (socketUDP.listen(PORT_CLIENT, IP_SERVER) != OK):
		printt("Error listening on port: " + str(PORT_CLIENT) + " in server: " + IP_SERVER)
	else:
		printt("Listening on port: " + str(PORT_CLIENT) + " in server: " + IP_SERVER)


func _exit_tree():
	socketUDP.close()


func _pressed():
	activate()

func activate():
	start_client()

func _process(delta):
	if cron_send > 0:
		cron_send -= delta
	if cron_send <= 0:
		if socketUDP.is_listening():
			 socketUDP.set_dest_address(IP_SERVER, PORT_SERVER)
			 var stg = "hi server!"
			 var pac = stg.to_ascii()
			 socketUDP.put_packet(pac)
			 print("send!")
			 cron_send = 3

	if socketUDP.get_available_packet_count() > 0:
		var array_bytes = socketUDP.get_packet()
		printt("msg server: " + array_bytes.get_string_from_ascii())
