import os
from pathlib import Path
import sys
sys.path.append(os.getcwd())
from util import util

class Sub:
    def __init__(self):
        self.x = 0
        self.y = 0
        self.aim = 0
    
    def up(self, val: int):
        self.aim -= val
    
    def down(self, val: int):
        self.aim += val
    
    def forward(self, val: int):
        self.x += val
        self.y += (self.aim * val)
    
    def move(self, direction: str, val: int):
        if direction == 'up':
            self.up(val)
        
        elif direction == 'down':
            self.down(val)
        
        elif direction == 'forward':
            self.forward(val)
    
    def __repr__(self) -> str:
        return f'({self.x}, {self.y})'

if (__name__ == "__main__"):
    example = True

    script_path = Path(os.path.realpath(__file__))
    if example:
        os.chdir(script_path.parent)

        with open('example.txt', 'r') as fd:
            lines = fd.readlines()

    else:
        lines = util.get_input(script_path.parent.name)
    
    print(lines)
    
    lines = [line.strip().split(' ') for line in lines]
    lines = [(line[0], int(line[1])) for line in lines]
    
    sub = Sub()
    for cmd in lines:
        sub.move(cmd[0], cmd[1])
    
    print(sub.x * sub.y)