with open("../sample.txt", "r") as fh:
    content = fh.readlines()

content = '((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2'

operators = ["+", "-", "*", "/"]

def resolve_pieces(pieces):
    cumulative = None
    modifier = None
    ind = 0
    ind_delta = -1
    for piece in pieces:
        if ind_delta + 1 > 0:
            ind += 1
            ind_delta -= 1
            continue
        if cumulative is None and piece != "(":
            cumulative = int(piece)
        else:
            if piece == "(":
                resp, ind_delta = resolve_pieces(pieces[ind+1:])
                if cumulative is None:
                    cumulative = resp
                else:
                    piece = resp
            elif piece == ")":
                return cumulative, ind
            elif piece in operators:
                modifier = piece
            if isinstance(piece, int) or piece.isnumeric():
                piece = int(piece)
                if modifier == "+":
                    cumulative += piece
                elif modifier == "-":
                    cumulative -= piece
                elif modifier == "*":
                    cumulative *= piece
                elif modifier == "/":
                    cumulative /= piece
                else:
                    print("Unsupported modifier", modifier)
        ind += 1
    return cumulative

content = content.replace("(", "( ")
content = content.replace(")", " )")
components = content.split()
with open("../day18.txt", "r") as fh:
    total = 0
    for content in fh.readlines():
        content = content.replace("(", "( ")
        content = content.replace(")", " )")
        components = content.split()
        total += resolve_pieces(components)

print("Part 1", total)