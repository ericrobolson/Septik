main (move-phase action-phase)

def {get-entity} (type-to-get {component-type})

def {move-phase} (pick-move-or-overwatch if (overwatch) then (pick-spot-to-watch) else pick-spot-to-move)



def {npc_ai} (while {get-next-character target-player} {eval {choose-action (eval {choose-target})})}