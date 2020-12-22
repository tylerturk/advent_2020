all_allergens = {}
all_words = dict()

with open("../day21.txt", "r") as fh:
    content = fh.readlines()
    for line in content:
        line = line.strip()
        menu_item, allergens_str = line.split(" (contains ")
        allergens = [x.replace(")", "") for x in allergens_str.split(", ")]
        print("Menu Item:",menu_item,"Allergens:", allergens)
        for allergen in allergens:
            if allergen not in all_allergens:
                all_allergens[allergen] = set()
                for word in menu_item.split():
                    all_allergens[allergen].add(word)
            else:
                all_allergens[allergen] = all_allergens[allergen].intersection(set(menu_item.split()))
        for word in menu_item.split():
            if word not in all_words:
                all_words[word] = 1
            else:
                all_words[word] += 1
            

associated = []
should_break = True
while len(associated) != len(all_allergens.keys()):
    should_break = True
    print(all_allergens)
    for allergen in all_allergens.keys():
        if len(all_allergens[allergen]) == 1 and all_allergens[allergen] not in associated:
            associated.append(list(all_allergens[allergen])[0])
            continue
        for word in associated:
            #print("Managing", word)
            if word in all_allergens[allergen] and len(all_allergens[allergen]) > 1:
                all_allergens[allergen].remove(word)
        if len(all_allergens[allergen]) > 1:
            should_break = False
    if should_break:
        break
total = 0
print(all_allergens)
discovered = [list(all_allergens[allergen])[0] for allergen in all_allergens.keys()]
print(discovered)
for word in all_words.keys():
    if word not in discovered:
        print(word, all_words[word])
        total += all_words[word]
print("Part 1", total)
print("Part 2", ",".join([str(list(all_allergens[x])[0]) for x in sorted(all_allergens.keys())]))