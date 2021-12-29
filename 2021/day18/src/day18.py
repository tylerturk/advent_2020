import json

class SnailFishNumber:
    def build(d, l, r):
        depth = d
        left = None
        right = None
        if isinstance(l, list):
            left = SnailFishNumber.build(d+1, l[0], l[1])
        else:
            left = l
        if isinstance(r, list):
            right = SnailFishNumber.build(d+1, r[0], r[1])
        else:
            right = r
        return SnailFishNumber(depth, left, right)

    def from_list(text):
        # [[[[[9,8],1],2],3],4]
        data = json.loads(text)
        sf = SnailFishNumber.build(0, data[0], data[1])
        sf.set_parents()
        return sf

    def __init__(self, depth, left, right):
        self.action_taken = False
        self.depth = depth
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

    def explode(self):
        if self.root.action_taken == True:
            self.root.action_taken = False
            return True
        exploded = False
        if isinstance(self.left, SnailFishNumber):
            if self.left.depth == 4:
                self.root.action_taken = True
                self.right += self.left.right
                left_val = self.left.left
                self.try_set_val("left", left_val)
                self.left = 0
                return True
            else:
                exploded = self.left.explode()
        if not exploded and isinstance(self.right, SnailFishNumber):
            if self.right.depth >= 4:
                self.root.action_taken = True
                self.left += self.right.left
                right_val = self.right.right
                self.right = 0
                self.try_set_val("right", right_val)
                return True
            elif not exploded:
                exploded = self.right.explode()

    def try_set_val(self, direction, val):
        if self.root == self.parent and self.direction == direction:
            return
        if self.direction == direction:
            if self.parent is None:
                return
            self.parent.try_set_val(direction, val)
        elif direction == "right":
            if self.parent is not None:
                if isinstance(self.parent.right, int):
                    self.parent.right += val
                elif isinstance(self.parent.right, SnailFishNumber):
                    self.parent.right.left += val
        elif direction == "left":
            if self.parent is not None:
                if isinstance(self.parent.left, int):
                    self.parent.left += val
                elif isinstance(self.parent.left, SnailFishNumber):
                    self.parent.left.right += val

    def split(self):
        if self.root.action_taken == True:
            self.root.action_taken = False
            return True
        if isinstance(self.left, SnailFishNumber):
            self.left.split()
        elif isinstance(self.left, int) and self.left >= 10:
            new_left = int(self.left / 2)
            new_right = new_left + self.left % 2
            self.left = SnailFishNumber(
                depth=self.depth + 1,
                left=new_left,
                right=new_right,
            )
            self.left.direction = "left"
            self.left.parent = self
            self.left.root = self.root
            self.root.action_taken = True
        if isinstance(self.right, SnailFishNumber):
            self.right.split()
        elif isinstance(self.right, int) and self.right >= 10:
            new_left = int(self.right / 2)
            new_right = new_left + self.right % 2
            self.right = SnailFishNumber(
                depth=self.depth + 1,
                left=new_left,
                right=new_right,
            )
            self.right.direction = "right"
            self.right.parent = self
            self.right.root = self.root
        return False

    def to_list(self):
        left = self.left
        if isinstance(self.left, SnailFishNumber):
            left = self.left.to_list()
        right = self.right
        if isinstance(self.right, SnailFishNumber):
            right = self.right.to_list()
        return [left, right]

    def __str__(self):
        return f"[{self.left}, {self.right}]"


def main():
    pass


if __name__ == "__main__":
    main()
