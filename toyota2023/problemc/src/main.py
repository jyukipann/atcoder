import numpy as np

def main():
    n = int(input())
    a = list(map(int, input().split()))
    if n == 2:
        print('2')
        print('1 2')
        return
    adjacency_matrix = np.zeros((n,n), bool)
    for i, a_i in enumerate(a):
        adjacency_matrix[i, a_i-1] = True
    
    # starting_candidate = list(range(n))
    a_prime = set((np.array(a) - 1).tolist())
    starting_candidate = list(a_prime)
    # starting_candidate = set(starting_candidate)
    # starting_candidate = starting_candidate & a_prime


    while len(starting_candidate) >= 3:
        started_vertex = starting_candidate.pop(0)
        b = [started_vertex+1]
        current_vertex = started_vertex
        while True:
            next_vertex = adjacency_matrix[current_vertex,:].argmax()
            if next_vertex == started_vertex:
                print(len(b))
                print(' '.join(map(str, b)))
                return
            if next_vertex + 1 in b:
                break
            current_vertex = next_vertex
            b.append(current_vertex+1)
    
    print(2)
    print(starting_candidate[0], starting_candidate[1])

if __name__ == '__main__':
    main()