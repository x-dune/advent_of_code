from pathlib import Path
from collections import deque
from time import time


def get_puzzle_input(test_case=0):
    def parse_deck(deck):
        return deque(int(x) for x in deck.split("\n")[1:])

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        decks = f.read().rstrip().split("\n\n")
        return tuple(parse_deck(deck) for deck in decks)


def play_combat(deck1, deck2):
    deck1 = deck1.copy()
    deck2 = deck2.copy()

    while len(deck1) != 0 and len(deck2) != 0:
        card1 = deck1.popleft()
        card2 = deck2.popleft()
        if card1 > card2:
            deck1.append(card1)
            deck1.append(card2)
        else:
            deck2.append(card2)
            deck2.append(card1)
    return (deck1, deck2)


def play_recurse_combat(deck1, deck2):
    deck1 = deck1.copy()
    deck2 = deck2.copy()

    previous_rounds = set()

    while len(deck1) != 0 and len(deck2) != 0:
        round = (tuple(deck1), tuple(deck2))
        if round in previous_rounds:
            return (deck1, [])
        else:
            previous_rounds.add(round)

        card1 = deck1.popleft()
        card2 = deck2.popleft()

        if len(deck1) >= card1 and len(deck2) >= card2:
            recurse_deck1, _recurse_deck2 = play_recurse_combat(
                deque(list(deck1)[:card1]), deque(list(deck2)[:card2])
            )
            if len(recurse_deck1) != 0:
                deck1.append(card1)
                deck1.append(card2)
            else:
                deck2.append(card2)
                deck2.append(card1)
        elif card1 > card2:
            deck1.append(card1)
            deck1.append(card2)
        else:
            deck2.append(card2)
            deck2.append(card1)

    return (deck1, deck2)


def main(decks):
    deck1, deck2 = play_combat(decks[0], decks[1])
    re_deck1, re_deck2 = play_recurse_combat(decks[0], decks[1])

    part1 = 0
    for x in [deck1, deck2]:
        if len(x) != 0:
            for i, y in enumerate(x):
                part1 += y * (len(x) - i)
    part2 = 0
    for x in [re_deck1, re_deck2]:
        if len(x) != 0:
            for i, y in enumerate(x):
                part2 += y * (len(x) - i)

    print("part1", part1)
    print("part2", part2)


if __name__ == "__main__":
    from sys import argv

    starttime = time()
    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    main(puzzle_input)
    print("%s seconds" % (time() - starttime))
