if (__name__ == "__main__"):
    with open('input.txt', 'r') as fd:
        lines = fd.readlines()
        lines = [int(line.strip()) for line in lines]
    
    count = 0
    last = 999999
    for num in lines:
        if num > last:
            count += 1
        last = num
    
    print(count)