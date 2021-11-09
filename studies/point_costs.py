def nth_point_cost(n):
    return round((n+2)**1.5)

for i in range(10):
    print('Point', i, '->', nth_point_cost(i))