import numpy as np

keystring = open('Day_14_keystring', 'r').read()[:-1]

def calc_knot_hash(input_list):
    numbers = list(range(0, 256))
    curr_pos, skip_size = 0, 0
    for _ in range(64):
        for l in input_list:
            to_reverse = []
            for x in range(l):
                n = (curr_pos + x) % 256
                to_reverse.append(numbers[n])
            to_reverse.reverse()
            for x in range(l):
                n = (curr_pos + x) % 256
                numbers[n] = to_reverse[x]
            curr_pos += (l + skip_size) % 256
            skip_size += 1
    knot_hash = []
    for i in range(16):
        c_hex = hex(int(np.bitwise_xor.reduce(numbers[16*i:16*i+16]))).split('x')[-1]
        c_hex = '0' + c_hex if len(c_hex) == 1 else c_hex
        knot_hash.append(c_hex)
    return ''.join(knot_hash)


def get_neighbors(coords):
    neighbors = []
    for i in [-1, 0, 1]:
        for j in [-1, 0, 1]:
            if (i != 0 or j != 0) and (i == 0 or j == 0) and coords[0]+i in range(128) and coords[1]+j in range(128):
                coord = coords[0]+i, coords[1]+j
                neighbors.append(coord)    
    return neighbors            


def add_region(coords, bitmap, checked):
    for n in get_neighbors(coords):
        if not checked[n[0]][n[1]]:
            checked[n[0]][n[1]] = True
            if bitmap[n[0]][n[1]] == 1:
                add_region(n, bitmap, checked)


bitmap = []
checked_map = [[False for _ in range(256)] for _ in range(256)]
count_used = 0
for i in range(128):
    curr_list = [ord(c) for c in keystring + '-' + str(i)] + [17, 31, 73, 47, 23]
    curr_knot_hash = calc_knot_hash(curr_list)
    binary_string = ''
    for c in curr_knot_hash:
        binary_string += bin(int(c, 16))[2:].zfill(4)
    count_used += binary_string.count('1')
    bitmap.append([int(x) for x in binary_string])

num_regions = 0
for x in range(128):
    for y in range(128):
        coord = x, y
        if not checked_map[x][y]:
            checked_map[x][y] = True
            if bitmap[x][y] == 1:
                num_regions += 1
                add_region(coord, bitmap, checked_map)

print(count_used, num_regions)

