preamble = 25

with open("../day09.txt", "r") as fh:
    content = [int(x.strip()) for x in fh.readlines()]


def sum_exists(numbers, target):
    for number in numbers:
        delta = target - number
        if delta in numbers:
            return True
    return False

index = 0
broken = -1

while index + preamble < len(content):
    if not sum_exists(content[index:index+preamble], content[index+preamble]):
        print("Number",content[index+preamble],"breaks the stride")
        broken = content[index+preamble]
        break
    index += 1

index = 0
print(content)
sum_found = False
for number in content:
    next_index = index + 2
    print(sum(content[index:next_index]))
    while True:
        cur_sum = sum(content[index:next_index])
        if cur_sum > broken:
            break
        if cur_sum == broken:
            print("Indexes from", index, "to", next_index, "add up")
            sub_list = content[index:next_index]
            sub_list.sort()
            print("Sum of the highest and lowest of these two are", sub_list[0] + sub_list[-1])
            sum_found = True
            break
        next_index += 1
    if sum_found:
        break
    index += 1
    