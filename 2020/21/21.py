from collections import defaultdict, Counter
from pathlib import Path
from itertools import chain


def get_puzzle_input(test_case=0):
    def parse_line(line):
        ingredients, allergens = line.split("(")
        ingredients = ingredients.split()
        allergens = allergens[9:-1].split(", ")
        return ingredients, allergens

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [parse_line(x) for x in f.read().splitlines()]


def identify_allergens(puzzle_input):
    all_allergens = set(chain(*[allergens for (_, allergens) in puzzle_input]))

    allergen_to_ingredient = defaultdict(set)
    # get potential ingredients for each allergen
    for allergen in all_allergens:
        food_containing_allergen = [
            ingredients
            for (ingredients, allergens) in puzzle_input
            if allergen in allergens
        ]

        counted = Counter(chain(*food_containing_allergen))

        for ingredient in counted:
            if counted[ingredient] == len(food_containing_allergen):
                allergen_to_ingredient[allergen].add(ingredient)

    def recurse_remove(ingredient, allergen_to_ingredient):
        for allergen in allergen_to_ingredient:
            ingredients = allergen_to_ingredient[allergen]
            if len(ingredients) != 1 and ingredient in ingredients:
                ingredients.remove(ingredient)
                if len(ingredients) == 1:
                    last_ingredient = list(ingredients)[0]
                    recurse_remove(last_ingredient, allergen_to_ingredient)

    # if ingredient is the only potential for an allergen then
    # recursively remove from other allergen's potential ingredients
    for allergen in allergen_to_ingredient:
        ingredients = allergen_to_ingredient[allergen]
        if len(ingredients) == 1:
            recurse_remove(list(ingredients)[0], allergen_to_ingredient)

    # clean data
    # extract only ingredient from set of potential ingredient
    for allergen in allergen_to_ingredient:
        ingredients = allergen_to_ingredient[allergen]
        if len(ingredients) == 1:
            allergen_to_ingredient[allergen] = list(ingredients)[0]
        else:
            # something has gone wrong
            # if there is a set of potential ingredients with more than 1 ingredient
            raise ValueError

    return allergen_to_ingredient


def solution1(puzzle_input, allergen_identity):
    count = 0
    for (ingredients, _) in puzzle_input:
        for ingredient in ingredients:
            if ingredient not in allergen_identity.values():
                count += 1
    return count


def solution2(allergen_identity):
    return ",".join(
        [ingredient for (_, ingredient) in sorted(allergen_identity.items())]
    )


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )

    allergen_identity = identify_allergens(puzzle_input)
    print(solution1(puzzle_input, allergen_identity))
    print(solution2(allergen_identity))
