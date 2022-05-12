import random
from badges import BADGES

def summarize(scenario):
    badges = []
    for badge, spec in BADGES.items():
        try:
            if spec['fn'](scenario):
                badges.append(badge)
        except:
            print('Error evaluating badge: {}'.format(badge))

    return {
        'win': scenario['win'],
        'badges': badges,
        'faction': estimate_faction(scenario),
        'scenario': scenario,
    }


def estimate_faction(scenario):
    allies = [npc for npc in scenario['npcs'] if npc['is_ally']]
    weights = [npc['relationship'] for npc in allies]
    if not allies:
        closest_ally = max(scenario['npcs'], key=lambda npc: npc['relationship'])
        allies = [closest_ally]
        weights = [closest_ally['relationship']]
    ally = random.choices(population=allies, weights=weights, k=1)[0]
    name = ally['name']
    return name[0].lower() + name[1:] + 's'


if __name__ == '__main__':
    import json
    scenario = json.load(open('example.json'))
    print(summarize(scenario))
