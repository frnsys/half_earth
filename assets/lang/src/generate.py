import os
import math
import json
import pandas as pd
from glob import glob

for csv in glob('*.csv'):
    lang = csv.replace('.csv', '')
    trans = {}
    df = pd.read_csv(csv)
    for i, row in df.iterrows():
        # Skip section headers
        if row.get('Section') == 'Y': continue

        en = row['English']
        tr = row['Translation']

        if not isinstance(en, str):
            continue
        if not isinstance(tr, str):
            continue

        trans[en] = tr
    with open('../{}.json'.format(lang), 'w') as f:
        json.dump(trans, f, indent=2)
