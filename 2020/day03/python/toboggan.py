import json
slopes = []
width = 0
with open("../puzzle3.txt", 'r') as fh:
    for line in fh.readlines():
        line = line.strip()
        slopes.append([x for x in line])
        width = len(line)

right = 3
down = 1

def num_trees(right, down):
    print(slopes)
    depth = len(slopes)
    print("Width: {}, Length: {}".format(width, depth))

    at_bottom = False
    trees = 0
    x_loc = 0
    y_loc = 0
    cycles = 0
    while not at_bottom:
        x_loc += right
        y_loc += down
        if x_loc >= width * (cycles + 1):
            cycles += 1
        # print(x_loc, "Actual: ", x_loc - (cycles * width), y_loc, slopes[y_loc][x_loc - (cycles * width)], "cycles: ", cycles, "cycles * width: ", cycles * width)
        #print(slopes[y_loc])
        #print(slopes[y_loc][x_loc - (cycles * width)])
        if slopes[y_loc][x_loc - (cycles * width)] == '#':
            trees += 1
        if y_loc == depth - 1:
            at_bottom = True
            break
    return trees
print(num_trees(1, 1) * num_trees(3, 1) * num_trees(5, 1) * num_trees(7, 1) * num_trees(1, 2))

#for line in slopes:
#    print("".join(line)*cycles)