rules = {}
my_ticket = []

nearby_tickets = []

valid_tickets = []

invalid_numbers = []
rule_possibilities = []

with open("../day16.txt", "r") as fh:
    content = fh.readlines()
    section = 'rules'
    for line in content:
        line = line.strip()
        if not line:
            continue
        if line.startswith('your'):
            section = 'yours'
            rule_possibilities = [list(rules.keys())] * len(rules.keys())
            continue
        elif line.startswith('nearby'):
            section = 'nearby'
            continue
        if section == 'rules':
            bits = line.split(":")
            identifier = bits[0]
            rule_bits = bits[1].split()
            first_bound = rule_bits[0]
            second_bound = rule_bits[-1]
            rules[identifier] = {
                "first": [int(first_bound.split('-')[0]), int(first_bound.split('-')[1])],
                "second": [int(second_bound.split('-')[0]), int(second_bound.split('-')[1])]
            }
        if section == 'nearby' or section == 'yours':
            ind = 0
            for num in line.split(","):
                valid_for_rule = False
                invalid_for = []
                valid_for_rules = list()
                num = int(num)
                if section == 'yours':
                    my_ticket.append(num)
                
                for rule in rules:
                    if rules[rule]["first"][0] <= num <= rules[rule]["first"][1] or rules[rule]["second"][0] <= num <= rules[rule]["second"][1]:
                        valid_for_rules.append(rule)
                        valid_for_rule = True
                    else:
                        invalid_for.append(rule)
                
                if not valid_for_rule:
                    invalid_numbers.append(int(num))
                else:
                    rule_possibilities[ind] = list(set(rule_possibilities[ind]) - set(invalid_for))
                ind += 1

print("Part 1", sum(invalid_numbers))

rules_not_determined = True
determined_rules = []
while len(determined_rules) != len(rules.keys()):
    for rule_possibility in rule_possibilities:
        if len(rule_possibility) == 1 and rule_possibility[0] not in determined_rules:
            determined_rules.append(rule_possibility[0])
            continue
        for rule in determined_rules:
            if rule in rule_possibility and len(rule_possibility) > 1:
                rule_possibility.remove(rule)

multiple = 1
ind = 0
for rule in rule_possibilities:
    if rule[0].startswith('departure'):
        multiple *= my_ticket[ind]
    ind += 1

print("Part 2", multiple)