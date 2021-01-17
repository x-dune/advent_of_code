from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return f.read().splitlines()


def get_address_and_value(line):
    address, value = line.split(" = ")
    return (int(address[4:-1]), int(value))


def normalized_binary(integer):
    rawbinary = bin(integer).split("b")[1]
    return rawbinary.rjust(36, "0")


def get_floating_addresses(address):
    floating = [address]
    for i, char in enumerate(address):
        if char == "X":
            next_floating = []
            for x in floating:
                next_floating.append(x[:i] + "0" + x[i + 1 :])
                next_floating.append(x[:i] + "1" + x[i + 1 :])
            floating = next_floating
    return floating


def solution1(lines):
    mask = ""
    memory = {}

    for line in lines:
        if line.startswith("mask"):
            mask = line.split(" = ")[1]
        else:
            address, value = get_address_and_value(line)
            binary_value = normalized_binary(value)
            for i, char in enumerate(mask):
                if char != "X":
                    binary_value = binary_value[:i] + char + binary_value[i + 1 :]
            memory[address] = int(binary_value, 2)
    return sum(memory.values())


def solution2(lines):
    mask = ""
    memory = {}

    for line in lines:
        if line.startswith("mask"):
            mask = line.split(" = ")[1]
        else:
            address, value = get_address_and_value(line)
            binary_address = normalized_binary(address)
            for i, char in enumerate(mask):
                if char != "0":
                    binary_address = binary_address[:i] + char + binary_address[i + 1 :]
            floating = get_floating_addresses(binary_address)
            for address in floating:
                memory[int(address, 2)] = value
    return sum(memory.values())


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
