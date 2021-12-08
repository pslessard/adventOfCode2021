import requests
import os
import sys

def get_input(day: int, session_path: str = '~/.adventOfCodeSession'):
    with open(os.path.expanduser(session_path), 'r') as fd:
        session = fd.readline().strip()
    
    cookies = dict(session=session)

    url = f'http://adventofcode.com/2021/day/{day}/input'
    r = requests.get(url, cookies=cookies)
    
    return r.text.splitlines()


if __name__ == "__main__":
    # lines = get_input(sys.argv[1], '~/.session')
    lines = get_input(sys.argv[1])
    with open(f'{sys.argv[1]}/input.txt', 'w') as fd:
        for line in lines:
            fd.write(line)
            fd.write("\n")
        print(f'Written to {sys.argv[1]}/input.txt')