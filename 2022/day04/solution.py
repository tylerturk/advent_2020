def load_file(file_name):
    with open(file_name, "r") as fh:
        return fh.readlines()


def part1(file_name="input.txt"):
    """
    >>> part1("sample.txt")
    2
    """
    contained = 0
    for line in load_file(file_name):
        line = line.strip()
        pairs = line.split(",")
        range_one = [int(x) for x in pairs[0].split("-")]
        range_two = [int(x) for x in pairs[1].split("-")]
        if range_one[0] <= range_two[0] and range_one[1] >= range_two[1] \
                or range_two[0] <= range_one[0] and range_two[1] >= range_one[1]:
            contained += 1
    return contained        


def part2(file_name="input.txt"):
    """
    >>> part2("sample.txt")
    4
    """
    contained = 0
    for line in load_file(file_name):
        line = line.strip()
        pairs = line.split(",")
        range_one = [int(x) for x in pairs[0].split("-")]
        range_two = [int(x) for x in pairs[1].split("-")]
        if range_one[0] <= range_two[0] and range_two[0] <= range_one[1] or \
                range_two[0] <= range_one[0] and range_one[0] <= range_two[1]:
            contained += 1
    return contained        


if __name__ == "__main__":
    print(part1())
    print(part2())