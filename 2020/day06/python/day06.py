
with open("../day06.txt", "r") as fh:
    content = fh.read()


groups = content.split("\n\n")

totes = 0
for group in groups:
    votes = {}
    expected = len(group.split())
    # print("Group",len(group.split()), group)
    for letter in group:
        if letter == '\n':
            continue
        if letter not in votes:
            votes[letter] = 1
        else:
            votes[letter] += 1
    for letter in votes.keys():
        if votes.get(letter) == expected:
            totes += 1
    # print(votes)
    # totes += len(votes)
# print(votes)

#for group in votes:
#    totes += len(group)
#print(len(votes))
print(totes)