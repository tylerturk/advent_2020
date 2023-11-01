from typing import Optional


def load_file(file_name):
    with open(file_name, "r") as fh:
        return fh.readlines()


class File:
    def __init__(self, name: str, size: int):
        self.name: str = name
        self.size: int = size


class Directory:
    def __init__(self, path: str, parent=None):
        self.size: int = 0
        self.files: list[File, Directory] = []
        self.path: str = path
        self.parent: Optional[Directory] = parent
    def determine_size(self, reset=False):
        if self.size != 0 and not reset:
            return self.size
        size = 0
        directories = [x for x in self.files if type(x) == Directory]
        for dir in directories:
            size += dir.determine_size()
        files = [x for x in self.files if type(x) == File]
        for file in files:
            size += file.size
        self.size = size
        return size
    def find_sizes(self) -> int:
        directories = [x for x in self.files if type(x) == Directory]
        for dir in directories:
            yield dir.size
            yield from dir.find_sizes()
    def get_directory(self, path):
        return [x for x in self.files if type(x) == Directory and x.path == path][0]
    def print_filesystem(self, indent=0):
        print(f"{'  ' * indent}{self.path} (dir) - totalsize {self.size}")
        indent += 4
        directories = [x for x in self.files if type(x) == Directory]
        files = [x for x in self.files if type(x) == File]
        for file in files:
            print(f"{' ' * indent}{file.name} ({file.size})")
        for directory in directories:
            directory.print_filesystem(indent)
    def sum_below_target_recursive(self, target: int) -> int:
        size = 0
        if self.size < target:
            size = self.size
        directories = [x for x in self.files if type(x) == Directory]
        for dir in directories:
            size += dir.sum_below_target_recursive(target)
        return size


def parse_filesystem(data: list):
    filesystem = None
    current = None
    for line in data:
        line = line.strip()
        if filesystem is None:
            filesystem = Directory("/", None)
            current = filesystem
            continue
        if line.startswith("$"):
            line = line[2:].split()
            if line[0] == "ls":
                continue
            elif line[0] == "cd":
                if line[1] == "..":
                    current = current.parent
                else:
                    dir_name = line[1]
                    current = current.get_directory(dir_name)
        else:
            parts = line.split()
            if parts[0] == "dir":
                current.files.append(Directory(parts[1], current))
            else:
                current.files.append(File(parts[1], int(parts[0])))
    return filesystem


def part1(file_name="input.txt"):
    """
    >>> part1("sample.txt")
    95437
    """
    filesystem = parse_filesystem(load_file(file_name))
    filesystem.determine_size()
    return filesystem.sum_below_target_recursive(100000)


def part2(file_name="input.txt"):
    """
    >>> part2("sample.txt")
    24933642
    """
    fs_size = 70000000
    filesystem = parse_filesystem(load_file(file_name))
    filesystem.determine_size()
    current_usage = filesystem.size
    target_free = 30000000
    current_free = fs_size - current_usage
    need_to_free = target_free - current_free
    # Setting an initial high point
    to_clear = filesystem.size
    for size in filesystem.find_sizes():
        if size < to_clear and size > need_to_free:
            to_clear = size
    return to_clear


def main():
    print(part1("input.txt"))

    print(part2("input.txt"))


if __name__ == "__main__":
    main()