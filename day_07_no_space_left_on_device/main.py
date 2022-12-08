from __future__ import annotations

from typing import Dict, List, Optional, Union

INPUT_FILE_PATH = "input"

PARENT_ALIAS = ".."
MAX_DIR_SIZE = 100000
MINIMUM_SPACE_REQUIRED = 30000000
MAXIMUM_SPACE_AVAILABLE = 70000000


def main():
    input_lines = read_input_file(INPUT_FILE_PATH)

    root_dir = Directory("/")
    populate_filesystem(root_dir, input_lines[1:])  # ignore first line

    print(f"The sum of sizes is {get_sum_of_sizes(root_dir)}.")
    print(f"The smallest dir to delete has size {get_smallest_dir_size(root_dir, root_dir.size)}.")


class File:
    def __init__(self, name: str, size: int) -> None:
        self.name = name
        self.size = size


class Directory:
    def __init__(self, name: str, parent: Optional[Directory] = None) -> None:
        self.name = name
        self.parent = parent
        self.contents: Dict[str, Union[File, Directory]] = {}

    @property
    def size(self) -> int:
        return sum(element.size for element in self.contents.values())


def populate_filesystem(root: Directory, commands: List[str]) -> None:
    current_dir = root
    for line in commands:
        if line.startswith("$ ls"):
            continue  # noop
        elif line.startswith("$ cd"):
            current_dir = cd(current_dir, line.split(" ")[-1])
        else:
            size_or_dir, name = line.split(" ")
            current_dir.contents[name] = (
                Directory(name, current_dir)
                if size_or_dir == "dir" else
                File(name, int(size_or_dir))
            )


def cd(cwd: Directory, path: str) -> Directory:
    possible_cwd = cwd.parent if path == PARENT_ALIAS else cwd.contents[path]
    if not isinstance(possible_cwd, Directory):
        raise TypeError("Must be a Directory instance.")
    return possible_cwd


def get_sum_of_sizes(root: Directory, limit: int = MAX_DIR_SIZE) -> int:
    return sum(
        element.size + get_sum_of_sizes(element)
        if element.size <= limit else get_sum_of_sizes(element)
        for element in root.contents.values() if isinstance(element, Directory)
    )


def get_smallest_dir_size(root: Directory, root_size: int) -> int:
    minimum_size = MINIMUM_SPACE_REQUIRED - (MAXIMUM_SPACE_AVAILABLE - root_size)

    return min(
        (
            x for x in (
                get_smallest_dir_size(element, root_size)
                for element in root.contents.values() if isinstance(element, Directory)
            ) if x >= minimum_size
        ),
        default=root.size,
    )


def read_input_file(file_path: str) -> List[str]:
    with open(file_path, "r", encoding="utf-8") as file_descriptor:
        return [line.strip() for line in file_descriptor.readlines()]


if __name__ == "__main__":
    main()
