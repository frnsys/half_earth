import random
from db import find_sessions

ANIMALS = open('data/animals.txt').read().splitlines()
ADJECTIVES = open('data/adjectives.txt').read().splitlines()

def gen_id():
    animal = random.choice(ANIMALS)
    adj_1 = random.choice(ADJECTIVES)
    adj_2 = random.choice(ADJECTIVES)
    id = '{}-{}-{}'.format(adj_1, adj_2, animal).lower().replace(' ', '-')
    existing = find_sessions(id)
    if existing:
        return '{}-{}'.format(id, len(existing))
    else:
        return id
