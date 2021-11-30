import heapq

memory_number = int(open('Day_3_number', 'r').read())
ring_number, max_number_in_ring = 1, 1
while max_number_in_ring < memory_number:
    ring_number += 1
    max_number_in_ring = (2*ring_number-1)**2
numbers_in_ring = 2*(2*ring_number-1)+2*(2*ring_number-3)
numbers_in_edge = int(numbers_in_ring/4)
edge = 0
while max_number_in_ring-edge*numbers_in_edge > memory_number:
    edge += 1
edge -= 1
closest_corner = max_number_in_ring-edge*numbers_in_edge if max_number_in_ring-edge*numbers_in_edge-memory_number < int((numbers_in_edge-1)/2) else max_number_in_ring-(edge+1)*numbers_in_edge
away_from_corner = closest_corner - memory_number if closest_corner - memory_number > 0 else memory_number - closest_corner
way_to_mid = int(numbers_in_edge/2)-away_from_corner
total_distance = ring_number-1+way_to_mid

#memory_number = 807
outer_numbers = [1, 2, 4, 5, 10, 11, 23, 25]
idx, first, max_idx, curr = 0, False, 3, 25
while curr < memory_number:
    curr += outer_numbers[0]
    if idx == 1:
        curr += outer_numbers[1] + outer_numbers[-2]
    elif idx == max_idx - 1:
        curr += outer_numbers[1]
        outer_numbers.pop(0)
        idx = -1
        if not first:
            max_idx += 1
        first = not first
    elif idx > 0 and idx < max_idx:
        curr += outer_numbers[1] + outer_numbers[2]
        outer_numbers.pop(0)
    outer_numbers.append(curr)
    idx += 1

print(total_distance, curr)
