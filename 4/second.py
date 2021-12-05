import os
from pathlib import Path
import sys
sys.path.append(os.getcwd())
from util import util
from typing import List, Dict, Tuple, Type
import numpy as np
import pandas as pd
from functools import reduce

import time


# https://stackoverflow.com/a/56231781
def _reduce_and(series):
    return reduce(np.logical_and, series)

def _reduce_or(series):
    return reduce(np.logical_or, series)


def parse_lines(lines):
    return [line.split() for line in lines]


def parse_input(lines: List[str]):
    lines = [line for line in lines if line]

    called = [int(val) for val in lines.pop(0).split(',')]

    assert(len(lines) % 5 == 0)
    boards = np.array([parse_lines(lines[board_start:board_start+5]) for board_start in range(0, len(lines), 5)])

    names = ['board', 'row', 'column']
    index = pd.MultiIndex.from_product([range(dimension) for dimension in boards.shape], names=names)
    df = pd.DataFrame({'boards': boards.flatten()}, index=index)['boards']
    df = df.apply(pd.to_numeric)

    df = df.unstack(level='row')
    
    return called, df


def parse_line(line: str) -> List[int]:
    return [int(num) for num in line.split()]


def call_num(boards, num):
    boards[boards == num] = np.nan


def _diagonal_is_win(df) -> bool:
    return np.logical_and.reduce(np.diagonal(df), initial=False)


def board_is_not_win(board):
    called = board.isnull()
    return not (
        called.aggregate(_reduce_and, axis=0).aggregate(_reduce_or) or
        called.aggregate(_reduce_and, axis=1).aggregate(_reduce_or) or
        _diagonal_is_win(called) or
        _diagonal_is_win(called[called.columns[::-1]]))

if (__name__ == "__main__"):
    example = False
    # example = True
    
    start = time.time()

    script_path = Path(os.path.realpath(__file__))
    if example:
        os.chdir(script_path.parent)

        with open('example.txt', 'r') as fd:
            lines = fd.readlines()

    else:
        lines = util.get_input(script_path.parent.name)
        
    lines = [line.strip() for line in lines]

    called, boards = parse_input(lines)

    mid = time.time()

    last = -1
    score = -1

    for num in called:
        call_num(boards, num)
        if (len(boards.groupby('board')) > 1):
            boards = boards.groupby('board').filter(board_is_not_win)
        else:
            last = num
            score = boards.aggregate(sum).aggregate(sum)
            break
    
    print('loser', last, score)
    print(last * score)

    end = time.time()

    print('Finished parsing in', mid - start)
    print('Finished in', end - start)