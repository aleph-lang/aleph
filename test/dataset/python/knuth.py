def knuth_test(n):
    sum = 0
    i = 1

    while(i <= n):
        sum = sum + i * i
        i = i + 1
    return sum

def knuth_recursive(n, current, sum):
    if(not(current <= n)):
        return sum
    else:
        return knuth_recursive(n, current + 1, sum + current * current)

def knuth_rec(n):
    return knuth_recursive(n, 1, 0)

def knuth_formula(n):
    return n * n + 1 * 2 * n + 1 / 6

print(knuth_test(1000))
print(knuth_rec(1000))
print(knuth_formula(1000))
