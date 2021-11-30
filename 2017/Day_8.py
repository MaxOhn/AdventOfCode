with open('Day_8_instructions') as f:
    lines = [line.rsplit() for line in f.readlines()]
registers = {line[0]: 0 for line in lines}


def eval_condition(condition):
    if condition[1] == '==':
        return registers[condition[0]] == int(condition[2])
    elif condition[1] == '!=':
        return registers[condition[0]] != int(condition[2])
    elif condition[1] == '<=':
        return registers[condition[0]] <= int(condition[2])
    elif condition[1] == '>=':
        return registers[condition[0]] >= int(condition[2])
    elif condition[1] == '<':
        return registers[condition[0]] < int(condition[2])
    elif condition[1] == '>':
        return registers[condition[0]] > int(condition[2])


def eval_instr(line):
    if line[1] == 'inc':
        registers[line[0]] += int(line[2]) if eval_condition(line[4:]) else 0
    elif line[1] == 'dec':
        registers[line[0]] -= int(line[2]) if eval_condition(line[4:]) else 0
    else:
        print('Neither inc nor dec')


curr_max = 0
for line in lines:
    eval_instr(line)
    reg_max = max(registers.values())
    curr_max = reg_max if reg_max > curr_max else curr_max

print(max(registers.values()), curr_max)
