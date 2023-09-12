#!/usr/bin/env python


def part1():
    highest = 0
    with open("input.txt") as fh:
        current = 0
        for line in fh.readlines():
            line = line.strip()
            if not line:
                if current > highest:
                    highest = current
                current = 0
            else:
                current += int(line)
    return highest


print(part1())


def part2():
    highest = 0
    calories = []
    with open("input.txt") as fh:
        current = 0
        for line in fh.readlines():
            line = line.strip()
            if not line:
                calories.append(current)
                current = 0
            else:
                current += int(line)
    calories.sort()
    return calories[-1] + calories[-2] + calories[-3]


print(part2())
