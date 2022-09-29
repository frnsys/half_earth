BADGES = {
    'seceded': {
        'fn': lambda s: any(r['seceded'] for r in s['world']['regions']),
        'desc': 'At least one region seceded from Gosplant.'
    },
    'aliens': {
        'fn': lambda s: 'AlienEncounter' in s['flags'],
        'desc': 'You had an extraterrestrial encounter.'
    },
    'biodiversity': {
        'fn': lambda s: s['world']['extinction_rate'] is not None and s['world']['extinction_rate'] <= 15,
        'desc': 'Planetary life flourished under your tenure.'
    },
    'electrification': {
        'fn': lambda s: any(
            p['name'] == 'Mass Electrification' and (p['status'] == 'Finished' or p['status'] == 'Active')
            for p in s['projects']),
        'desc': 'You helped electrify the world.',
    },
    'extinction': {
        'fn': lambda s: s['world']['extinction_rate'] is not None and s['world']['extinction_rate'] >= 60,
        'desc': 'Planetary life suffered under your tenure.',
    },
    'fossil_fuels': {
        'fn': lambda s: sum(p['mix_share'] for p in s['processes'] if 'IsFossil' in p['features']) > 0,
        'desc': 'You kept on using fossil fuels.',
    },
    'meat': {
        # Animal calories demand at least 80% of starting value
        'fn': lambda s: (s['output_demand']['animal_calories'] + s['output_demand_extras']['animal_calories']) * s['output_demand_modifier']['animal_calories'] >= 2e15,
        'desc': 'Carnivorous diets were left intact.'
    },
    'nuclear': {
        'fn': lambda s: sum(p['mix_share'] for p in s['processes'] if ('CanMeltdown' in p['features'] or 'MakesNuclearWaste' in p['features'])) >= 10,
        'desc': 'Nuclear was your preferred form of energy.',
    },
    'renewables': {
        'fn': lambda s: sum(p['mix_share'] for p in s['processes'] if 'IsIntermittent' in p['features']) >= 10,
        'desc': 'Renewables dominated energy production.',
    },
    'space': {
        'fn': lambda s: sum(1 for p in s['projects'] if p['group'] == 'Space' and (p['status'] == 'Finished' or p['status'] == 'Active')) >= 3,
        'desc': 'You pushed humanity towards the stars.',
    },
    'vegan': {
        # Animal calories demand down to less than 10% of starting val
        'fn': lambda s: (s['output_demand']['animal_calories'] + s['output_demand_extras']['animal_calories']) * s['output_demand_modifier']['animal_calories'] < 2e14,
        'desc': 'Global diets shifted towards vegan.',
    },
}
