"""
Data from: https://ourworldindata.org/grapher/agricultural-land
Agricultural land use dominates overall land use so just using that
"""

import json
import pandas as pd

df = pd.read_csv('src/agricultural-land.csv')
land_use = df.groupby('Entity').get_group('World').set_index('Year')['agricultural_land']
change = land_use.pct_change().mean()

# 1990-2018
years = list(range(1990, 2018+1))
vals = land_use.loc[years]
vals *= 1e4 # From ha to m2
land_use = list(zip(years, vals))
land_use.append((2019, land_use[-1][1] * 1 + change))
land_use.append((2020, land_use[-1][1] * 1 + change))
land_use.append((2021, land_use[-1][1] * 1 + change))
years, land_use = zip(*land_use)

print(land_use)
with open('out/land_use.json', 'w') as f:
    json.dump(land_use, f)