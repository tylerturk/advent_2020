content = None

with open("../day08.txt", "r") as fh:
    content = fh.readlines()

lines_hit = set()
all_jmp_indexes = []
all_nop_indexes = []
index = 0

prev_index = -1

indexes_to_try_to_change = []
last_index_changed = -1
import time
while True:
    accumulator = 0
    success = True
    index = 0
    lines_hit.clear()
    while True:
        if index in lines_hit:
            success = False
            break
        lines_hit.add(index)
        if len(content) == index:
            break
        instruction = content[index]
        prev_index = index
        if instruction.startswith("nop"):
            if index not in indexes_to_try_to_change:
                indexes_to_try_to_change.append(index)
            index += 1
            continue
        if instruction.startswith("jmp"):
            if index not in indexes_to_try_to_change:
                indexes_to_try_to_change.append(index)
            index += int(instruction.split()[1])
            print(instruction)
            continue
        if instruction.startswith("acc"):
            accumulator += int(instruction.split()[1])
            index += 1
            continue
    if success:
        break
    print(last_index_changed)
    if last_index_changed != -1:
        if content[indexes_to_try_to_change[last_index_changed]].startswith("nop"):
            content[indexes_to_try_to_change[last_index_changed]] = content[indexes_to_try_to_change[last_index_changed]].replace("nop", "jmp")
        else:
            content[indexes_to_try_to_change[last_index_changed]] = content[indexes_to_try_to_change[last_index_changed]].replace("jmp", "nop")
    last_index_changed += 1
    if content[indexes_to_try_to_change[last_index_changed]].startswith("nop"):
            content[indexes_to_try_to_change[last_index_changed]] = content[indexes_to_try_to_change[last_index_changed]].replace("nop", "jmp")
    else:
        content[indexes_to_try_to_change[last_index_changed]] = content[indexes_to_try_to_change[last_index_changed]].replace("jmp", "nop")
    

print("Accumulator is", accumulator)
print(lines_hit)
if success:
    print("Made it through")
else:
    print("Broken")
    print("Index",prev_index,"caused the infinite loop")