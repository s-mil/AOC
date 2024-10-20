class Day1:
    def __init__(self):
        pass

    def part1(items: list[int]) -> int:
        previous_item = items[0]
        increasing_item_count = 0
        for item in items[1:]:
            if item > previous_item:
                increasing_item_count += 1

            previous_item = item

        return increasing_item_count

    def part2(items: list[int]) -> int:
        previous_item = items[0] + items[1] + items[2]
        increasing_item_count = 0
        
        return increasing_item_count


def load_data(filename):
    result = []
    with open(filename) as f:
        for line in f:
            result.append(int(line))
    return result


def main():
    print("Hello")
    part1Result = Day1.part1(load_data("input.txt"))
    print(part1Result)


if __name__ == "__main__":
    main()
