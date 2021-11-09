import json
import numpy as np
import pandas as pd
from tqdm import tqdm
from collections import defaultdict

mapping = {
    'Micronesia (country)': 'Micronesia (Federated States of)',
    'Micronesia (Fed. States of)': 'Micronesia (Federated States of)',
    'Vietnam': 'Viet Nam',
    'Russia': 'Russian Federation',
    'South Korea': 'Republic of Korea',
    'North Korea': 'Democratic People\'s Republic of Korea',
    'Dem. People\'s Republic of Korea': 'Democratic People\'s Republic of Korea',
    'Iran': 'Iran (Islamic Republic of)',
    'Laos': 'Lao People\'s Democratic Republic',
    'Palestine': 'State of Palestine',
    'Democratic Republic of Congo': 'Democratic Republic of the Congo',
    'Moldova': 'Republic of Moldova',
    'Bolivia': 'Bolivia (Plurinational State of)',
    'Brunei': 'Brunei Darussalam',
    'Venezuela': 'Venezuela (Bolivarian Republic of)',
    'Syria': 'Syrian Arab Republic',
    'United States': 'United States of America',
    'Tanzania': 'United Republic of Tanzania',
    'The Bahamas': 'Bahamas',
    'Timor': 'Timor-Leste',
    'Cote d\'Ivoire': 'Côte d\'Ivoire',
    'Côte d’Ivoire': 'Côte d\'Ivoire',
    'United Kingdom': 'United Kingdom of Great Britain and Northern Ireland',
    'Taiwan': 'China, Taiwan Province of China',
    'Cape Verde': 'Cabo Verde',
    'Hong Kong': 'China, Hong Kong Special Administrative Region',
    'China, Hong Kong SAR': 'China, Hong Kong Special Administrative Region',
    'Faeroe Islands': 'Faroe Islands',
}

# To keep naming consistent/merge regions
region_mapping = {
    'South America': 'Southern America',
    'Australia and New Zealand': 'Australasia',
    'Micronesia': 'Oceania',
    'Melanesia': 'Oceania',
    'Polynesia': 'Oceania',
    'Channel Islands': 'Western Europe'
}

def get_standard_name(name):
    return mapping.get(name, name)

def standardize_names(df, key):
    df[key] = df[key].apply(get_standard_name)

# https://unstats.un.org/unsd/methodology/m49/overview/
df = pd.read_csv('src/UNSD — Methodology.csv')
standardize_names(df, 'Country or Area')

country_to_region = {}
region_to_countries = defaultdict(list)
for i, row in df[['Country or Area', 'Sub-region Name', 'Intermediate Region Name']].iterrows():
    country = row['Country or Area']
    if country == 'Antarctica': continue

    subregion = row['Sub-region Name']
    intregion = row['Intermediate Region Name']
    if type(intregion) is str:
        country_to_region[country] = region_mapping.get(intregion, intregion)
    else:
        country_to_region[country] = region_mapping.get(subregion, subregion)
country_to_region['China, Taiwan Province of China'] = 'Eastern Asia'
country_to_region['Cabo Verde'] = 'Western Africa'

for country, region in country_to_region.items():
    region_to_countries[region].append(country)

def region_for_country(country):
    country = get_standard_name(country)
    return country_to_region.get(country)


# https://ourworldindata.org/grapher/world-banks-income-groups
income_df = pd.read_csv('src/world-banks-income-groups.csv')
income_groups = {}
for entity, group in tqdm(income_df.groupby('Entity'), desc='Loading income groups'):
    name = get_standard_name(entity)
    income_groups[name] = {}
    for _, row in group.iterrows():
        income_groups[name][row['Year']] = row['Income classifications (World Bank (2017))']

last_income_group_year = 2016 # Latest year of income group data
def income_group_for_country_year(country, year=last_income_group_year):
    name = get_standard_name(country)
    return income_groups.get(name, {}).get(year)


# https://population.un.org/wpp/Download/Standard/Population/
# There are a number of different predictions under different
# conditions, we'll just use the "Medium" variant
# (which I believe is the same as "Median PI")
try:
    populations = json.load(open('/tmp/populations.json'))
except FileNotFoundError:
    pop_df = pd.read_csv('src/WPP2019_TotalPopulationBySex.csv')
    populations = {}
    for entity, group in tqdm(pop_df.groupby('Location'), desc='Loading populations'):
        name = get_standard_name(entity)
        populations[name] = {}
        for _, row in group.iterrows():
            if row['Time'] >= 1990 and row['Variant'] == 'Medium':
                # Population figures are in thousands
                populations[name][row['Time']] = row['PopTotal'] * 1000
    with open('/tmp/populations.json', 'w') as f:
        json.dump(populations, f)

    # Reload, so keys are in proper form
    populations = json.load(open('/tmp/populations.json'))

def population_for_country_year(country, year):
    name = get_standard_name(country)
    return populations.get(name, {}).get(str(year))
