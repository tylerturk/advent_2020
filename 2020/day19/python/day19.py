with open("../day19.txt", "r") as fh:
    content = fh.readlines()

rules = {}

def add_to_rules(line):
    global rules
    rule_number, rule_line = line.split(": ")
    if rule_line.startswith('"'):
        rules[rule_number] = rule_line.replace('"', '')
    else:
        rule = [r.split() for r in [rc for rc in rule_line.split("|")]]
        rules[rule_number] = rule

def ingest_rules():
    matching = 0
    in_rules = True
    possibilities = set()
    for line in content:
        line = line.strip()
        if not line:
            # determine_valid_paths()
            in_rules = False
            possibilities = resolve_rule("0")
        if in_rules:
            add_to_rules(line)
        else:
            if line in possibilities:
                matching += 1
    return matching
valid_paths = []
# def determine_valid_paths(path=list(), cur_options="0"):
#     global rules
#     if not isinstance(cur_options, list) and cur_options != "0":
#         valid_paths.append(path)
#         return
#     path.extend(cur_options)
#     print(path)
#     options = rules[path[-1]]
#     for option in options:
#         determine_valid_paths(path, cur_options)



rule_cache = {}

#def determine_rule_meaning
rule_cache = {}
def resolve_rule(num, cur_str=""):
    cur_strings = []
    result = rules[num]
    if num in rule_cache:
        print("Used cache")
        return rule_cache[num]
    if isinstance(result, str) and result.isalpha():
        return result
    for comborule in result:
        possibilities = [cur_str]
        for rule in comborule:
            res = resolve_rule(rule, cur_str)
            if not isinstance(res, list):
                possibilities = ["{}{}".format(cs, res) for cs in possibilities]
            else:
                tmp = set()
                print("Evaluating",len(res),"permutations")
                for r in res:
                   for possibility in possibilities:
                       tmp.add("{}{}".format(possibility, r))
                
                possibilities = list(tmp)

        cur_strings.extend(possibilities)
    rule_cache[num] = cur_strings
    return cur_strings


print("Part 1", ingest_rules())
