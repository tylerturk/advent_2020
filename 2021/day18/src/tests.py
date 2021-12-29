from day18 import *
import unittest
import itertools

class TestSnailFish(unittest.TestCase):
    def test_explode_1(self):
        explode = SnailFishNumber.from_list("[[[[[9,8],1],2],3],4]")
        explode.explode()
        expected = SnailFishNumber.from_list("[[[[0,9],2],3],4]")
        self.assertEqual(explode, expected)

    def test_explode_2(self):
        explode = SnailFishNumber.from_list("[7,[6,[5,[4,[3,2]]]]]")
        explode.explode()
        expected = SnailFishNumber.from_list("[7,[6,[5,[7,0]]]]")
        self.assertEqual(explode, expected)

    def test_explode_3(self):
        explode = SnailFishNumber.from_list("[[6,[5,[4,[3,2]]]],1]")
        explode.explode()
        expected = SnailFishNumber.from_list("[[6,[5,[7,0]]],3]")
        self.assertEqual(explode, expected)

    def test_explode_4(self):
        explode = SnailFishNumber.from_list("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
        explode.explode()
        expected = SnailFishNumber.from_list("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        self.assertEqual(explode, expected)

    def test_explode_5(self):
        explode = SnailFishNumber.from_list("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        explode.explode()
        expected = SnailFishNumber.from_list("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        self.assertEqual(explode, expected)

    def test_split_1(self):
        to_split = SnailFishNumber.from_list("[1, [13, [5, 3]]]")
        to_split.split()
        expected = SnailFishNumber.from_list("[1, [[6, 7], [5, 3]]]")
        self.assertEqual(to_split, expected)

    def test_split_2(self):
        to_split = SnailFishNumber.from_list("[[[[0, 7], 4], [15, [0, 13]]], [1, 1]]")
        to_split.split()
        expected = SnailFishNumber.from_list("[[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]]")
        self.assertEqual(to_split, expected)

    def test_solve(self):
        sf1 = SnailFishNumber.from_list("[[[[4,3],4],4],[7,[[8,4],9]]]")
        sf2 = SnailFishNumber.from_list("[1,1]")
        sf = SnailFishNumber(sf1, sf2)
        sf.set_depths()
        sf.set_parents()
        sf.reduce()
        expected = SnailFishNumber.from_list("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        self.assertEqual(sf, expected)

    def test_solve_2(self):
        initial = SnailFishNumber.from_list("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
        to_add = SnailFishNumber.from_list("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")
        expected = SnailFishNumber.from_list("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        sf = SnailFishNumber(initial, to_add)
        sf.reduce()
        self.assertEqual(sf, expected)

    def test_solve_3(self):
        initial = SnailFishNumber.from_list("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")
        to_add = SnailFishNumber.from_list("[7,[5,[[3,8],[1,4]]]]")
        expected = SnailFishNumber.from_list("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]")
        sf = SnailFishNumber(initial, to_add)
        sf.reduce()
        self.assertEqual(sf, expected)

    def test_addition_1(self):
        sf1 = SnailFishNumber.from_list("[1,1]")
        sf2 = SnailFishNumber.from_list("[2,2]")
        sf3 = SnailFishNumber.from_list("[3,3]")
        sf4 = SnailFishNumber.from_list("[4,4]")
        first = SnailFishNumber(sf1, sf2)
        first.reduce()
        second = SnailFishNumber(first, sf3)
        second.reduce()
        final = SnailFishNumber(second, sf4)
        final.reduce()
        expected = SnailFishNumber.from_list("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        self.assertEqual(final, expected)

    def test_addition_2(self):
        sf1 = SnailFishNumber.from_list("[1,1]")
        sf2 = SnailFishNumber.from_list("[2,2]")
        sf3 = SnailFishNumber.from_list("[3,3]")
        sf4 = SnailFishNumber.from_list("[4,4]")
        sf5 = SnailFishNumber.from_list("[5,5]")
        first = SnailFishNumber(sf1, sf2)
        first.reduce()
        second = SnailFishNumber(first, sf3)
        second.reduce()
        third = SnailFishNumber(second, sf4)
        third.reduce()
        final = SnailFishNumber(third, sf5)
        final.reduce()
        expected = SnailFishNumber.from_list("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        self.assertEqual(final, expected)

    def test_complex(self):
        sf = SnailFishNumber.from_list("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
        entries = [
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]"
        ]
        for num in entries:
            to_add = SnailFishNumber.from_list(num)
            sf = SnailFishNumber(sf, to_add)
            sf.reduce()
        output = SnailFishNumber.from_list("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        self.assertEqual(sf, output)

    def test_magnitude_1(self):
        sf = SnailFishNumber.from_list("[9,1]")
        self.assertEqual(sf.determine_magnitude(), 29)

    def test_magnitude_2(self):
        sf = SnailFishNumber.from_list("[[9,1],[1,9]]")
        self.assertEqual(sf.determine_magnitude(), 129)

    def test_magnitude_3(self):
        sf = SnailFishNumber.from_list("[[1,2],[[3,4],5]]")
        self.assertEqual(sf.determine_magnitude(), 143)

    def test_magnitude_4(self):
        sf1 = SnailFishNumber.from_list("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]")
        sf2 = SnailFishNumber.from_list("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]")
        sf3 = SnailFishNumber(sf1, sf2)
        sf3.reduce()
        self.assertEqual(sf3.determine_magnitude(), 3993)

    def test_magnitude_5(self):
        sf1 = SnailFishNumber.from_list("[[[7,9],[7,8]],[[8,3],8]]")
        sf2 = SnailFishNumber.from_list("[[[7,7],[7,7]],[[6,6],[6,6]]]")
        sf3 = SnailFishNumber(sf1, sf2)
        sf3.reduce()
        sf3.determine_magnitude()

        self.assertEqual(sf3.determine_magnitude(), 1610)

    def complex_magnitude(self):
        sf = SnailFishNumber.from_list("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        self.assertEqual(sf.determine_magnitude(), 3488)

    def test_max_magnitude(self):
        magnitude_entries = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        ]
        max_magnitude = 0
        for [num_1, num_2] in list(itertools.permutations(magnitude_entries, 2)):
            sf1 = SnailFishNumber.from_list(num_1)
            sf2 = SnailFishNumber.from_list(num_2)
            sf = SnailFishNumber(sf1, sf2)
            sf.reduce()
            mag = sf.determine_magnitude()
            if mag > max_magnitude:
                max_magnitude = mag
        self.assertEqual(max_magnitude, 3993)

unittest.main()
