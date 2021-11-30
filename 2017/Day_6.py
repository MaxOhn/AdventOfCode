import numpy as np

numbers, seen_patterns = [int(x) for x in open('Day_6_memorybanks').read().split()], []
while numbers not in seen_patterns:
    seen_patterns.append(numbers[:])
    m = int(np.argmax(numbers))
    redistribute, numbers[m] = numbers[m], 0
    for i in range(redistribute):
        m = 0 if m == len(numbers)-1 else m+1
        numbers[m] += 1
print(len(seen_patterns), len(seen_patterns)-seen_patterns.index(numbers))
