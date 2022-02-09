import random
from db import Database

db = Database('history.db')

ANIMALS = open('data/animals.txt').read().splitlines()
ADJECTIVES = open('data/adjectives.txt').read().splitlines()

def gen_id():
    # TODO this isn't really efficient and could
    # theoretically get stuck
    animal = random.choice(ANIMALS)
    adj_1 = random.choice(ADJECTIVES)
    adj_2 = random.choice(ADJECTIVES)
    id = '{}-{}-{}'.format(adj_1, adj_2, animal).lower().replace(' ', '-')
    if db.session(id) is None:
        return id
    else:
        return gen_id()
