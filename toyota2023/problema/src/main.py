def main(n, s):
    if n == 3:
        return 3
    
    abc = [False, False, False]

    for i, char in enumerate(s):
        if char == 'A':
            abc[0] = True
        elif char == 'B':
            abc[1] = True
        elif char == 'C':
            abc[2] = True

        if abc[0] and abc[1] and abc[2]:
            return i+1

if __name__ == '__main__':
    n = int(input())
    s = input()
    print(main(n, s))



