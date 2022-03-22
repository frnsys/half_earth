import sys
import json
import click
from db import Database
from datetime import datetime

def gtco2eq(byproducts):
  co2eq = byproducts['co2_emissions'] + byproducts['ch4_emissions'] * 36 + byproducts['n2o_emissions'] * 298
  return co2eq * 1e-15

def pprint(obj, indent=0, prefix=''):
    if isinstance(obj, list):
        for o in obj:
            pprint(o, indent=indent+1, prefix='- ')
    elif isinstance(obj, dict):
        for i, (k, v) in enumerate(obj.items()):
            if isinstance(v, dict):
                pprint(v, indent=indent+1)
            else:
                print('{}{}{}: {}'.format(
                    prefix if i == 0 else '',
                    '  '*(indent-1 if i == 0 else indent),
                    k, v))

def emissions(snapshots):
    return [{
        'year': s['snapshot']['gameState']['world']['year'],
        'emissions': gtco2eq(s['snapshot']['gameState']['world'])
    } for s in snapshots]

def process_mix(snapshots):
    return [{
        'name': p['name'],
        'mix_share': p['mix_share']
    } for p in snapshots[-1]['snapshot']['gameState']['processes']]

def electricity_demand(snapshots):
    return [{
        'year': s['snapshot']['gameState']['world']['year'],
        'demand': s['snapshot']['gameState']['output_demand']['electricity'] * 1e-9
    } for s in snapshots]

def active_projects(snapshots):
    projects = snapshots[-1]['snapshot']['gameState']['projects']
    return [p for p in projects if p['status'] != 'Inactive']

def project_timeline(snapshots):
    timeline = []
    last_projects = {}
    for i, s in enumerate(snapshots):
        year = s['snapshot']['gameState']['world']['year']
        projects = s['snapshot']['gameState']['projects']
        timeline.append({
            'year': year
        })
        for p in projects:
            if p['status'] != 'Inactive':
                last = last_projects.get(p['name'])
                status = (p['status'], p['points'], p['level'])
                if last is None or last != status:
                    timeline[-1][p['name']] = status
                last_projects[p['name']] = status
    return timeline

def process_timeline(snapshots):
    timeline = []
    last_processes = {}
    for i, s in enumerate(snapshots):
        year = s['snapshot']['gameState']['world']['year']
        processes = s['snapshot']['gameState']['processes']
        timeline.append({
            'year': year
        })
        for p in processes:
            last = last_processes.get(p['name'])
            if last is None or p['mix_share'] != last:
                timeline[-1][p['name']] = p['mix_share']
            last_processes[p['name']] = p['mix_share']
    return timeline

def events(snapshots):
    return [{
        'year': s['snapshot']['gameState']['world']['year'],
        'events': s['snapshot']['events']
    } for s in snapshots]

def save_playback_script(snapshots, path):
    script = {
        'processes': [],
        'projects': [],
        'events': [],
    }
    for mix in process_timeline(snapshots):
        del mix['year']
        script['processes'].append(mix)
    for changes in project_timeline(snapshots):
        del changes['year']
        script['projects'].append({name: vals for name, vals in changes.items()})
    for year in events(snapshots):
        print(year['events'])
        script['events'].append(year['events'])
    with open(path, 'w') as f:
        json.dump(script, f)

@click.command()
@click.option('--id', default=None, help='Target session ID')
@click.option('--script_path', default=None, help='Path to save playback script to')
@click.option('--date', default=None, help='Target date',
        type=click.DateTime(formats=['%m/%d']))
def main(id, script_path, date):
    db = Database('logs.db')

    sessions = db.sessions()
    sessions.reverse()

    # Browsing
    if id is None and date is None:
        sessions = sessions[:100]

    for session in sessions:
        dt = datetime.fromtimestamp(float(session['timestamp']))

        # Browsing
        if id is None:
            display = date is None or (dt.month == date.month and dt.day == date.day)
            if display:
                print(session['id'])
                print(' ', dt)
                print('  Version:', session['version'])
                print('  User-Agent:', session['useragent'])
                print('  Snapshots:', len(db.snapshots(session['id'])))

        if session['id'] == id:
            snapshots = db.snapshots(session['id'])
            if script_path:
                save_playback_script(snapshots, script_path)
            else:
                import ipdb; ipdb.set_trace()

if __name__ == '__main__':
    main()