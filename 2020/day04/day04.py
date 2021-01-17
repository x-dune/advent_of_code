from pathlib import Path
import re


def get_puzzle_input(test_case=0):
    def parseInputLine(line):
        return dict([x.split(":") for x in re.split("\s|\n", line)])

    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [parseInputLine(x) for x in f.read().rstrip().split("\n\n")]


def solution1(lines):
    count = 0
    for line in lines:
        if (
            "byr" in line
            and "iyr" in line
            and "eyr" in line
            and "hgt" in line
            and "hcl" in line
            and "ecl" in line
            and "pid" in line
        ):
            count = count + 1
    return count


def solution2(lines):
    count = 0
    for line in lines:
        try:
            (height, height_unit) = re.match("(^\d{2,3})(cm|in)$", line["hgt"]).groups()

            if (
                (int(line["byr"]) >= 1920 and int(line["byr"]) <= 2002)
                and (int(line["iyr"]) >= 2010 and int(line["iyr"]) <= 2020)
                and (int(line["eyr"]) >= 2020 and int(line["eyr"]) <= 2030)
                and (
                    (height_unit == "cm" and int(height) >= 150 and int(height) <= 193)
                    or (height_unit == "in" and int(height) >= 59 and int(height) <= 76)
                )
                and bool(re.match("^#[a-f0-9]{6}$", line["hcl"]))
                and line["ecl"] in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                and bool(re.match("^\d{9}$", line["pid"]))
            ):
                count = count + 1
        except:
            pass
    return count


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
