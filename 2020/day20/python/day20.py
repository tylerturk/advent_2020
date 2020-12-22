tiles = {}

def create_tiles(filename="../sample.txt"):
    with open(filename, "r") as fh:
        top = []
        left = []
        right = []
        prev_line = None
        for line in fh.readlines():
            line = line.strip()
            if not line:
                tiles[tilenum]["edges"].add(prev_line)
                bottom = [x for x in prev_line]
                bottom.reverse()
                tiles[tilenum]["edges"].add("".join(bottom))
                tiles[tilenum]["edges"].add("".join(left))
                left.reverse()
                tiles[tilenum]["edges"].add("".join(left))
                tiles[tilenum]["edges"].add("".join(right))
                right.reverse()
                tiles[tilenum]["edges"].add("".join(right))
                left = []
                right = []
                continue
            if line.startswith("Tile"):
                tilenum = line.split()[-1][:-1]
                tiles[tilenum] = {
                    "edges": set(),
                    "neighbors": set()
                }
                print(tilenum)
                at_top = True
                continue
            if at_top:
                tiles[tilenum]["edges"].add(line)
                top = [x for x in line]
                top.reverse()
                tiles[tilenum]["edges"].add("".join(top))
                at_top = False
            left.append(line[0])
            right.append(line[-1])
            prev_line = line
        # Guarantee we process the file line
        tiles[tilenum]["edges"].add(prev_line)
        bottom = [x for x in prev_line]
        bottom.reverse()
        tiles[tilenum]["edges"].add("".join(bottom))
        tiles[tilenum]["edges"].add("".join(bottom))
        tiles[tilenum]["edges"].add("".join(left))
        left.reverse()
        tiles[tilenum]["edges"].add("".join(left))
        tiles[tilenum]["edges"].add("".join(right))
        right.reverse()
        tiles[tilenum]["edges"].add("".join(right))
        left = []
        right = []


def process_tiles():
    for tile1 in tiles:
        for tile2 in tiles:
            print(tile1)
            if tile1 == tile2:
                continue
            if len(tiles[tile1]["edges"].intersection(tiles[tile2]["edges"])) >= 1:
                tiles[tile1]["neighbors"].add(tile2)
                tiles[tile2]["neighbors"].add(tile1)


create_tiles("../day20.txt")
process_tiles()


corner_tile = -1
for tile in tiles:
    if len(tiles[tile]["neighbors"]) == 2:
        if corner_tile == -1:
            corner_tile = int(tile)
        else:
            corner_tile *= int(tile)
print("Part 1", corner_tile)