"""
Use this script to tune probabilities
for each likelihood.

This runs a "game" for each likelihood
and prints how many times the event occurs in that game.
"""

import random

likelihoods = {
    'Impossible': 0,
    'Improbable': 0.00005,
    'Rare': 0.0005,
    'Unlikely': 0.005,
    'Random': 0.05,
    'Likely': 0.15,
    'Guaranteed': 1.0,
}

n_games = 10
turns_per_game = 12 * 100 # 1 turn = 1 month, over 1 century

def run_game(likelihood):
    count = 0
    for _ in range(turns_per_game):
        if random.random() < likelihoods[likelihood]:
            count += 1
    return count

for likelihood in likelihoods.keys():
    print(likelihood)
    for _ in range(n_games):
        count = run_game(likelihood)
        print(' ', count)