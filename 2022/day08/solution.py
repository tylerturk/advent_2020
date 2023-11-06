def load_file(file_name):
    with open(file_name, "r") as fh:
        return fh.readlines()


def generate_map(file_name):
    data = load_file(file_name)
    map = []
    for ind, line in enumerate(data):
        chars = [x for x in line.strip()]
        if ind == 0:
            for char in chars:
                map.append([{
                    "v": int(char),
                    "counted": False,
                }])
        else:
            for r, char in enumerate(chars):
                map[r].append({
                    "v": int(char),
                    "counted": False,
                })
    return map


def part1(file_name="input.txt"):
    """
    >>> part1("sample.txt")
    21
    """
    map = generate_map(file_name)
    x_len = len(map)
    y_len = len(map[0])
    visible = 0
    # Checking from north
    for x in range(0, x_len):
        highest = 0
        for y in range(0, y_len):
            if y == 0:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
            elif map[x][y]["v"] > highest:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
    # Checking from south
    for x in range(x_len - 1, -1, -1):
        highest = 0
        for y in range(y_len - 1, -1, -1):
            if y == y_len - 1:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
            elif map[x][y]["v"] > highest:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
    # Checking from west
    for y in range(0, y_len):
        highest = 0
        for x in range(0, x_len):
            if x == 0:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
            elif map[x][y]["v"] > highest:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
    # Checking from east
    for y in range(y_len - 1, -1, -1):
        for x in range(x_len -1, -1, -1):
            if x == x_len - 1:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
            elif map[x][y]["v"] > highest:
                highest = map[x][y]["v"]
                if not map[x][y]["counted"]:
                    visible += 1
                    map[x][y]["counted"] = True
    return visible


def search_north(map, x, start_y):
    distance = 0
    start_height = map[x][start_y]["v"]
    for y in range(start_y - 1, -1, -1):
        distance = start_y - y
        if map[x][y]["v"] >= start_height:
            break
    return distance


def search_south(map, x, start_y):
    y_len = len(map[0])
    distance = 0
    start_height = map[x][start_y]["v"]
    for y in range(start_y + 1, y_len):
        distance = y - start_y
        if map[x][y]["v"] >= start_height:
            break
    return distance


def search_east(map, start_x, y):
    distance = 0
    start_height = map[start_x][y]["v"]
    for x in range(start_x - 1, -1, -1):
        distance = start_x - x
        if map[x][y]["v"] >= start_height:
            break
    return distance


def search_west(map, start_x, y):
    x_len = len(map)
    distance = 0
    start_height = map[start_x][y]["v"]
    for x in range(start_x + 1, x_len):
        distance = x - start_x
        if map[x][y]["v"] >= start_height:
            break
    
    return distance


def part2(file_name="input.txt"):
    """
    >>> part2("sample.txt")
    8
    """
    map = generate_map(file_name)
    highest = 0
    for x in range(0, len(map)):
        for y in range(0, len(map[0])):
            north = search_north(map, x, y)
            south = search_south(map, x, y)
            east = search_east(map, x, y)
            west = search_west(map, x, y)
            value = north * south * east * west
            if value > highest:
                highest = value
    return highest


def main():
    print(part1("input.txt"))

    print(part2("input.txt"))


if __name__ == "__main__":
    main()