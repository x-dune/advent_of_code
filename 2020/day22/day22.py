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


def play_combat(deck1, deck2, part2=False):
    previous_rounds = set()
    while deck1 and deck2:
        if part2:
            current_round = (tuple(deck1), tuple(deck2))
            if current_round in previous_rounds:
                return (deck1, [])
            else:
                previous_rounds.add(current_round)

        card1 = deck1.popleft()
        card2 = deck2.popleft()

        if part2 and len(deck1) >= card1 and len(deck2) >= card2:
            recurse_deck1, _recurse_deck2 = play_combat(
                deque(list(deck1)[:card1]),
                deque(list(deck2)[:card2]),
                part2,
            )
            if len(recurse_deck1) != 0:
                deck1.extend([card1, card2])
            else:
                deck2.extend([card2, card1])

        elif card1 > card2:
            deck1.extend([card1, card2])
        else:
            deck2.extend([card2, card1])

    return (deck1, deck2)


def solution1(deck1, deck2):
    deck1, deck2 = play_combat(deck1.copy(), deck2.copy())
    # one of the deck is empty, so this doesn't matter
    combined = deck1 + deck2

    return sum([(len(combined) - i) * x for i, x in enumerate(combined)])


def solution2(deck1, deck2):
    deck1, deck2 = play_combat(deck1.copy(), deck2.copy(), part2=True)
    combined = deck1 + deck2

    return sum([(len(combined) - i) * x for i, x in enumerate(combined)])


if __name__ == "__main__":
    from sys import argv

    start_time = time()
    deck1, deck2 = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(deck1, deck2))
    print(solution2(deck1, deck2))
    print(f"Time: {'{0:.2f}'.format(time() - start_time)} seconds")
