import re
capture_groups=r'[0-9]|one|two|three|four|five|six|seven|eight|nine'

spell_to_int = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

for i in range(1, 10):
    spell_to_int.update({str(i): i})
    
def parse_groups(line):
    result = []
    while len(line):
        match = re.search(capture_groups, line)
        if match:
            result.append(match.group())
            line = line[match.start() + 1:]
        else:
            break
    return result

with open("input.txt", "r") as f:
    sum=0
    for line in f:
        digits = parse_groups(line)
        sum += spell_to_int[digits[0]]*10 + spell_to_int[digits[-1]]
    print(f"Sum: {sum}")