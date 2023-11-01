from collections import deque


def open_and_read_file(input_file_name: str) -> list:
    f = open(input_file_name)
    lines = f.readlines()

    results = [letter for letter in lines[0]]
    return results


def find_number_of_chars(data: list[str], distinct_chars: int) -> int:
    """
    >>> find_number_of_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)
    7
    >>> find_number_of_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)
    19
    """
    for ind in range(0, len(data) - distinct_chars + 1):
        set_of_chars = set(data[ind:ind+distinct_chars])

        if len(set_of_chars) == distinct_chars:
            return ind+distinct_chars

    raise IndexError()


def find_number_use_deque(data: list[str], distinct_chars: int) -> int:
    """
    >>> find_number_use_deque("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)
    7
    >>> find_number_use_deque("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)
    19
    """
    sliding_window = deque()
    for ind in range(0, len(data)):
        if len(sliding_window) < distinct_chars:
            sliding_window.append(data[ind])
        elif len(set(sliding_window)) == distinct_chars:
            return ind
        else:
            sliding_window.popleft()
            sliding_window.append(data[ind])



if __name__ == "__main__":
    input_data = open_and_read_file("input.txt")

    part1_ans = find_number_of_chars(input_data, 4)
    print(part1_ans)

    part2_ans = find_number_of_chars(input_data, 14)
    print(part2_ans)
