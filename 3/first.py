import os
from pathlib import Path
import sys
sys.path.append(os.getcwd())
from util import util
from typing import List, Dict, Tuple

def count_bits(vals: List[str]) -> List[Dict[str, int]]:
    assert(len(vals) != 0)
    length = len(vals[0])

    count = [
        {
            '0': 0,
            '1': 0
        }
        for i in range(length)
    ]

    for val in vals:
        for i, char in enumerate(val):
            count[i][char] += 1
    
    return count


def get_values(count: List[Dict[str, int]]) -> Tuple[int, int]:
    gam = ''
    eps = ''

    for bit in count:
        gam += max(bit, key=bit.get)[0]
        eps += min(bit, key=bit.get)[0]

    return int(gam, 2), int(eps, 2)


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
    # print(lines)
    
    bit_count = count_bits(lines)
    gam, eps = get_values(bit_count)
    
    print(gam, eps)
    print(gam * eps)