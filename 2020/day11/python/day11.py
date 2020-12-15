cur_grid = []
with open("../day11.txt", "r") as fh:
    for line in fh.readlines():
        cur_grid.append([x for x in line.strip()])

immediate_cache = dict()

def immediately_adjacent_seats(grid, row, col):
    adjacent = []
    height = len(grid)
    width = len(grid[0])
    cache_key = f"{row}:{col}"
    if cache_key in immediate_cache:
        return immediate_cache[cache_key]
    for row_ind in range(row - 1, row + 2):
        for col_ind in range(col - 1, col + 2):
            if row_ind == row and col_ind == col:
                continue
            if col_ind >= 0 and col_ind < width and row_ind >= 0 and row_ind < height:
                adjacent.append((row_ind, col_ind))
    immediate_cache[cache_key] = adjacent
    return adjacent

def within_bounds(row, col):
    height = len(cur_grid)
    width = len(cur_grid[0])
    return 0 <= row < height and 0 <= col < width


def traverse(grid, loc, direction):
    while True:
        loc[0] += direction[0]
        loc[1] += direction[1]
        if not within_bounds(loc[0], loc[1]):
            return None
        if grid[loc[0]][loc[1]] != '.':
            return loc
    
directional_cache = dict()
def directionally_adjacent_seats(grid, row, col):
    seats = []
    cache_key = f"{row}:{col}"
    if cache_key in directional_cache:
        return directional_cache[cache_key]
    for row_dir in range(-1, 2):
        for col_dir in range(-1, 2):
            if row_dir == 0 and col_dir == 0:
                continue
            seats.append(traverse(grid, [row, col], [row_dir, col_dir]))
    seats = [x for x in seats if x is not None]
    directional_cache[cache_key] = seats
    return seats

def check_someone_sits(grid, row, col, adjacency_method):
    for seat in adjacency_method(grid, row, col):
        if grid[seat[0]][seat[1]] == '#':
            return False
    return True

def check_someone_vacate(grid, row, col, adjacency_method, max_adjacent):
    occupied = 0
    for seat in adjacency_method(grid, row, col):
        if grid[seat[0]][seat[1]] == '#':
            occupied += 1
    if occupied >= max_adjacent:
        return True
    return False

def seat_change(grid, max_adjacent=5, adjacency_method=directionally_adjacent_seats):
    changed = False
    seats_to_fill = []
    seats_to_vacate = []
    row_index = 0
    col_index = 0
    for row in grid:
        col_index = 0
        for grid_loc in grid[row_index]:
            if grid_loc == 'L' and check_someone_sits(grid, row_index, col_index, adjacency_method):
                seats_to_fill.append((row_index, col_index))
            elif grid_loc == '#' and check_someone_vacate(grid, row_index, col_index, adjacency_method, max_adjacent):
                seats_to_vacate.append((row_index, col_index))
            col_index += 1
        row_index += 1
    for seat in seats_to_fill:
        changed = True
        grid[seat[0]][seat[1]] = '#'
    for seat in seats_to_vacate:
        changed = True
        grid[seat[0]][seat[1]] = 'L'
    return grid, changed

def print_grid(grid):
    print('\n'.join([''.join(x) for x in grid]))

print(len(cur_grid), len(cur_grid[0]))


def part_one(grid):
    immediate_cache = {}
    grid = [x.copy() for x in cur_grid]
    changed = True
    while changed:
        grid, changed = seat_change(grid, 4, immediately_adjacent_seats)
        # print_grid(cur_grid)

    total_sitting = 0
    for row in range(0, len(grid)):
        for col in grid[row]:
            if col == '#':
                total_sitting += 1

    print("Part 1: ", total_sitting)


part_one(cur_grid.copy())


def part_two(grid):
    immediate_cache = {}
    grid = [x.copy() for x in cur_grid]
    changed = True
    while changed:
        grid, changed = seat_change(grid, 5, directionally_adjacent_seats)
        # print_grid(cur_grid)

    total_sitting = 0
    for row in range(0, len(grid)):
        for col in grid[row]:
            if col == '#':
                total_sitting += 1

    print("Part 2: ", total_sitting)

part_two(cur_grid)