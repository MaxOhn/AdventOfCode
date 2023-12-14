with open('Day_4_joshiinput.txt') as f:
    lines = [x.split() for x in f.readlines()]
pp_count_1, pp_count_2 = 0, 0
for line in lines:
    pp_count_1 += 1 if len(line) == len(set(line)) else 0
    sorted_line = [''.join(sorted(x)) for x in line]
    if len(sorted_line) == len(set(sorted_line)):
        pp_count_2 += 1
    else:
        print(''.join([word + " " for word in line])[:-1])
    #pp_count_2 += 1 if len(sorted_line) == len(set(sorted_line)) else 0
print(pp_count_1, pp_count_2)
