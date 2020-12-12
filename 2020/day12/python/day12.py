with open("../day12.txt", "r") as fh:
    content = fh.readlines()

facing_options = ["N", "E", "S", "W"]
facing = 1


def adjust_facing(facing_delta):
    facing_delta = abs(facing_delta)
    first_val = way_point[0]
    second_val = way_point[1]
    if facing_delta == 1:
        way_point[0] = second_val
        way_point[1] = first_val * -1
    elif facing_delta == 2:
        way_point[0] = first_val * -1
        way_point[1] = second_val * -1
    elif facing_delta == 3:
        way_point[0] = second_val * -1
        way_point[1] = first_val


way_point = [10, 1]
ship_loc = [0, 0]
print(ship_loc, way_point)
for line in content:
    direction = line[0]
    dist = int(line[1:])
    if direction == "F":
        ship_loc[0] = ship_loc[0] + dist * way_point[0]
        ship_loc[1] = ship_loc[1] + dist * way_point[1]
    if direction == "E":
        way_point[0] += dist
    elif direction == "W":
        way_point[0] -= dist
    elif direction == "N":
        way_point[1] += dist
    elif direction == "S":
        way_point[1] -= dist
    elif direction == "L":
        rotation = (360 - dist) / 90
        adjust_facing(rotation)
        facing -= rotation
        if facing < 0:
            facing += 4
    elif direction == "R":
        rotation = dist / 90
        adjust_facing(rotation)
        facing += rotation
        if facing >= len(facing_options):
            facing = facing % len(facing_options)

manhattan_distance = abs(ship_loc[0]) + abs(ship_loc[1])
print("Manhattan Distance: ", manhattan_distance)