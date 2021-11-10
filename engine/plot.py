import os
import pandas as pd
import matplotlib.pyplot as plt
from collections import defaultdict

df = pd.read_csv('/tmp/calibration.csv')

if not os.path.exists('/tmp/plots'):
    os.makedirs('/tmp/plots')

plots = {
    'Population (b)': ['Population (b)', 'Pop Ref (2100, bn people)'],
    'CO2eq Emissions': [
        'CO2eq Emissions',
        'CO2eq Ref (Gt)',
    ],
    'CO2 Emissions (Gt)': [
        'CO2 Emissions (Gt)',
        'Energy CO2 Emissions (Gt)',
        'Calorie CO2 Emissions (Gt)',
        'Industry CO2 Emissions (Gt)',
        'CO2 Ref (Gt)',
    ],
    'CH4 Emissions (Mt)': [
        'CH4 Emissions (Mt)',
        'Energy CH4 Emissions (Mt)',
        'Calorie CH4 Emissions (Mt)',
        'Industry CH4 Emissions (Mt)',
        'CH4 Ref (Mt)',
    ],
    'N2O Emissions (Mt)': [
        'N2O Emissions (Mt)',
        'Energy N2O Emissions (Mt)',
        'Calorie N2O Emissions (Mt)',
        'Industry N2O Emissions (Mt)',
        'N2O Ref (Mt)',
    ],
    'Fuel (TWh)': [
        'Industry Fuel Demand (TWh)',
        'Agg Fuel Demand (TWh)',
        'Produced Fuel (TWh)',
        'Fuel Ref (TWh)',
    ],
    'Electricity (TWh)': [
        'Industry Elec Demand (TWh)',
        'Agg Elec Demand (TWh)',
        'Produced Elec (TWh)',
        'Elec Ref (TWh)',
    ],
    'Animal Calories (Tcals)': [
        'Base Animal Cal Demand (Tcals)',
        'Agg Animal Cal Demand (Tcals)',
        'Produced Animal Cals (Tcals)',
    ],
    'Plant Calories (Tcals)': [
       'Base Plant Cal Demand (Tcals)',
       'Agg Plant Cal Demand (Tcals)',
       'Produced Plant Cals (Tcals)',
    ],
    'Demand & Consumed': [
       'Produced Fuel (% Demand)',
       'Produced Elec (% Demand)',
       'Produced Animal Cals (% Demand)',
       'Produced Plant Cals (% Demand)',
       'Consumed Water (%)',
       'Consumed Land (%)',
    ],
    'Water': [
       'Energy Water Req. (km3)',
       'Calorie Water Req. (km3)',
       'Industry Water Demand (km3)',
       'Water Ref (km3)',
    ],
    'Land': [
       'Energy Land Req. (km2)',
       'Calorie Land Req. (km2)',
       'Cals Land Ref (km2)',
    ],
    'Cal per Capita per Day': [
        'Cal/Capita/Day',
        'Cals Ref (kcal/person/day)'
    ],
    'Mean Income Level': [
        'Mean Income Level'
    ],
}

outputs = ['Electricity', 'Fuel', 'PlantCalories', 'AnimalCalories']
process_cols_by_output = defaultdict(lambda: defaultdict(list))
for col in df.columns:
    for o in outputs:
        if col.startswith('{}:'.format(o)):
            _, process, category = col.split(':')
            process_cols_by_output[o][category].append(col)
for output, categories in process_cols_by_output.items():
    for category, cols in categories.items():
        plots['Process:{}:{}'.format(output, category)] = cols

for title, cols in plots.items():
    plt.title(title)
    for col in cols:
        vals = df[col]
        plt.plot(df['Year'], vals, label=col)
    plt.legend(fontsize=6)
    plt.savefig('/tmp/plots/{}.png'.format(title))
    plt.close()