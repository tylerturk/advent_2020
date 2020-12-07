
total_rows = 127
row_modifier = 8

found_seats = []
highest = 0
rows = {}
cols = {}

with open("../day05.txt", 'r') as fh:

    for line in fh.readlines():
        lower_bound = 0
        left_bound = 0
        right_bound = 7
        higher_bound = total_rows
        # letters = line.split("")
        for letter in line:
            row_delta = (higher_bound - lower_bound + 1) / 2
            col_delta = (right_bound - left_bound + 1) / 2
            if letter == 'F':
                higher_bound -= row_delta
            elif letter == 'B':
                lower_bound += row_delta
            elif letter == 'L':
                right_bound -= col_delta
            elif letter == 'R':
                left_bound += col_delta
        print("Seat is", higher_bound, lower_bound, right_bound, left_bound)
        seat_number = higher_bound * row_modifier + left_bound
        found_seats.append([higher_bound, left_bound, seat_number])
        if higher_bound not in rows.keys():
            rows[higher_bound] = 1
        else:
            rows[higher_bound] += 1
        if left_bound not in cols.keys():
            cols[left_bound] = 1
        else:
            cols[left_bound] += 1
        if seat_number > highest:
            highest = seat_number
print(highest)

import json

found_seats.sort(key=lambda x: x[1])
found_seats.sort(key=lambda x: x[0])
print([x for x in found_seats if x[0] == 76])
# print(json.dumps(rows, indent=2))
