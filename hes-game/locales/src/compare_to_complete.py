# Compare translation to a complete translation
# Use PT-BR as the example of a complete translation

import pandas as pd
lang = pd.read_csv('es.csv')
lang = lang[['English', 'Translation']].set_index('English').to_dict()
lang = lang['Translation']
complete = pd.read_csv('pt-br.csv')
complete = complete[['English', 'Translation']].set_index('English').to_dict()
complete = complete['Translation']

print('MISSING KEYS:')
for k in complete.keys():
    if k not in lang:
        print(k)
# import ipdb; ipdb.set_trace()