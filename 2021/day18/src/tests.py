from day18 import *
import unittest


class TestSnailFish(unittest.TestCase):
    def test_explodes(self):
        explode_1 = SnailFishNumber.from_list("[[[[[9,8],1],2],3],4]")
        explode_1.explode()
        expected = SnailFishNumber.from_list("[[[[0,9],2],3],4]")
        self.assertEqual(explode_1, expected)

        explode_2 = SnailFishNumber.from_list("[7,[6,[5,[4,[3,2]]]]]")
        explode_2.explode()
        expected = SnailFishNumber.from_list("[7,[6,[5,[7,0]]]]")
        self.assertEqual(explode_2, expected)

        explode_3 = SnailFishNumber.from_list("[[6,[5,[4,[3,2]]]],1]")
        explode_3.explode()
        expected = SnailFishNumber.from_list("[[6,[5,[7,0]]],3]")
        self.assertEqual(explode_3, expected)

        explode_4 = SnailFishNumber.from_list("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
        explode_4.explode()
        expected = SnailFishNumber.from_list("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        self.assertEqual(explode_4, expected)

        explode_5 = SnailFishNumber.from_list("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        explode_5.explode()
        expected = SnailFishNumber.from_list("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        self.assertEqual(explode_5, expected)

    def test_split(self):
        split_1 = SnailFishNumber.from_list("[1, [13, [5, 3]]]")
        split_1.split()
        expected = SnailFishNumber.from_list("[1, [[6, 7], [5, 3]]]")
        self.assertEqual(split_1, expected)

    def test_solve(self):
        sf1 = SnailFishNumber.from_list("[[[[4,3],4],4],[7,[[8,4],9]]]")
        sf2 = SnailFishNumber.from_list("[1,1]")
        sf = SnailFishNumber(0, sf1, sf2)
        sf.set_depths()
        sf.set_parents()
        while True:
            exploded = sf.explode()
            was_split = sf.split()
            sf.root.action_taken = False
            if not exploded and not was_split:
                break
        print("\n{}".format(sf))
        expected = SnailFishNumber.from_list("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        print(expected)

        self.assertEqual(sf, expected)


unittest.main()
