import re

FIELDS = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
]

filename = "../day04.txt"
valid = 0
with open(filename, "r") as fh:
    passports = fh.read().split('\n\n')

def valid_passport(p):
    found_fields = set([x for x in p.keys() if x in FIELDS])
    if len(found_fields) != len(FIELDS):
        return False
    byr = int(p["byr"])
    if byr < 1920 or byr > 2002:
        print(byr,"failed validation on byr")
        return False
    iyr = int(p["iyr"])
    if iyr < 2010 or iyr > 2020:
        print(iyr,"failed validation on iyr")
        return False
    eyr = int(p["eyr"])
    if eyr < 2020 or eyr > 2030:
        print(eyr,"failed validation on eyr")
        return False
    hgt = p["hgt"]
    if hgt[-2:] == "cm":
        size = int(hgt[:-2])
        if size < 150 or size > 193:
            print(hgt,"failed on height")
            return False
    elif hgt[-2:] == "in":
        size = int(hgt[:-2])
        if size < 59 or size > 76:
            print(hgt,"failed on height")
            return False
    else:
        return False
    hcl = re.compile("^#[0-9a-f]{6}$")
    if not re.match(hcl, p["hcl"]):
        print(p["hcl"],"failed on hcl")
        return False
    if p["ecl"] not in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
        print(p["ecl"],"failed on ecl")
        return False
    pid = re.compile("^[0-9]{9}$")
    if not re.match(pid, p["pid"]):
        print(p["pid"],"failed on pid")
        return False
    return True

for passport in passports:
    cleaned = {x.split(":")[0]: x.split(":")[1] for x in passport.split()}
    if valid_passport(cleaned):
        valid += 1
print(valid)