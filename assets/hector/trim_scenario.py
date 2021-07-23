import json

# Latest starting year for the game is 2050
name = 'rcp45'
end_year = 2050
scenario = json.load(open('{}.json'.format(name)))

n_years = end_year - scenario['startYear']
for key, vals in scenario['data'].items():
    scenario['data'][key] = vals[:n_years]

with open('{}.to_2050.json'.format(name), 'w') as f:
    json.dump(scenario, f)