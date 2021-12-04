if (__name__ == "__main__"):
    with open('input.txt', 'r') as fd:
        lines = fd.readlines()
        lines = [int(line.strip()) for line in lines]
    
    count = 0
    for i, num in enumerate(lines):
        if i < 3:
            continue

        if num > lines[i-3]:
            count += 1
    
    print(count)