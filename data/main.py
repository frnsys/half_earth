"""
1. Get income group for country for year
2. Different income group scaling factors:
    - Energy use
    - Material use (this is for "Industries" that we aren't modeling at the proces-level)
    - Food use
    - Water use
3. Calculate majority income group for each region
"""
import ref
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from tqdm import tqdm
from collections import defaultdict

income_group_names = [
    'Low income',
    'Lower-middle income',
    'Upper-middle income',
    'High income'
]

def per_capita_by_income_groups(csv, key, country_key='Entity', year_key='Year', is_per_capita=False, scale=1):
    df = pd.read_csv(csv)
    missing = []
    years = defaultdict(int)
    group_vals = defaultdict(list)
    for entity, group in df.groupby(country_key):
        # Get data for latest year available
        last_year = group[year_key].max()
        row = group[group[year_key] == last_year].iloc[0]

        years[last_year] += 1

        income_group = ref.income_group_for_country_year(entity, last_year)
        if income_group is None:
            income_group = ref.income_group_for_country_year(entity)
        if income_group == 'Not categorized': continue
        if income_group is None:
            missing.append((entity, 'income group'))
            continue

        population = ref.population_for_country_year(entity, last_year)
        if population is None:
            missing.append((entity, 'population'))
            continue

        if isinstance(key, list):
            val = sum(row[k] if not np.isnan(row[k]) else 0 for k in key)
        else:
            val = row[key]

        if is_per_capita:
            val *= population
        group_vals[income_group].append((val * scale, population))

    group_pops = {}
    group_totals = {}
    group_per_capitas = {}
    for group in income_group_names:
        vals = group_vals[group]
        total_pop = sum(pop for _, pop in vals)
        total_val = sum(val for val, _ in vals)
        per_capitas = [val/pop for val, pop in vals]
        per_capita_mean = total_val/total_pop
        print(group)
        print('  Per capita mean:', per_capita_mean)
        print('  Per capita range:', np.min(per_capitas), '-', np.max(per_capitas))
        group_pops[group] = total_pop
        group_totals[group] = total_val
        group_per_capitas[group] = per_capita_mean

    print('\nScaling:')
    income_scales = {}
    adj_group_pops = {}
    for group in income_group_names:
        scale = group_per_capitas[group]/group_per_capitas['Low income']
        income_scales[group] = scale
        print('{: >20}'.format(group), '{:.2f}'.format(scale))
        adj_group_pops[group] = scale * group_pops[group]

    print('\nTotal:', sum(group_totals.values()))

    print('Years:', ['{}:{}'.format(y, years[y]) for y in sorted(list(years))])
    if missing:
        print('\nMissing:')
        for entity, key in missing:
            print(' ', key, 'for', entity)

    return adj_group_pops, group_totals


def title(title):
    print('\n\u001b[4m{}\u001b[0m'.format(title))


def ej_to_kwh(ej):
    return ej*277.778*1e9

n2o_gwp = 298
ch4_gwp = 84


if __name__ == '__main__':
    print('Income population change coefficients')
    # Calculate polynomials for population projections
    income_group_countries = defaultdict(list)
    for country, years in ref.income_groups.items():
        group = years[2016]
        income_group_countries[group].append(country)

    income_group_pop_changes = {}
    for group, countries in income_group_countries.items():
        trajectory = []
        for year in range(2020, 2101):
            changes = []
            for country in countries:
                pop_prev = ref.population_for_country_year(country, year-1)
                pop_next = ref.population_for_country_year(country, year)
                if pop_prev is None: continue
                change = pop_next/pop_prev - 1
                changes.append(change)
            trajectory.append(np.mean(changes))
        income_group_pop_changes[group] = trajectory

    for group, vals in income_group_pop_changes.items():
        y = np.array(vals)
        X = np.array([2020 + i for i in range(0, len(y))])
        coefs = np.polyfit(X, y, 3)
        p = np.poly1d(coefs)
        # plt.title(group)
        # plt.plot(X, y, '.', X, p(X), '-')
        # plt.show()
        print(group)
        for coef in coefs[::-1]: # polyfit returns coefs in opposite order
            print(' ', coef)

    print('Starting populations for regions')
    year = 2022
    for region, countries in ref.region_to_countries.items():
        print(region)
        pop = sum(filter(None, [ref.population_for_country_year(c, year) for c in countries]))
        print(' ', pop)

    # https://ourworldindata.org/grapher/municipal-water-withdrawal
    # Original data in m3/year, change to L/year
    title('Municipal/household water withdrawals (L/year):')
    per_capita_by_income_groups('src/municipal-water-withdrawal.csv', 'Municipal water withdrawal', scale=1000)

    # https://ourworldindata.org/grapher/industrial-water-withdrawal
    # Original data in m3/year, change to L/year
    title('Industrial water withdrawals (L/year):')
    ind_water_group_pops, ind_water_group_totals = per_capita_by_income_groups('src/industrial-water-withdrawal.csv', 'Industrial water withdrawal', scale=1000)

    # https://ourworldindata.org/grapher/per-capita-energy-use
    # Original data in kWh/year
    title('Energy use (kWh/year):')
    per_capita_by_income_groups('src/per-capita-energy-use.csv', 'Energy consumption per capita (kWh)', is_per_capita=True)

    # https://ourworldindata.org/grapher/per-capita-electricity-consumption
    # Original data per year
    title('Electricity use (kWh/year)')
    per_capita_by_income_groups('src/per-capita-electricity-consumption.csv', 'Per capita electricity (kWh)', is_per_capita=True)

    print('(For fuel, calculate the difference b/w energy use per capita and electricity use per capita.)')

    # https://wesr.unep.org/indicator/index/12_2_1
    title('Material footprints (tonnes)')
    material_group_pops, _ = per_capita_by_income_groups('src/material_footprint/12_2_1_country_data_total_mf_per_capita.csv', 'data_value', country_key='country_name', year_key='year', is_per_capita=True)
    total_adj_material_pop = sum(material_group_pops.values())

    print('\nTotal material LIC pop:', total_adj_material_pop)

    # https://ourworldindata.org/grapher/dietary-composition-by-country
    # Original data in kcal/day
    title('Plant calories (kcal/year)')
    _, plant_calorie_totals = per_capita_by_income_groups('src/dietary-composition-by-country.csv', [
       'Miscellaneous (FAO (2017))',
       'Alcoholic Beverages (FAO (2017))',
       'Vegetable Oils (FAO (2017))',
       'Oilcrops (FAO (2017))',
       'Sugar Crops (FAO (2017))',
       'Sugar & Sweeteners (FAO (2017))',
       'Starchy Roots (FAO (2017))',
       'Nuts & Seeds (FAO (2017))',
       'Fruit (FAO (2017))',
       'Vegetables (FAO (2017))',
       'Pulses (FAO (2017))',
       'Cereals, Other (FAO (2017))',
       'Barley (FAO (2017))',
       'Maize (FAO (2017))',
       'Rice (FAO (2017))',
       'Wheat (FAO (2017))',
    ], is_per_capita=True, scale=365)

    # Original data in kcal/day, change to kcal/year
    title('Animal calories (kcal/year)')
    _, animal_calorie_totals = per_capita_by_income_groups('src/dietary-composition-by-country.csv', [
       'Animal fats (FAO (2017))',
       'Fish & seafood (FAO (2017))',
       'Meat, Other (FAO (2017))',
       'Mutton & Goat Meat (FAO (2017))',
       'Pigmeat (FAO (2017))',
       'Poultry Meat (FAO (2017))',
       'Bovine Meat (FAO (2017))',
       'Eggs (FAO (2017))',
       'Milk (FAO (2017))',
    ], is_per_capita=True, scale=365)

    title('Income groups for regions')
    income_levels = {'Low income': 1, 'Lower-middle income': 2, 'Upper-middle income': 3, 'High income': 4}
    level_incomes = {l: i for i, l in income_levels.items()}
    for region, countries in ref.region_to_countries.items():
        groups = [ref.income_group_for_country_year(c, ref.last_income_group_year) for c in countries]
        levels = [income_levels[g] for g in groups if g is not None and g != 'Not categorized']
        level = np.mean(levels)
        print('{: >20}'.format(region), '{:.2f}'.format(level), '=>', level_incomes[round(level)])

    # https://ourworldindata.org/grapher/healthcare-access-and-quality-index
    title('Healthcare quality index')
    per_capita_by_income_groups('src/healthcare-access-and-quality-index.csv', 'HAQ Index (IHME (2017))', is_per_capita=True)

    # Other calculations
    # ==================

    # Reminder: one metric liter is one cubic decimeter
    # https://ourworldindata.org/grapher/water-requirement-per-kilocalorie
    # From Drew's model, assume a 34% yield gap b/w organic and conventional ag
    # https://colab.research.google.com/drive/1XD4QtHowmy7GJm2g6VVPXPKVZf7SLq0W?usp=sharing#scrollTo=q8jy-2nlYaLD
    # Verena Seufert et. al., ‘Comparing the Yields of Organic and Conventional Agriculture’, Nature 485, no. 7397 (2012): 229–32
    organic_yield = 0.66

    # https://ourworldindata.org/smallholder-food-production
    # Smallholders use 24% of ag land, produce 29% of crops (in kcals), some of which go into fuel and feed, and ultimately contribute to 32% of the food supply (directly consumed crops).
    # So assume if they produce 29% of crops with 24% of land, about 21% greater yield than conventional ag.
    smallholder_yield = 1.21
    title('Water requirements for food (liter/kcal)')
    df = pd.read_csv('src/water-requirement-per-kilocalorie.csv')
    animal = [
        'Beef',
        'Butter',
        'Chicken meat',
        'Eggs',
        'Milk',
        'Pigmeat',
        'Sheep/goat meat',
    ]
    plant = [
        'Cereals',
        'Fruits',
        'Nuts',
        'Oil crops',
        'Pulses',
        'Starchy roots',
        'Sugar crops',
        'Vegetables',
    ]
    mean_animal_water = df[df.Entity.isin(animal)]['Water requirement per calorie'].mean()
    mean_plant_water = df[df.Entity.isin(plant)]['Water requirement per calorie'].mean()
    print('  Conventional:')
    print('    Animal:', mean_animal_water)
    print('    Plant:', mean_plant_water)
    print('  Organic:')
    print('    Animal:', mean_animal_water/organic_yield)
    print('    Plant:', mean_plant_water/organic_yield)
    print('  Smallholder:')
    print('    Plant:', mean_plant_water/smallholder_yield)

    # https://ourworldindata.org/grapher/key-crop-yields
    title('Crop yield improvements (2000 to 2017)')
    df = pd.read_csv('src/key-crop-yields.csv')
    crops = [
       'Crops - Wheat - 15 - Yield - 5419 - hg/ha',
       'Crops - Rice, paddy - 27 - Yield - 5419 - hg/ha',
       'Crops - Maize - 56 - Yield - 5419 - hg/ha',
       'Crops - Soybeans - 236 - Yield - 5419 - hg/ha',
       'Crops - Potatoes - 116 - Yield - 5419 - hg/ha',
       'Crops - Beans, dry - 176 - Yield - 5419 - hg/ha',
       'Crops - Peas, dry - 187 - Yield - 5419 - hg/ha',
       'Crops - Cassava - 125 - Yield - 5419 - hg/ha',
       'Crops - Barley - 44 - Yield - 5419 - hg/ha',
       'Crops - Cocoa, beans - 661 - Yield - 5419 - hg/ha',
       'Crops - Bananas - 486 - Yield - 5419 - hg/ha'
    ]
    year_2000 = df[df['Year'] == 2000]
    yield_means_by_crop = [year_2000[crop].mean() for crop in crops]
    yield_mean_2000 = np.mean(yield_means_by_crop)

    year_2017 = df[df['Year'] == 2017]
    yield_means_by_crop = [year_2017[crop].mean() for crop in crops]
    yield_mean_2017 = np.mean(yield_means_by_crop)
    print(' ', yield_mean_2017/yield_mean_2000)

    # Industries
    # Final consumption energy values, in EJ/year, for 2020
    # https://www.iea.org/reports/key-world-energy-statistics-2021/final-consumption
    # https://iea.blob.core.windows.net/assets/52f66a88-0b63-4ad2-94a5-29d36e864b82/KeyWorldEnergyStatistics2021.pdf
    title('Non-modeled industry requirements per LIC')
    energy = {
        'coal': ej_to_kwh(40),
        'oil': ej_to_kwh(169),
        'natural_gas': ej_to_kwh(68),
        'electricity': ej_to_kwh(82),
    }
    industries = {
        'Iron and Steel': {
            'coal': 0.34
        },
        'Road Transport': {
            'oil': 0.492
        },
        'Aviation': {
            'oil': 0.086
        },
        'Shipping': {
                   # Maritime shipping
            'oil': 0.067 + 0.008
                           # Rail
        },
        'Chemical': { # Chemical and petrochemicals
            'coal': 0.075,
            'co2': 0.022  # Non-energy CO2eq
        },
        'Concrete': { # Non-metallic minerals
            'coal': 0.217,
            'co2': 0.03  # Non-energy CO2eq
        },
        'Buildings': { # Residential & commercial
            'electricity': 0.266 + 0.212,
            'natural_gas': 0.297 + 0.128,
            'oil': 0.053,
            'coal': 0.064
        },
        'Other Industry': {
            'electricity': 0.018 + 0.419,
            'natural_gas': 0.119 + 0.073 + 0.374,
            'oil': 0.167 + 0.073,
            'coal': 0.089 + 0.121 + 0.052,
        },
        'Agriculture': {
        }
    }
    # https://ourworldindata.org/emissions-by-sector#sector-by-sector-where-do-global-greenhouse-gas-emissions-come-from
    co2_emissions = 4.94e16 # 2016, gCO2eq/year

    for industry, usages in industries.items():
        print(industry)
        ind_uses = {}
        for k, p in usages.items():
            val = co2_emissions if k == 'co2' else energy[k]
            usage = val * p
            unit = 'gCO2eq' if k == 'co2' else 'kWh'
            ind_uses[k] = usage/total_adj_material_pop # should be per year
            print('{:>15}'.format(k), '{:.2f}{}/lic/year'.format(ind_uses[k], unit))
        fuel = ind_uses.get('coal', 0) + ind_uses.get('oil', 0) + ind_uses.get('natural_gas', 0)
        print('{:>15}'.format('fuel'), '{:.2f}kWh/lic/year'.format(fuel))

    title('Industrial water usage L/lic/year')
    print(sum(ind_water_group_totals.values())/sum(ind_water_group_pops.values()))

    title('Agriculture')

    # Not sure what year this land use data is for
    # https://ourworldindata.org/land-use#breakdown-of-global-land-use-today
    print('  Land (m2/kcal):')
    livestock_land_use = 4e13 # m2, this includes crop and grazing land
    total_animal_calories = sum(animal_calorie_totals.values())
    print('    Animal Conventional:', livestock_land_use/total_animal_calories)
    print('    Animal Organic:', livestock_land_use/total_animal_calories/organic_yield)

    crop_land_use = 1.1e13 # m2, this does not include crop and grazing land
    total_plant_calories = sum(plant_calorie_totals.values())
    land_use_per_plant_calorie = crop_land_use/total_plant_calories
    print('    Plant Conventional:', land_use_per_plant_calorie)
    print('    Plant Organic:', land_use_per_plant_calorie/organic_yield)
    print('    Plant Smallholder:', land_use_per_plant_calorie/smallholder_yield)

    # All crop calories, including feed.
    # Use a feed efficiency conversion of 12% (from https://iopscience.iop.org/article/10.1088/1748-9326/8/3/034015)
    feed_conversion_efficiency = 0.12
    total_crop_calories = total_plant_calories + total_animal_calories/feed_conversion_efficiency

    # Non-energy agricultural emissions
    # https://ourworldindata.org/emissions-by-sector#sector-by-sector-where-do-global-greenhouse-gas-emissions-come-from
    crop_calorie_co2 = {
        'Crop burning': 0.035,
        'Deforestation': 0.022,
        'Cropland': 0.014,
        'Grassland Degradation': 0.001,
    }
    crop_calorie_n2o = {
        'Syn. fertilizer soil emissions': 0.041,
    }
    animal_calorie_ch4 ={
        'Livestock & manure': 0.058
    }
    # import ipdb; ipdb.set_trace()

    print('  CO2 (gCO2/kcal):')
    co2_per_plant_calorie = (sum(crop_calorie_co2.values()) * co2_emissions)/total_crop_calories
    co2_per_animal_calorie = co2_per_plant_calorie/feed_conversion_efficiency
    print('    Plant Conventional:', co2_per_plant_calorie)
    print('    Plant Organic:', co2_per_plant_calorie/organic_yield)
    print('    Plant Smallholder:', co2_per_plant_calorie/smallholder_yield)
    print('    Animal Conventional:', co2_per_animal_calorie)
    print('    Animal Organic:', co2_per_animal_calorie/organic_yield)

    print('  N2O (gN2O/kcal):')
    n2o_per_plant_calorie = ((sum(crop_calorie_n2o.values()) * co2_emissions)/n2o_gwp)/total_crop_calories
    n2o_per_animal_calorie = n2o_per_plant_calorie/feed_conversion_efficiency
    print('    Plant Conventional:', n2o_per_plant_calorie)
    print('    Animal Conventional:', n2o_per_animal_calorie)
    # Assuming organic doesn't use any synthetic fertilizer

    print('  CH4 (gCH4/kcal):')
    ch4_per_animal_calorie = ((sum(animal_calorie_ch4.values()) * co2_emissions)/ch4_gwp)/total_animal_calories
    print('    Animal Conventional:', ch4_per_animal_calorie)
    print('    Animal Organic:', ch4_per_animal_calorie/organic_yield)

    # Agriculture energy
    print('  Electricity (kWh/kcal):')
    ag_electricity = 588941670000 # kWh, for 2015, https://energyeducation.ca/encyclopedia/Agricultural_energy_use
    kwh_per_plant_calorie = ag_electricity/total_crop_calories
    kwh_per_animal_calorie = kwh_per_plant_calorie/feed_conversion_efficiency
    print('    Plant Conventional:', kwh_per_plant_calorie)
    print('    Plant Organic:', kwh_per_plant_calorie/organic_yield)
    print('    Plant Smallholder:', kwh_per_plant_calorie/smallholder_yield)
    print('    Animal Conventional:', kwh_per_animal_calorie)
    print('    Animal Organic:', kwh_per_animal_calorie/organic_yield)

    print('  Fuel (kWh/kcal):')
    ag_fuel = 1.6551e+12 # kWh, for 2016, https://energyeducation.ca/encyclopedia/Agricultural_energy_use
    fuel_per_plant_calorie = ag_fuel/total_crop_calories
    fuel_per_animal_calorie = fuel_per_plant_calorie/feed_conversion_efficiency
    print('    Plant Conventional:', fuel_per_plant_calorie)
    print('    Plant Organic:', fuel_per_plant_calorie/organic_yield)
    print('    Plant Smallholder:', fuel_per_plant_calorie/smallholder_yield)
    print('    Animal Conventional:', fuel_per_animal_calorie)
    print('    Animal Organic:', fuel_per_animal_calorie/organic_yield)

    # Regenerative ag carbon sequestration
    # https://www.frontiersin.org/articles/10.3389/fclim.2019.00008/full
    # "Despite somewhat different scope (land types included) and assumptions (practices considered), there is fairly close alignment among global estimates (Figure 1), suggesting a technical soil C sequestration potential of 2–5 Gt CO2 per year, for what were characterized in the section above as existing best conservation management practices."
    # And based on https://www.quantamagazine.org/a-soil-science-revolution-upends-plans-to-fight-climate-change-20210727/
    # these numbers may be optimistic.
    print('  Carbon sequestration (gCO2/kcal)')
    total_co2_sequestration = 2e15 # gCO2
    co2_seq_per_plant_calorie = total_co2_sequestration/total_crop_calories
    co2_seq_per_animal_calorie = co2_seq_per_plant_calorie/feed_conversion_efficiency
    print('    Plant:', co2_seq_per_plant_calorie)
    print('    Animal:', co2_seq_per_animal_calorie)

    # Vertical/hydroponic farming
    # https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7516583/
    # Compares values for lettuce...biiig assumption but assume these values
    # hold across other crop types. In practice, vertical farming/hydroponics
    # grows a much smaller variety of crops (mostly leafy greens)
    # These are all (indoor hydroponic value)/(outdoor farming value)
    water_use = 20/250 # L/kg
    energy_use = 120/0.3 # kWh/kg, provided range for hydroponic is 60-180kWh/kg
    co2_emissions = 352/540 # kg/tons
    crop_yield = 41/3.9 # kg/m2
    # Note that https://www.rsis.edu.sg/rsis-publication/nts/vertical-farms-are-they-sustainable/
    # describes 3000% higher energy use for strawberries, so this energy use ratio
    # may be a very conservative estimate

    # Assuming that all energy use in vertical farming is electric,
    # and that its relative compactness means no fuel is needed
    print('\nVertical Farming:')
    print('  Electricity per calorie (kWh/kcal):', kwh_per_plant_calorie * energy_use)
    print('  Water per calorie (L/kcal):', mean_plant_water * water_use)
    print('  Land use per calorie (m2/kcal):', land_use_per_plant_calorie * (1/crop_yield))
    print('  CO2 emissions per calorie (gCO2/kcal):', co2_per_plant_calorie * co2_emissions)
