import json

class SnailFishNumber:
    def build(l, r):
        left = None
        right = None
        if isinstance(l, list):
            left = SnailFishNumber.build(l[0], l[1])
        else:
            left = l
        if isinstance(r, list):
            right = SnailFishNumber.build(r[0], r[1])
        else:
            right = r
        return SnailFishNumber(left, right)

    def from_list(text):
        # [[[[[9,8],1],2],3],4]
        data = json.loads(text)
        sf = SnailFishNumber.build(data[0], data[1])
        sf.set_depths()
        sf.set_parents()
        return sf

    def __init__(self, left, right):
        self.action_taken = False
        self.depth = 0
        self.direction = None
        self.left = left
        self.parent = None
        self.root = None
        self.right = right
        self.set_depths()
        self.set_parents()

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.to_list() == other.to_list()
        else:
            return False

    def set_depths(self):
        if self.left is not None and isinstance(self.left, SnailFishNumber):
            self.left.depth = self.depth + 1
            self.left.set_depths()
        if self.right is not None and isinstance(self.right, SnailFishNumber):
            self.right.depth = self.depth + 1
            self.right.set_depths()

    def set_parents(self, root=None):
        if root is None:
            root = self
            self.root = root
        if isinstance(self.left, SnailFishNumber):
            self.left.direction = "left"
            self.left.parent = self
            self.left.root = root
            self.left.set_parents(root)
        if isinstance(self.right, SnailFishNumber):
            self.right.direction = "right"
            self.right.parent = self
            self.right.root = root
            self.right.set_parents(root)

    def only_ints(self):
        return not isinstance(self.left, SnailFishNumber) and not isinstance(self.right, SnailFishNumber)

    def explode(self):
        if self.root.action_taken == True:
            return True
        if isinstance(self.left, SnailFishNumber):
            if self.left.depth == 4:
                print("LEFT   :", self.to_list())
                self.root.action_taken = True
                if isinstance(self.right, int):
                    self.right += self.left.right
                elif isinstance(self.right, SnailFishNumber):
                    self.right.walk_it_down("right", self.left.right)
                left_val = self.left.left
                self.try_set_val("left", left_val)
                self.left = 0
                return True
            else:
                self.left.explode()
        if not self.root.action_taken and isinstance(self.right, SnailFishNumber):
            if self.right.depth == 4:
                print("RIGHT  :", self.to_list())
                self.root.action_taken = True
                if isinstance(self.left, int):
                    self.left += self.right.left
                elif isinstance(self.left, SnailFishNumber):
                    self.left.walk_it_down("left", self.right.left)
                right_val = self.right.right
                self.right = 0
                self.try_set_val("right", right_val)
                return True
            else:
                self.right.explode()

    def try_set_val(self, direction, val):
        if self.root == self.parent and self.direction == direction:
            return
        if self.direction == direction:
            if self.parent is None:
                return
            self.parent.try_set_val(direction, val)
        elif direction == "right":
            if self.parent is not None and self.parent.right is not None:
                if isinstance(self.parent.right, int):
                    self.parent.right += val
                elif isinstance(self.parent.right, SnailFishNumber):
                    self.parent.right.walk_it_down("right", val)
        elif direction == "left":
            if self.parent is not None and self.parent.left is not None:
                if isinstance(self.parent.left, int):
                    self.parent.left += val
                elif isinstance(self.parent.left, SnailFishNumber):
                    self.parent.left.walk_it_down("left", val)

    def walk_it_down(self, direction, val):
        if direction == "right":
            if isinstance(self.left, SnailFishNumber):
                if isinstance(self.right, int):
                    self.right += val
                else:
                    self.left.walk_it_down(direction, val)
            elif isinstance(self.left, int):
                self.left += val
        if direction == "left":
            if isinstance(self.right, SnailFishNumber):
                if isinstance(self.left, int):
                    self.left += val
                else:
                    self.right.walk_it_down(direction, val)
            elif isinstance(self.right, int):
                self.right += val

    def split(self):
        if self.root.action_taken == True:
            return
        if isinstance(self.left, SnailFishNumber):
            self.left.split()
        elif isinstance(self.left, int) and self.left >= 10:
            new_left = int(self.left / 2)
            new_right = new_left + self.left % 2
            self.left = SnailFishNumber(
                left=new_left,
                right=new_right,
            )
            self.left.direction = "left"
            self.left.parent = self
            self.left.root = self.root
            self.set_depths()
            self.root.action_taken = True
            return
        if isinstance(self.right, SnailFishNumber):
            self.right.split()
        elif isinstance(self.right, int) and self.right >= 10 and not self.root.action_taken:
            new_left = int(self.right / 2)
            new_right = new_left + self.right % 2
            self.right = SnailFishNumber(
                left=new_left,
                right=new_right,
            )
            self.right.direction = "right"
            self.right.parent = self
            self.set_depths()
            self.right.root = self.root
            self.root.action_taken = True
            return
        return

    def reduce(self):
        print("INITIAL:",self.to_list())
        while True:
            self.explode()
            if self.action_taken:
                print("EXPLODE:", self.to_list())
                self.action_taken = False
                continue
            self.split()
            if self.action_taken:
                print("SPLIT:  ", self.to_list())
                self.action_taken = False
                continue
            if not self.action_taken:
                break

    def to_list(self):
        left = self.left
        if isinstance(self.left, SnailFishNumber):
            left = self.left.to_list()
        right = self.right
        if isinstance(self.right, SnailFishNumber):
            right = self.right.to_list()
        return [left, right]

    def determine_magnitude(self):
        left = self.left if self.left is not None else 0
        if isinstance(self.left, SnailFishNumber):
            left = self.left.determine_magnitude()
        right = self.right if self.right is not None else 0
        if isinstance(self.right, SnailFishNumber):
            right = self.right.determine_magnitude()
        return left * 3 + right * 2

    def __str__(self):
        return f"[{self.left},{self.right}]"


def main():
    with open("input.txt") as fh:
        sf = None
        entries = list()
        for line in fh.readlines():
            if sf is None:
                sf = SnailFishNumber.from_list(line)
                sf.reduce()
            else:
                sf = SnailFishNumber(sf, SnailFishNumber.from_list(line))
                sf.reduce()
    print(sf.determine_magnitude())


if __name__ == "__main__":
    main()
