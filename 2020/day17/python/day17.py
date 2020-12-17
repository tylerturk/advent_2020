import copy
import math

def parse_content(fh):
    return fh.readlines()

def get_sample():
    with open("../sample.txt", "r") as fh:
        return parse_content(fh)


def get_problem():
    with open("../day17.txt", "r") as fh:
        return parse_content(fh)


def adjacents_with_energy(pd, start_x, start_y, start_z):
    """
    :param pd: The 3d pocket dimension
    :param loc
    """
    adjacent_energy = 0
    for x in range(start_x-1, start_x+2):
        for y in range(start_y-1, start_y+2):
            for z in range(start_z-1, start_z+2):
                if x == start_x and y == start_y and z == start_z:
                    continue
                try:
                    if pd[x][y][z] == '#':
                        adjacent_energy += 1
                except KeyError:
                    pass
    return adjacent_energy

def evaluate_pocket_dimension(pd):
    new_pd = copy.deepcopy(pd)
    x_keys = pd.keys()
    low_x_bound = int(min(x_keys)) - 1
    high_x_bound = int(max(x_keys)) + 1
    y_keys = pd[0].keys()
    low_y_bound = int(min(y_keys)) - 1
    high_y_bound = int(max(y_keys)) + 1
    z_keys = pd[0][0].keys()
    low_z_bound = int(min(z_keys)) - 1
    high_z_bound = int(max(z_keys)) + 1
    for x in range(low_x_bound, high_x_bound + 1):
        for y in range(low_y_bound, high_y_bound + 1):
            for z in range(low_z_bound, high_z_bound + 1):
                if x not in new_pd:
                    new_pd[x] = {}
                if y not in new_pd[x]:
                    new_pd[x][y] = {}
                active = False
                try:
                    active = pd[x][y][z] == '#'
                except KeyError:
                    pass
                num_active = adjacents_with_energy(pd, x, y, z)
                if active and num_active in [2, 3] or not active and num_active == 3:
                    new_pd[x][y][z] = '#'
                else:
                    new_pd[x][y][z] = '.'
    return new_pd


def build_initial_pd(build_method=get_sample):
    content = build_method()
    initial_grid = {}
    y = int(math.floor(len(content) / 2) * - 1)
    initial_x = None
    z = 0

    for line in content:
        x = int(math.floor(len(line.strip()) / 2) * -1)
        for letter in line.strip():
            if x not in initial_grid:
                initial_grid[x] = {}
            if y not in initial_grid[x]:
                initial_grid[x][y] = {}
            initial_grid[x][y][z] = letter
            x += 1
        y += 1
    return initial_grid

def determine_energy(grid):
    total_energy = 0
    x_keys = grid.keys()
    y_keys = grid[0].keys()
    z_keys = grid[0][0].keys()
    for x in x_keys:
        for y in y_keys:
            for z in z_keys:
                if grid[x][y][z] == '#':
                    total_energy += 1
    return total_energy

grid = build_initial_pd(get_problem)
iterations = 0
while iterations < 6:
   grid = evaluate_pocket_dimension(grid)
   iterations += 1
total_energy = determine_energy(grid)


print("Part 1", total_energy)