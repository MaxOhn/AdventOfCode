stream = open('Day_9_stream').read()
result_sum, group_level, count_garbage = 0, 0, 0
in_garbage, skip_next = False, False
for char in stream:
    if not in_garbage:
        if char == '{':
            group_level += 1
            result_sum += group_level
        elif char == '}':
            group_level -= 1
        elif char == '<':
            in_garbage = True
    else:
        if not skip_next:
            if char == '!':
                skip_next = True
            elif char == '>':
                in_garbage = False
            else:
                count_garbage += 1
        else:
            skip_next = False
print(result_sum, count_garbage)
