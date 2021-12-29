from day18 import *
import unittest


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
        sf = SnailFishNumber(0, sf1, sf2)
        sf.set_depths()
        sf.set_parents()
        for _ in range(1, 20):
            exploded = sf.explode()
            was_split = sf.split()
            sf.root.action_taken = False
        expected = SnailFishNumber.from_list("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")

        self.assertEqual(sf, expected)


unittest.main()
