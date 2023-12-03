import re

def parse_set(set): # (?=hello)
    try:
        red = int(re.search(r'[0-9]+(?= red)', set).group(0))
    except:
        red = 0
    try:
        green = int(re.search(r'[0-9]+(?= green)', set).group(0))
    except:
        green = 0
    try:
        blue = int(re.search(r'[0-9]+(?= blue)', set).group(0))
    except:
        blue = 0
    return {
        "red": red,
        "green": green,
        "blue": blue
    }

def parse_game(line):
    game_info = {
        "id": 0,
        "sets": []
    }
    colon_split = line.split(":")
    game_info["id"] = int(re.search(r'[0-9]+', colon_split[0]).group(0))
    sets = colon_split[1].split(";")
    for set in sets:
        game_info["sets"].append(parse_set(set))
    return game_info
    
def get_cube_power(game):
    bag = {
        "red": 0,
        "green": 0,
        "blue": 0
    }
    for set in game["sets"]:
        for color in ("red", "green", "blue"):
            if(set[color] > bag[color]):
                bag[color] = set[color]
    return bag["red"]*bag["green"]*bag["blue"]

with open("input.txt") as f:
    sum = 0
    for line in f:
        game = parse_game(line)
        sum+=get_cube_power(game)
    print(sum)