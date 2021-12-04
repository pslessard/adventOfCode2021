import os
from pathlib import Path
import sys

from pandas.core.algorithms import value_counts
sys.path.append(os.getcwd())
from util import util
from typing import List, Dict, Tuple
import numpy as np
import pandas as pd

def count_bits(vals, func):
    print(vals)
    return func(vals)


def get_closest(df, func) -> int:
    for col in df.columns:
        key = count_bits(df[col], func)
        df = df[df[col] == key]

        if (len(df) == 1):
            break
    
    return df


if (__name__ == "__main__"):
    example = False

    script_path = Path(os.path.realpath(__file__))
    if example:
        os.chdir(script_path.parent)

        with open('example.txt', 'r') as fd:
            lines = fd.readlines()

    else:
        lines = util.get_input(script_path.parent.name)
        
    lines = [line.strip() for line in lines]

    lines2d = pd.DataFrame([[char for char in line] for line in lines])

    def get_max(ser):
        values = ser.value_counts()
        if (values[0] == values[1]):
            return '1'
        else:
            return values.idxmax()

    def get_min(ser):
        max = get_max(ser)
        return '0' if max == '1' else '1'
    
    o2 = get_closest(lines2d,get_max).reset_index(drop=True)
    co2 = get_closest(lines2d, get_min).reset_index(drop=True)

    def getInt(vals):
        string = ''
        for val in vals:
            string += val
        
        return int(string, 2)
    
    o = getInt(o2.iloc[0].values)
    co = getInt(co2.iloc[0].values)

    print(o, co)
    print(o * co)