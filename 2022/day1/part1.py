with open("day1/input.txt", "r") as f:
    increases = 0
    last = int(f.readline())
    for line in f:
        current = int(line)
        if current > last:
            increases += 1
        last = current
    print(increases)