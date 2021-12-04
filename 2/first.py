import os
from pathlib import Path

class Sub:
    def __init__(self):
        self.x = 0
        self.y = 0
    
    def up(self, val: int):
        self.y -= val
    
    def down(self, val: int):
        self.y += val
    
    def forward(self, val: int):
        self.x += val
    
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
    print(os.getcwd())
    script_path = Path(os.path.realpath(__file__))
    os.chdir(script_path.parent)
    print(os.getcwd())

    with open('input.txt', 'r') as fd:
        lines = fd.readlines()
        lines = [line.strip().split(' ') for line in lines]
        print(lines)
        lines = [(line[0], int(line[1])) for line in lines]
    
    sub = Sub()
    for cmd in lines:
        sub.move(cmd[0], cmd[1])
    
    print(sub.x * sub.y)