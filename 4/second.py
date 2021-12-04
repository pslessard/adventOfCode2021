import os
from pathlib import Path
import sys
sys.path.append(os.getcwd())
from util import util
from typing import List, Dict, Tuple, Type
import numpy as np
import pandas as pd
from functools import reduce


# https://stackoverflow.com/a/56231781
def _reduce_and(series):
    return reduce(np.logical_and, series)

def _reduce_or(series):
    return reduce(np.logical_or, series)


class Board:
    def __init__(self, lines: List[str], board_num: int):
        self._numbers = pd.DataFrame([parse_line(line) for line in lines])
        self._called = pd.DataFrame(np.zeros(self._numbers.shape, dtype=bool))
        self.id = board_num
        self.won = False

    def __repr__(self):
        string = ''
        string += repr(self._numbers)
        string += '\n'
        string += repr(self._called)
        return string

    def _is_orthogonal_win(self) -> Tuple[bool, np.ndarray]:
        vertical = self._called.aggregate(_reduce_and, axis=0)
        if vertical.aggregate(_reduce_or):
            return True, self._numbers[vertical == True].to_numpy()

        horizontal = self._called.aggregate(_reduce_and, axis=1)
        if horizontal.aggregate(_reduce_or):
            return True, self._numbers[horizontal == True].to_numpy()

        return False, None

    def _is_diagonal_win(self) -> Tuple[bool, np.ndarray]:
        def _diagonal_is_win(df) -> bool:
            return np.logical_and.reduce(np.diagonal(df), initial=False)

        first = _diagonal_is_win(self._called)
        if first:
            return True, np.diagonal(self._numbers)

        second = _diagonal_is_win(self._called[self._called.columns[::-1]])
        if second:
            return True, np.diagonal(self._numbers[self._numbers.columns[::-1]])

        return False, None

    def is_win(self) -> Tuple[bool, np.ndarray]:
        win, numbers = self._is_orthogonal_win()
        if win:
            self.won = True
            return win, numbers
        
        win, numbers = self._is_diagonal_win()
        if win:
            self.won = True
            return win, numbers
        
        return False, None

    def call_num(self, num: int) -> bool:
        self._called[self._numbers == num] = True
        return self.is_win()

    def sum(self) -> int:
        return np.add.reduce(np.add.reduce(self._numbers[self._called == False].fillna(0)))



def parse_input(lines: List[str]) -> Tuple[List[int], List[Type[Board]]]:
    lines = [line for line in lines if line]

    called = [int(val) for val in lines.pop(0).split(',')]

    boards = []

    assert(len(lines) % 5 == 0)
    for board_start in range(0, len(lines), 5):
        boards.append(Board(lines[board_start:board_start+5], board_start / 5))
    
    return called, boards


def parse_line(line: str) -> List[int]:
    return [int(num) for num in line.split()]


if (__name__ == "__main__"):
    example = False
    # example = True

    script_path = Path(os.path.realpath(__file__))
    if example:
        os.chdir(script_path.parent)

        with open('example.txt', 'r') as fd:
            lines = fd.readlines()

    else:
        lines = util.get_input(script_path.parent.name)
        
    lines = [line.strip() for line in lines]
    # print(lines)

    called, boards = parse_input(lines)

    num_won = 0
    game_over = False
    for num in called:
        # print('calling', num)
        for board in boards:
            if board.won:
                continue

            is_win, numbers = board.call_num(num)
            if is_win:
                num_won += 1
                print('Won:', board.id+1)
                
                if num_won == len(boards):
                    print('Loser:', board.id+1)
                    # print(board)
                    print(numbers)

                    sum = board.sum()
                    print(sum)
                    print(sum * num)

                    game_over = True
                    break

        if game_over:
            break