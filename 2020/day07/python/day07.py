bag_structure = {}

filename = "../day07.txt"
# filename = "../day07.txt"
# Example relationship:
# {
#     "light red": {
#         "children": {
#             "bright white": 1,
#             "muted yellow": 2
#         },
#         "kiddos": [
#             "bright white",
#             "muted yellow",
#             "muted yellow",
#         ]
#         "parents": {}
# }

def ensure_bag_in_structure(bag):
    if bag not in bag_structure:
        bag_structure[bag] = {
            "children": {},
            "kiddos": [],
            "parents": []
        }

with open("../sample.txt", "r") as fh:
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
import sys
sys.setrecursionlimit(5000000)

bag_to_find = "shiny gold"
parents = []
parents_to_add = True
parents.extend(bag_structure[bag_to_find]["parents"])
for parent in parents:
    for parent_parent in bag_structure[parent]["parents"]:
        if parent_parent not in parents:
            parents.append(parent_parent)

def recursive_sum_children(input_list):
    if input_list == []:
        return 0
    else:
        head = input_list[0]
        smaller_list = input_list[1:]
        smaller_list.extend(bag_structure[head]["kiddos"])
        return len(bag_structure[head]["kiddos"]) + recursive_sum_children(smaller_list)


print("Number possible parent bags", len(parents))
print("Total number of required child bags", recursive_sum_children(bag_structure[bag_to_find]["kiddos"]) + len(bag_structure[bag_to_find]["kiddos"]))
