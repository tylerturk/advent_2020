bag_structure = {}

def ensure_bag_in_structure(bag):
    if bag not in bag_structure:
        bag_structure[bag] = {
            "name": bag,
            "children": {},
            "kiddos": [],
            "parents": []
        }

with open("../day07.txt", "r") as fh:
    # Determine bag structuring
    for line in fh.readlines():
        line = line.replace("bags ", "").replace("bag ", "").replace(".", "").strip()
        parent_bag, child_bags = line.split(" contain ")
        ensure_bag_in_structure(parent_bag)
        if child_bags == "no other bags":
            continue
        child_bags = child_bags.split(", ")
        for bag in child_bags:
            count = bag.split()[0]
            bag_type = " ".join(bag.split()[1:-1])
            bag_structure[parent_bag]["children"][bag_type] = int(count)
            bag_structure[parent_bag]["kiddos"].extend([bag_type]*int(count))
            ensure_bag_in_structure(bag_type)
            bag_structure[bag_type]["parents"].append(parent_bag)

bag_to_find = "shiny gold"
parents = []
parents_to_add = True
parents.extend(bag_structure[bag_to_find]["parents"])
for parent in parents:
    for parent_parent in bag_structure[parent]["parents"]:
        if parent_parent not in parents:
            parents.append(parent_parent)

def recursive_create(bag):
    local_bag = {}
    for bag_keys in bag.keys():
        for child in bag_structure[bag_keys]["children"].keys():
            bag[bag_keys][child] = {}
            recursive_create(bag[bag_keys])
    return bag

def count_bags(bag_obj, parent):
    keys = bag_obj.keys()
    local = 0
    for key in keys:
        local += (1 + count_bags(bag_obj[key], key)) * bag_structure[parent]["children"][key]
    return local

bag_obj = recursive_create({bag_to_find: {}})

print("Number possible parent bags", len(parents))
print("Total number of required child bags", count_bags(bag_obj[bag_to_find], bag_to_find))
