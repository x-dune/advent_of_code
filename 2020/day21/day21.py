from collections import defaultdict, Counter
import itertools
from pathlib import Path


def get_puzzle_input(test_case=0):
    def parse_line(line):
        ingredients, allergens = line.split("(")
        ingredients = ingredients.split()
        allergens = allergens[9:-1].split(", ")
        return ingredients, allergens

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [parse_line(x) for x in f.read().splitlines()]


def main(puzzle_input):
    all_allergens = set()
    all_ings = set()
    for (ings, allergens) in puzzle_input:
        for allergen in allergens:
            all_allergens.add(allergen)
        for ing in ings:
            all_ings.add(ing)

    ingredient_allergen = defaultdict(set)

    for allergen in all_allergens:
        raw_ingredients = [x[0] for x in puzzle_input if allergen in x[1]]

        ingredients = list(itertools.chain.from_iterable(raw_ingredients))
        counted = Counter(ingredients)

        for ing in counted:
            if counted[ing] == len(raw_ingredients):
                ingredient_allergen[ing].add(allergen)

    def remove_allergen(ingredient_allergen, allergen):
        for ing in ingredient_allergen:
            current = ingredient_allergen[ing]
            if len(current) != 1 and allergen in current:
                current.remove(allergen)
                if len(current) == 1:
                    remove_allergen(ingredient_allergen, list(current)[0])

    for ing in ingredient_allergen:
        if len(ingredient_allergen[ing]) == 1:
            remove_allergen(ingredient_allergen, list(ingredient_allergen[ing])[0])

    not_allergen = []
    for ing in all_ings:
        if not ing in ingredient_allergen:
            not_allergen.append(ing)

    count = 0
    for (ings, _) in puzzle_input:
        for ing in ings:
            if ing in not_allergen:
                count += 1

    print("part1", count)
    y = list(ingredient_allergen.items())
    list.sort(y, key=lambda e: list(e[1])[0]) # extract first and only item from set
    print(y)
    print("part2", ",".join([x[0] for x in y]))


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(0)
    main(puzzle_input)
