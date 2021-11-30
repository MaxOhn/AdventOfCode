with open('Day_16_dance') as f:
    moves = [m.rstrip().split(',') for m in f.readlines()][0]
programs = [chr(i) for i in range(ord('a'), ord('p') + 1)]
seen = [''.join(programs)]
iterations = 1000000000


def dance(programs):
    p = programs.copy()
    while 1:
        for m in moves:
            s = m[1:].split('/')
            if m[0] == 's':
                d = int(s[0])
                p = list(p[-d:]) + list(p[:-d])
            elif m[0] == 'x':
                d = [int(n) for n in s]
                p[d[0]], p[d[1]] = p[d[1]], p[d[0]]
            elif m[0] == 'p':
                d = [p.index(n) for n in s]
                p[d[0]], p[d[1]] = p[d[1]], p[d[0]]
        yield ''.join(p)


D = dance(programs)
counter = 0
while True:
    np = next(D)
    counter += 1
    if np in seen:
        break;
    else:
        seen.append(np)

D = dance(programs)
remaining_loops = iterations % counter
for _ in range(remaining_loops):
    p = next(D)
print(seen[1], p)

