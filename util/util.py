import requests
import os

def get_input(day: int):
    with open(os.path.expanduser('~/.adventOfCodeSession'), 'r') as fd:
        session = fd.readline().strip()
    
    cookies = dict(session=session)

    url = f'http://adventofcode.com/2021/day/{day}/input'
    r = requests.get(url, cookies=cookies)
    
    return r.text.splitlines()