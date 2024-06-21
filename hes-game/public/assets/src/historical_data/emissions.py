"""
Using historical data from <https://data.worldbank.org/indicator/EN.ATM.GHGT.KT.CE> (`API_EN.ATM.GHGT.KT.CE_DS2_en_csv_v2_3159354.csv`)
and growing by 1% per year. Was not able to find total GHG emissions for 2019 and 2020 but there is a CO2 emission dip in 2020 because of COVID-19 (looks like a roughly 5% dip), so we can manually add that in.
"""

import json
import pandas as pd

# 1990-2018
years = list(range(1990, 2018+1))
df = pd.read_csv('src/API_EN.ATM.GHGT.KT.CE_DS2_en_csv_v2_3159354.csv', index_col='Country Name')

cols = [str(y) for y in years]
vals = df[cols].loc['World'].values
vals *= 1e-6 # from kt to Gt
emissions = list(zip(years, vals))
emissions.append((2019, emissions[-1][1] * 1.01))
emissions.append((2020, emissions[-1][1] * (1.01 - 0.05)))
emissions.append((2021, emissions[-1][1] * 1.02))
years, emissions = zip(*emissions)

print(emissions)

with open('out/emissions.json', 'w') as f:
    json.dump(emissions, f)