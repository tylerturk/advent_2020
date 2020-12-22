def get_cards(filename="../sample.txt"):
    cards = {}
    with open(filename, "r") as fh:
        key = None
        player1 = True
        for line in fh.readlines():
            line = line.strip()
            if not line:
                continue
            if "Player" in line:
                print(line)
                key = "player{}".format(line.split()[1][:-1])
                cards[key] = []
                if line[-2] == "2":
                    player1 = False
                continue
            cards[key].append(int(line))
    return cards

def play_game(cards):
    iterations = 0
    while True:
        iterations += 1
        print(cards)
        p1 = cards["player1"].pop(0)
        p2 = cards["player2"].pop(0)
        if p1 > p2:
            cards["player1"].extend([p1, p2])
        else:
            cards["player2"].extend([p2, p1])
        if len(cards["player1"]) == 0 or len(cards["player2"]) == 0:
            break
    return cards

def recursive_round(p1, p2):
    p1n = p1[0]
    p2n = p2[0]
    #if p1n > len(p1) and p2n > len(p2)

def play_recursive_game(cards):
    pass

def calculate_score(cards):
    player = [player for player in cards.keys() if cards[player]][0]
    return sum([x * (len(cards[player]) - cards[player].index(x)) for x in cards[player]])

cards = get_cards("../day22.txt")
finished = play_game(cards)
final_score = calculate_score(finished)
print("Part 1", final_score)