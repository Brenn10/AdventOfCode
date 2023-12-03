import re

with open("input.txt", "r") as f:
    sum=0
    for line in f:
        digits = re.findall(r'[0-9]', line)
        sum += int(digits[0])*10 + int(digits[-1])
print(f"Sum: {sum}")