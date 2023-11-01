def load_file(file_name):
    with open(file_name, "r") as fh:
        return fh.readlines()


def part1(file_name="input.txt"):
    """
    >>> part1("sample.txt")
    7
    """
    packet = [x for x in load_file(file_name)[0].strip()]
    for ind, _ in enumerate(range(0, len(packet)-3)):
        chars = set(packet[ind:ind+4])
        if len(chars) == 4:
            return ind + 4

def part2(file_name="input.txt"):
    """
    >>> part2("sample.txt")
    19
    """
    target = 14
    packet = [x for x in load_file(file_name)[0].strip()]
    for ind, _ in enumerate(range(0, len(packet)-target+1)):
        chars = set(packet[ind:ind+target])
        if len(chars) == target:
            return ind + target



if __name__ == "__main__":
    print(part1())
    print(part2())