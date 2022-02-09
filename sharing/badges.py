BADGES = {
    'seceded': {
        'fn': lambda s: True,
        'desc': 'At least one region seceded from Gosplant.'
    },
    'aliens': {
        'fn': lambda s: True,
        'desc': 'You had an extraterrestrial encounter.'
    },
    'biodiversity': {
        'fn': lambda s: True,
        'desc': 'Planetary life flourished under your tenure.'
    },
    'electrification': {
        'fn': lambda s: True,
        'desc': 'You helped electrify the world.',
    },
    'extinction': {
        'fn': lambda s: False,
        'desc': 'Planetary life suffered under your tenure.',
    },
    'fossil_fuels': {
        'fn': lambda s: False,
        'desc': 'You kept on using fossil fuels.',
    },
    'meat': {
        'fn': lambda s: False,
        'desc': 'Carnivorous diets were left intact.'
    },
    'nuclear': {
        'fn': lambda s: False,
        'desc': 'Nuclear was your preferred form of energy',
    },
    'renewables': {
        'fn': lambda s: False,
        'desc': 'Renewables dominated energy production.',
    },
    'space': {
        'fn': lambda s: False,
        'desc': 'You pushed humanity towards the stars.',
    },
    'vegan': {
        'fn': lambda s: False,
        'desc': 'Global diets shifted towards vegan.',
    },
}


