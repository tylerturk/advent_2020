def part1():
    score = 0
    with open('input.txt') as fh:
        for line in fh.readlines():
            vals = line.split()
            if vals:
                if vals[1] == 'X':
                    score += 1
                    if vals[0] == 'A':
                        score += 3
                    elif vals[0] == 'C':
                        score += 6
                elif vals[1] == 'Y':
                    score += 2
                    if vals[0] == 'B':
                        score += 3
                    elif vals[0] == 'A':
                        score += 6
                else:
                    score += 3
                    if vals[0] == 'C':
                        score += 3
                    elif vals[0] == 'B':
                        score += 6
    print(score)


def part2():
    score = 0
    with open('input.txt') as fh:
        for line in fh.readlines():
            vals = line.split()
            if vals:
                if vals[1] == 'X':
                    if vals[0] == 'A':
                        score += 3
                    elif vals[0] == 'B':
                        score += 1
                    else:
                        score += 2
                elif vals[1] == 'Y':
                    score += 3
                    if vals[0] == 'A':
                        score += 1
                    elif vals[0] == 'B':
                        score += 2
                    else:
                        score += 3
                else:
                    score += 6
                    if vals[0] == 'A':
                        score += 2
                    elif vals[0] == 'B':
                        score += 3
                    else:
                        score += 1
    print(score)


if __name__ == '__main__':
    part1()
    part2()