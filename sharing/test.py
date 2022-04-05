import json
from badges import BADGES

state = json.load(open('data/example_game.json'))
for badge, spec in BADGES.items():
    print(badge, ':', spec['fn'](state))
