import math
with open("../input.txt", "r") as fh:
    content = [int(x) for x in fh.readlines()]

content.sort()

diff_1 = []
diff_3 = []
last = 0

step_counts = {
    1: 0,
    2: 0,
    3: 0
}

steps_to_number = {}

for num in content:
    steps = 0
    if num - last == 1:
        diff_1.append(num)
    elif num - last == 3:
        diff_3.append(num)

end = content[-1] + 3


cache = {}
def walk_path(cur_num):
    total = 0
    if cur_num + 3 == end:
        return 1
    if cur_num in cache:
        return cache[cur_num]
    if cur_num + 1 in content:
        total += walk_path(cur_num+1)
    if cur_num + 2 in content:
        total += walk_path(cur_num+2)
    if cur_num + 3 in content:
        total += walk_path(cur_num+3)
    cache[cur_num] = total
    return total


print(len(diff_1) * (len(diff_3) + 1))
print(walk_path(0))