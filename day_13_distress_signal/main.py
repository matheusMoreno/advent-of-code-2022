"""Day 13 challenge, written in Python."""

from ast import literal_eval
from functools import cmp_to_key
from typing import List, Tuple, Optional

INPUT_FILE_PATH = "input"

DIVIDER_PACKETS = [[[2]], [[6]]]
IS_IN_ORDER_MAPPING = {True: -1, False: 1, None: 0}


def main():
    """Resolve the problem."""
    input_str = read_input_file(INPUT_FILE_PATH)

    lists_pairs = parse_lists_pairs(input_str)
    n_ordered_lists = count_ordered_lists(lists_pairs)

    lists_unsorted = join_lists_with_divider_packets(input_str)
    decoder_key = compute_decoder_key(lists_unsorted)

    print(f"Sum of indexes of ordered lists: {n_ordered_lists}")
    print(f"Value of decoder key: {decoder_key}")


def read_input_file(file_path: str) -> str:
    """Read input file without splitting its lines."""
    with open(file_path, "r", encoding="utf-8") as file_descriptor:
        return file_descriptor.read().strip()


def parse_lists_pairs(input_str: str) -> List[Tuple[list, list]]:
    """Parse input into a list containing each pair of lists."""
    return [
        tuple(literal_eval(e) for e in pairs.split("\n"))  # type: ignore
        for pairs in input_str.split("\n\n")
    ]


def count_ordered_lists(lists_pairs: List[Tuple[list, list]]) -> int:
    """Count number of ordered lists in input."""
    return sum(
        index if is_in_order(left, right) else 0
        for index, (left, right) in enumerate(lists_pairs, start=1)
    )


def join_lists_with_divider_packets(input_str: str) -> List[list]:
    """Parse lists ignoring the blank lines."""
    return [
        literal_eval(element)
        for element in input_str.replace("\n\n", "\n").split("\n")
    ] + DIVIDER_PACKETS


def compute_decoder_key(lists: List[list]) -> int:
    """Sort lists and compute decoder key."""
    indices = [
        i
        for i, l in enumerate(sort_lists(lists), start=1)
        if l in DIVIDER_PACKETS
    ]

    decoder_key = 1
    for index in indices:
        decoder_key *= index
    return decoder_key


def sort_lists(lists: List[list]) -> List[list]:
    """Sort all lists."""
    return sorted(
        lists,
        key=cmp_to_key(  # type: ignore
            lambda x, y: IS_IN_ORDER_MAPPING[is_in_order(x, y)]
        )
    )


def is_in_order(left: list, right: list) -> Optional[bool]:
    """Check if two lists are ordered."""
    element_cases = {
        (list, list): is_in_order,
        (list, int): lambda l, i: is_in_order(l, [i]),
        (int, list): lambda i, l: is_in_order([i], l),
        (int, int): lambda x, y: None if x == y else x < y,
    }

    for elem_left, elem_right in zip(left, right):
        index_case = (type(elem_left), type(elem_right))
        elements_ordered = element_cases[index_case](elem_left, elem_right)
        if elements_ordered is not None:
            return elements_ordered

    return None if len(left) == len(right) else len(left) < len(right)


if __name__ == "__main__":
    main()
