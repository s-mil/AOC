import unittest
import Day1


class TestDay1(unittest.TestCase):
    items = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]

    def test_part1(self):
        items = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]

        self.assertEqual(Day1.Day1.part1(items), 7)

    def test_part2(self):
        items = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        self.assertEqual(Day1.Day1.part2(items), 5)


if __name__ == "__main__":
    unittest.main()
