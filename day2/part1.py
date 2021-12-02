with open("day2/input.txt") as f:
    x = 0
    y = 0
    for line in f:
        line = line.split(" ")
        if line[0][0] == "u":
            y -= int(line[1])
        elif line[0][0] == "d":
            y += int(line[1])
        elif line[0][0] == "f":
            x += int(line[1])
        else:
            print("Error: ")
            print(line)
    print(x*y)