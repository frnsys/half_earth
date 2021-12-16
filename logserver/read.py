from db import Database
from datetime import datetime
import matplotlib.pyplot as plt

def gtco2eq(byproducts):
  co2eq = byproducts['co2_emissions'] + byproducts['ch4_emissions'] * 36 + byproducts['n2o_emissions'] * 298
  return co2eq * 1e-15

db = Database('logs.db')

print('Recent Sessions:')
print('-'*50)
sessions = db.sessions()
sessions.reverse()
for session in sessions[:5]:
    print(session['id'])
    print(' ', datetime.fromtimestamp(float(session['timestamp'])))
    print(' ', session['version'])
    print(' ', session['useragent'])
    # snapshots = db.snapshots(session['id'])
    # if snapshots:
    #     emissions = gtco2eq(snapshots[-1]['snapshot']['gameState']['world'])
    #     if emissions > 200:
    #         print(session['id'])
    #         print('  Emissions:', emissions)
    #         electricity_demand = [snapshots[14]['snapshot']['gameState']['output_demand']['electricity'] * 1e-9 for s in snapshots]
    #         x = list(range(len(snapshots)))
    #         plt.plot(x, electricity_demand)

    #         before_projects = [p for p in snapshots[12]['snapshot']['gameState']['projects'] if p['status'] == 'Active' or p['status'] == 'Finished']
    #         after_projects = [p for p in snapshots[13]['snapshot']['gameState']['projects'] if p['status'] == 'Active' or p['status'] == 'Finished']

    #         processes = [(p['name'], p['mix_share']) for p in snapshots[14]['snapshot']['gameState']['processes']]
    #         import ipdb; ipdb.set_trace()