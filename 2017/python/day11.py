path = open('Day_11_path', 'r').read().rstrip().split(',')
directions = {'n': [0, 1, -1], 's': [0, -1, 1],
              'nw': [-1, 1, 0], 'se': [1, -1, 0],
              'ne': [1, 0, -1], 'sw': [-1, 0, 1]}
pos = [0, 0, 0]
max_dist = 0
for c in path:
    pos = [sum(x) for x in zip(pos, directions[c])]
    max_dist = max(int((abs(pos[0]) + abs(pos[1]) + abs(pos[2]))/2), max_dist)
print(int((abs(pos[0]) + abs(pos[1]) + abs(pos[2]))/2), max_dist)
