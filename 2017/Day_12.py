with open('Day_12_pipes') as f:
    pipes = [[int(content.replace(',', '')) for content in pipe.rstrip().split()[2:]] for pipe in f.readlines()[:-1]]
added_members = set()
groups = []

def add_members(index, group):
    for m in pipes[index]:
        if m not in group:
            group.add(m)
            added_members.add(m)
            add_members(m, group)

for i in range(len(pipes)):
    if i not in added_members:
        new_group = {i}
        add_members(i, new_group)
        groups.append(new_group)
print(len(groups[0]), len(groups))
