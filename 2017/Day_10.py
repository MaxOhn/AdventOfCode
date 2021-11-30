import numpy as np

# Part 1:
lengths = [int(n) for n in open('Day_10_list', 'r').read().split(',')]
numbers_1 = list(range(0, 256))
curr_pos, skip_size = 0, 0
for l in lengths:
    to_reverse = []
    for x in range(l):
        n = (curr_pos + x) % 256
        to_reverse.append(numbers_1[n])
    to_reverse.reverse()
    for x in range(l):
        n = (curr_pos + x) % 256
        numbers_1[n] = to_reverse[x]
    curr_pos += (l + skip_size) % 256
    skip_size += 1

# Part 2:
ascii_input = [ord(c) for c in open('Day_10_list', 'r').read()] + [17, 31, 73, 47, 23]
numbers_2 = list(range(0, 256))
curr_pos, skip_size = 0, 0
for _ in range(64):
    for l in ascii_input:
        to_reverse = []
        for x in range(l):
            n = (curr_pos + x) % 256
            to_reverse.append(numbers_2[n])
        to_reverse.reverse()
        for x in range(l):
            n = (curr_pos + x) % 256
            numbers_2[n] = to_reverse[x]
        curr_pos += (l + skip_size) % 256
        skip_size += 1
knot_hash = ''.join([hex(int(np.bitwise_xor.reduce(numbers_2[16*i:16*i+16]))).split('x')[-1] for i in range(16)])

print(numbers_1[0] * numbers_1[1], knot_hash)
