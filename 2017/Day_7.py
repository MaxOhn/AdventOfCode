import numpy as np

with open('Day_7_programs') as f:
    lines = [x.rstrip().split() for x in f.readlines()]


# Integer index leading to argument string in variables lines
def find_index(name):
    for k in range(len(lines)):
        if lines[k][0] == name:
            return k


# True if no children
def is_leaf(name):
    return len(lines[find_index(name)]) == 2


# List of string containing names of children
def get_children(name):
    return [] if is_leaf(name) else lines[find_index(name)][3:]


# 0 for leafs, minimum child level + 1 otherwise
def get_level(name):
    return 0 if is_leaf(name) else min([get_level(c) for c in get_children(name)])+1


# Recursively calculate tower_value of children
def tower_value(name):
    index = find_index(name)
    out = 0
    if len(lines[index]) > 2:
        for child in lines[index][3:]:
            out += tower_value(child)
        return out + lines[index][1]
    else:
        return lines[index][1]


# List containing tower values of children
def get_child_values(name):
    child_values = []
    for child in get_children(name):
        child_values.append(tower_value(child))
    return child_values


# True if all children have same value, else False
def is_balanced(name):
    c_v = get_child_values(name)
    return len(set(c_v)) == 1 or len(set(c_v)) == 0


all_nodes, all_children = [], []
for i in range(len(lines)):
    all_nodes += [lines[i][0]]
    lines[i][1] = int(lines[i][1][1:-1])
    if len(lines[i]) > 2:
        for j in range(3, len(lines[i])):
            lines[i][j] = lines[i][j].replace(',', '')
        all_children += lines[i][3:]
difference = list(set(all_nodes)-set(all_children))[0]

for i in range(len(lines)):
    if not is_leaf(lines[i][0]):
        lines[i][2] = get_level(lines[i][0])

non_leafs = [line for line in lines if len(line) > 2]
potential_smallest = [line for line in non_leafs if not is_balanced(line[0])]
remove_lines = []
for i in potential_smallest:
    l1 = [line[0] for line in potential_smallest]
    s1 = set(l1)
    s2 = set(get_children(i[0]))
    if not s1.isdisjoint(s2):
        remove_lines.append(i)
potential_smallest = [line for line in potential_smallest if line not in remove_lines][0]
c_values = get_child_values(potential_smallest[0])
min_v, max_v = min(c_values), max(c_values)
diff = abs(min_v-max_v)
if c_values.count(min_v) > c_values.count(max_v):
    print(difference, lines[find_index(potential_smallest[3+c_values.index(max_v)])][1] - diff)
else:
    print(difference, lines[find_index(potential_smallest[3+c_values.index(max_v)])][1] + diff)
