with open('Day_15_generators') as f:
    starting = [int(l.rstrip().split()[-1]) for l in f.readlines()]

'''
# First attempt
next_values = starting
judge_count = 0
gen_A = []
gen_B = []
compare_counter = 0
while True:
    next_values = [(next_values[0]*16807) % 2147483647, (next_values[1]*48271) % 2147483647]
    binaries = [bin(n).split('b')[-1][-16:] for n in next_values]
    if next_values[0] % 4 == 0 and len(gen_A) < 2500000:
        gen_A.append(binaries[0])
    if next_values[1] % 8 == 0:
        gen_B.append(binaries[1])
    if len(gen_A) > 0 and len(gen_B) > 0:
        judge_count += 1 if gen_A.pop(0) == gen_B.pop(0) else 0
        compare_counter += 1
        if compare_counter == 5000000:
            break
        if compare_counter % 500000 == 0:
            print('{}: {}'.format(compare_counter, judge_count))
print(judge_count)
'''


# Second attempt
def gen_a(modulo=1):
    curr = starting[0]
    while True:
        curr = (curr * 16807) % 2147483647
        if curr % modulo == 0:
            yield bin(curr).split('b')[-1][-16:]


def gen_b(modulo=1):
    curr = starting[1]
    while True:
        curr = (curr * 48271) % 2147483647
        if curr % modulo == 0:
            yield bin(curr).split('b')[-1][-16:]


A = gen_a()
B = gen_b()
judge_count_1 = 0
for _ in range(40000000):
    judge_count_1 += 1 if next(A) == next(B) else 0
A = gen_a(4)
B = gen_b(8)
judge_count_2 = 0
for i in range(5000000):
    judge_count_2 += 1 if next(A) == next(B) else 0
print(judge_count_1, judge_count_2)
