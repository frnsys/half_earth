"""Determine how much each points takes off of a project's years"""

import math

def point_fn(points, years):
    # points = points**(1/2)
    points = points**(1/3)
    return nearest_multiple(years/points, 5)

def nearest_multiple(v, base):
    return base * round(v/base)

for years in [1, 10, 15, 20, 50, 100]:
    print('\nYears:', years)
    for i in range(10):
        points = i+1
        print(points, ' => ', point_fn(points, years))

# Notes: based on this, suggested minimum project years for research is 15

print('\n\n')

# Check how completion times change
# if point allocations are changed
years = 100
points = 5
progress = 0
print('Starting points:', points, '; target years:', years)
for i in range(100):
    if progress >= 1:
        print('Completed on turn:', i)
        break

    # Increase point allocations at some point
    if points < 10 and i > 30:
        points = 10
        print('--------------changed')

    actual_points = 1/point_fn(points, years)
    remaining = 1 - progress
    print(' ', math.ceil(remaining/actual_points), 'years left')
    progress += actual_points