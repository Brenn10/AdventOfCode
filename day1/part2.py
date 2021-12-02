with open("day1/input.txt", "r") as f:
    increases = 0
    A = int(f.readline())
    B = int(f.readline())
    C = int(f.readline())
    for line in f:
        current = int(line)
        if current > A:
            increases += 1
        last = current
        A=B
        B=C
        C = current
    print(increases)