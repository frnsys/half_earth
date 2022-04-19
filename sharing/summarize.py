import random
from badges import BADGES

def summarize(scenario):
    badges = []
    for badge, spec in BADGES.items():
        if spec['fn'](scenario):
            badges.append(badge)

    return {
        'win': scenario['win'],
        'badges': badges,
        'faction': estimate_faction(scenario),
        'scenario': scenario,
    }


def estimate_faction(scenario):
    ally = max(scenario['npcs'], key=lambda npc: npc['relationship'])
    max_val = ally['relationship']
    top = []
    for npc in scenario['npcs']:
        if npc['relationship'] == max_val:
            top.append(npc)
    if len(top) > 1:
        ally = random.choice(top)
    name = ally['name']
    return name[0].lower() + name[1:] + 's'


if __name__ == '__main__':
    import json
    scenario = json.load(open('example.json'))
    print(summarize(scenario))
