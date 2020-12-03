def main():
    valid_passwords = 0
    with open('../day02.txt', 'r') as fh:
        for line in fh.readlines():
            if within_constraints_count(line):
                valid_passwords += 1
    print(valid_passwords)

def within_constraints_count(unsplit_line):
    unsplit_line = unsplit_line.strip()
    min_max = unsplit_line.split(" ")[0]
    min_count = int(min_max.split("-")[0])
    max_count = int(min_max.split("-")[1])
    letter = unsplit_line.split(" ")[1].replace(":", "")
    password = unsplit_line.split(":")[-1].strip()
    if password.count(letter) >= min_count and password.count(letter) <= max_count:
        return True
    return False

def within_constraints_pos(unsplit_line):
    unsplit_line = unsplit_line.strip()
    pos = unsplit_line.split(" ")[0]
    first = int(pos.split("-")[0]) - 1
    second = int(pos.split("-")[1]) - 1
    letter = unsplit_line.split(" ")[1].replace(":", "")
    password = unsplit_line.split(":")[-1].strip()
    first_letter = password[first]
    second_letter = password[second]
    if first_letter != second_letter and letter in [first_letter, second_letter]:
        return True
    return False

if __name__ == '__main__':
    main()