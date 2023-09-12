static_mapping = {
    "a": 1,
    "b": 2,
    "c": 3,
    "d": 4,
    "e": 5,
    "f": 6,
    "g": 7,
    "h": 8,
    "i": 9,
    "j": 10,
    "k": 11,
    "l": 12,
    "m": 13,
    "n": 14,
    "o": 15,
    "p": 16,
    "q": 17,
    "r": 18,
    "s": 19,
    "t": 20,
    "u": 21,
    "v": 22,
    "w": 23,
    "x": 24,
    "y": 25,
    "z": 26,
    "A": 27,
    "B": 28,
    "C": 29,
    "D": 30,
    "E": 31,
    "F": 32,
    "G": 33,
    "H": 34,
    "I": 35,
    "J": 36,
    "K": 37,
    "L": 38,
    "M": 39,
    "N": 40,
    "O": 41,
    "P": 42,
    "Q": 43,
    "R": 44,
    "S": 45,
    "T": 46,
    "U": 47,
    "V": 48,
    "W": 49,
    "X": 50,
    "Y": 51,
    "Z": 52
}


def load_file(file_name):
    with open(file_name, "r") as fh:
        return fh.readlines()


def part1(file_name="input.txt"):
    """
    >>> part1("sample.txt")
    157
    >>>
    """
    contents = load_file(file_name)
    value = 0
    for line in contents:
        all_items = list(line.strip())
        size = int(len(all_items) / 2)
        compartment_one = set(all_items[0:size])
        compartment_two = set(all_items[size:])
        shared = compartment_one & compartment_two
        value += static_mapping.get(shared.pop())
    return value


def part2(file_name="input.txt"):
    """
    >>> part2("sample.txt")
    70
    >>>
    """
    contents = load_file(file_name)
    value = 0
    for ind, line in enumerate(contents):
        if ind == 0 or ind % 3 == 0:
            elf_one = set(line.strip())
            elf_two = set(contents[ind+1].strip())
            elf_three = set(contents[ind+2].strip())
            shared_item = elf_one & elf_two & elf_three
            value += static_mapping.get(shared_item.pop())
    return value


if __name__ == '__main__':
    print(part1())
    print(part2())
