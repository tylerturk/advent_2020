def solve_v1(content):
    mem_values = {}
    cur_mask = ''
    for line in content:
        if line.startswith("mask"):
            cur_mask = line[-37:]
            continue
        mem_slot = line.split()[0][4:-1]
        mem_value_pre_mask = int(line.split()[-1])
        bin_value = [x for x in bin(mem_value_pre_mask)[2:].zfill(36)]
        for ind in range(0, len(cur_mask)):
            if cur_mask[ind] == '0':
                bin_value[ind] = '0'
            elif cur_mask[ind] == '1':
                bin_value[ind] = '1'
        bin_value = ''.join(bin_value)
        mem_values[mem_slot] = bin_value

    total = 0
    for key in mem_values:
        total += int(mem_values[key], 2)

    print("V1:", total)


def recursively_find(val, resolved_slots):
    if 'X' not in val:
        resolved_slots.append(int(val, 2))
        return resolved_slots
    ind_replace = val.index('X')
    zero_val = "{}{}{}".format(val[0:ind_replace], 0, val[ind_replace+1:])
    one_val = "{}{}{}".format(val[0:ind_replace], 1, val[ind_replace+1:])
    recursively_find(zero_val, resolved_slots)
    recursively_find(one_val, resolved_slots)
    cached_values[val] = resolved_slots
    return resolved_slots


def solve_v2(content):
    mem_values = {}
    cur_mask = ''
    for line in content:
        if line.startswith("mask"):
            cur_mask = line[-37:]
            continue
        
        mem_slot_pre_mask = [x for x in bin(int(line.split()[0][4:-1]))[2:].zfill(36)]
        value = line.split()[-1]
        for ind in range(0, len(cur_mask)):
            if cur_mask[ind] == 'X':
                mem_slot_pre_mask[ind] = 'X'
            elif cur_mask[ind] == '1':
                mem_slot_pre_mask[ind] = '1'
        mem_slot_masked = ''.join(mem_slot_pre_mask)
        all_mem_slots = set(recursively_find(mem_slot_masked, []))
        
        for mem_value in set(all_mem_slots):
            mem_values[mem_value] = value

    total = 0
    for key in mem_values:
        total += int(mem_values[key])
    print("V2:", total)


def process_content():
    with open("../day14.txt", "r") as fh:
        content = fh.readlines()
    solve_v1(content)
    solve_v2(content) 

process_content()