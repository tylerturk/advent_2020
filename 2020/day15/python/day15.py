with open("../day15.txt", "r") as fh:
    content = fh.readlines()

numbers = [int(x) for x in content[0].split(",")]
numbers = [None] * 30000000
numbers_index = {x: [numbers.index(x) + 1] for x in numbers}
counted = 3
last_number = numbers[-1]
while len(numbers) < 30000000:
    if len(numbers_index[last_number]) == 1:
        new_num = 0
    else:
        new_num = numbers_index[last_number][-1] - numbers_index[last_number][-2]
    numbers.append(new_num)
    if new_num not in numbers_index:
        numbers_index[new_num] = list()
    numbers_index[new_num].append(len(numbers))
    last_number = numbers[-1]

print(numbers[-1])