with open('Day_2_checksum') as f:
#with open('Day_2_sateinput') as f:
    lines = [list(map(int, x.rstrip().split('\t'))) for x in f.readlines()]
    #lines = [list(map(int, x.rstrip().split(','))) for x in f.readlines()[:-1]]    # sateinput
checksum_1, checksum_2 = 0, 0
for line in lines:
    checksum_1 += max(line) - min(line)
    for j in line:
        for k in line:
            divi, divo = max(j, k), min(j, k)
            checksum_2 += int(divi/divo) if divi % divo == 0 and divi != divo else 0
checksum_2 = int(checksum_2/2)
print(checksum_1, checksum_2)
