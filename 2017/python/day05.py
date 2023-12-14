import itertools as it

with open('Day_5_jumps') as f:
    numbers = list(it.chain(*[list(map(int, x.rstrip().split())) for x in f.readlines()]))
curr_pos, counter = 0, 0
while curr_pos < len(numbers):
    factor = 1 if numbers[curr_pos] < 3 else -1
    numbers[curr_pos] += factor
    curr_pos += numbers[curr_pos]-factor
    counter += 1
print(counter)
