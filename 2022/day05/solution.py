from collections import deque


def load_file(file_name):
    with open(file_name, "r") as fh:
        return fh.readlines()


def generate_input(file_name):
    parsing_crates = True
    crates = []
    instructions = []
    number_of_crates = 0
    for line in load_file(file_name):
        line = line.strip('\n')
        if not line or line[1] == "1":
            parsing_crates = False
            continue
        if parsing_crates:
            if number_of_crates == 0:
                number_of_crates = int(len(line) / 4)
                crates = [deque() for _ in range(0, number_of_crates + 1)]
            # Have two options here, append and reverse later or insert at start
            # Using a deque will make the insert operation O(1) instead of O(n) for lists
            for ind, chind in enumerate(range(1, len(line), 4)):
                if line[chind] != ' ':
                    crates[ind].insert(0, line[chind])
        else:
            instr_line = line.split()
            instructions.append(
                {
                    "count": int(instr_line[1]),
                    "source": int(instr_line[3]),
                    "dest": int(instr_line[5])
                }
            )
    return crates, instructions


def part1(file_name="input.txt"):
    """
    >>> part1("sample.txt")
    'CMZ'
    """
    crates, instructions = generate_input(file_name)
    for instruction in instructions:
        # Fix 0-indexing
        source = instruction.get("source") - 1
        dest = instruction.get("dest") - 1
        for _ in range(0, instruction.get("count")):
            val = crates[source].pop()
            crates[dest].append(val)
    top_line = ""
    for crate in crates:
        top_line = f"{top_line}{crate[-1]}"
    return top_line


def part2(file_name="input.txt"):
    """
    >>> part2("sample.txt")
    'MCD'
    """
    crates, instructions = generate_input(file_name)
    for instruction in instructions:
        # Fix 0-indexing
        source = instruction.get("source") - 1
        dest = instruction.get("dest") - 1
        moving = deque()
        for _ in range(0, instruction.get("count")):
            moving.insert(0, crates[source].pop())
        for val in moving:
            crates[dest].append(val)
    top_line = ""
    for crate in crates:
        top_line = f"{top_line}{crate[-1]}"
    return top_line       


if __name__ == "__main__":
    print(part1())
    print(part2())