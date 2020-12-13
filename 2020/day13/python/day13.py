with open("../day13.txt", "r") as fh:
    content = fh.readlines()

earliest_time = int(content[0])
active_buses = {int(x): 0 for x in content[1].split(",") if x != 'x'}



def part_one():
    first_available_time = -1
    earliest_bus_id = -1
    for bus_id in active_buses.keys():
        num_before = int(earliest_time / bus_id)
        if num_before * bus_id < earliest_time:
            num_before += 1
        # Evaluate first departure time after your earliest time
        active_buses[bus_id] = num_before * bus_id
        if first_available_time < 0:
            first_available_time = active_buses[bus_id]
            earliest_bus_id = bus_id
        elif active_buses[bus_id] < first_available_time:
            first_available_time = active_buses[bus_id]
            earliest_bus_id = bus_id

    print((first_available_time - earliest_time) * earliest_bus_id)

part_one()