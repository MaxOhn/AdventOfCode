with open('Day_13_firewall') as f:
    layers = [[int(layer.rsplit()[0].replace(':', '')), int(layer.rsplit()[1]), 1, True] for layer in f.readlines()]

'''
# First attempt:
move_scanners = False
monitored = [l[0] for l in layers]
pos = -1
severity = 0
while pos < max(monitored):
    if move_scanners:
        for i in range(len(layers)):
            if layers[i][3]:
                if layers[i][2] == layers[i][1]:
                    layers[i][3] = False
                    layers[i][2] -= 1
                else:
                    layers[i][2] += 1
            else:
                if layers[i][2] == 1:
                    layers[i][3] = True
                    layers[i][2] += 1
                else:
                    layers[i][2] -= 1
    else:
        pos += 1
        if pos in monitored:
            layer = [l for l in layers if l[0] == pos][0]
            if layer[2] == 1:
                print('Caught in layer {}'.format(layer))
                severity += layer[0]*(layer[1])
                print(layer[0])
    move_scanners = not move_scanners
print(severity)
'''

# Second attempt:
severity = 0
for layer in layers:
    if layer[0] % ((layer[1]-1)*2) == 0:
        severity += layer[0]*layer[1]

'''
# First attempt:
monitored = [l[0] for l in layers]
for i in range(len(layers)):
    layers[i][2] = 1
    layers[i][3] = True
move_scanners = False
pos = -1
delayed_steps = 0
delay_counter = 0
counter = 0
curr_max = 0
while pos < max(monitored):
    if delay_counter > 0 and move_scanners is False:
        delay_counter -= 1
        move_scanners = True
    if move_scanners:
        for i in range(len(layers)):
            if layers[i][3]:
                if layers[i][2] == layers[i][1]:
                    layers[i][3] = False
                    layers[i][2] -= 1
                else:
                    layers[i][2] += 1
            else:
                if layers[i][2] == 1:
                    layers[i][3] = True
                    layers[i][2] += 1
                else:
                    layers[i][2] -= 1
    else:
        pos += 1
        if pos in monitored:
            layer = [l for l in layers if l[0] == pos][0]
            curr_max = max(curr_max, layer[0])
            if layer[2] == 1:
                print('Caught in layer {} at delay {}'.format(layer, delayed_steps))
                delayed_steps += 1
                delay_counter = delayed_steps
                move_scanners = True
                pos = -1
                for i in range(len(layers)):
                    layers[i][2] = 1
                    layers[i][3] = True
    move_scanners = not move_scanners
print(delayed_steps)
'''

# Second attempt:
delay, checking = 0, True
while checking:
    checking = False
    for layer in layers:
        if (layer[0]+delay) % ((layer[1]-1)*2) == 0:
            checking = True
            delay += 1
            break
print(severity, delay)
