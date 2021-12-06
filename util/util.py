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
    lines = get_input(sys.argv[1], '~/.session')
    with open(f'{sys.argv[1]}/input.txt', 'w') as fd:
        fd.writelines(lines)