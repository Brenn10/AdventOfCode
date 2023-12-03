from run import *
def parse_set_test():
    line = "20 green, 3 red, 2 blue"
    set = parse_set(line)
    assert set["red"] == 3
    assert set["green"] == 20
    assert set["blue"] == 2
parse_set_test()

def parse_game_test():
    line = "Game 1: 20 green, 3 red, 2 blue; 9 red, 16 blue, 18 green; 6 blue, 19 red, 10 green; 12 red, 19 green, 11 blue"
    game = parse_game(line)
    assert game["id"] == 1
    assert game["sets"][3] == {"red": 12, "green": 19, "blue": 11}
parse_game_test()

