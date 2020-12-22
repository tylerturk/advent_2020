def get_cards(filename="../sample.txt"):
    p1c = []
    p2c = []
    with open(filename, "r") as fh:
        key = None
        player1 = True
        for line in fh.readlines():
            line = line.strip()
            if not line:
                continue
            if "Player" in line:
                if line[-2] == "2":
                    player1 = False
                continue
            if player1:
                p1c.append(int(line))
            else:
                p2c.append(int(line))
    return p1c, p2c

def play_game(p1c, p2c):
    iterations = 0
    while True:
        iterations += 1
        print(p1c, p2c)
        p1 = p1c.pop(0)
        p2 = p2c.pop(0)
        if p1 > p2:
            p1c.extend([p1, p2])
        else:
            p2c.extend([p2, p1])
        if len(p1c) == 0 or len(p2c) == 0:
            break
    return p1c if p1c else p2c

def recursive_round(p1, p2):
    p1n = p1[0]
    p2n = p2[0]
    #if p1n > len(p1) and p2n > len(p2)

def play_recursive_game(cards):
    pass

def calculate_score(cards):
    return sum([x * (len(cards) - cards.index(x)) for x in cards])

p1c, p2c = get_cards("../day22.txt")
finished = play_game(p1c, p2c)
final_score = calculate_score(finished)
print("Part 1", final_score)