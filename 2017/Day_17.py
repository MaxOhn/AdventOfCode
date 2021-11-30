steps = int(open('Day_17_spinlock', 'r').read())
memory, pos, num = [0], 0, 1
for _ in range(2017):
    pos = ((pos + steps) % num) + 1
    if pos == num:
        memory.append(num)
    else:
        memory = memory[:pos] + [num] + memory[pos:]
    pos = memory.index(num)    
    num += 1
part_1 = memory[0] if memory[-1] == 2017 else memory[memory.index(2017)+1]


def gen_follower(steps):
    pos, num = 0, 1
    while True:
        pos = ((pos + steps) % num) + 1
        if pos == 1:
            yield num
        num += 1


calc_next = gen_follower(steps)
n = 0
while n < 50000000:
    curr = n
    n = next(calc_next)
print(part_1, curr)
