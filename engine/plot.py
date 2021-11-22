import os
import math
import json
import pandas as pd
import matplotlib.pyplot as plt
from collections import defaultdict

N_COLORS = 10
LINE_STYLES = ['solid', 'dashed', 'dashdot', 'dotted']

df = pd.read_csv('/tmp/calibration.csv')

report = json.load(open('/tmp/calibration.json'))

if not os.path.exists('/tmp/plots'):
    os.makedirs('/tmp/plots')

plots = {
    'Population (b)': ['Population (b)', 'Pop Ref (2100, bn people)'],
    'Events': ['Events'],
    'Temperature': ['Temperature'],
    'Outlook': ['Outlook'],
    'Habitability': ['Habitability'],
    'Extinction Rate': ['Extinction Rate'],
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
        plots['Process-{}-{}'.format(output, category)] = cols

files = []
for title, cols in plots.items():
    plt.title(title)
    for i, col in enumerate(cols):
        vals = df[col]
        linestyle = math.floor(i/N_COLORS)
        if title == 'Events':
            plt.scatter(df['Year'], vals, label=col, s=2)
        else:
            plt.plot(df['Year'], vals, label=col, linestyle=LINE_STYLES[linestyle])
    plt.legend(fontsize=6)
    fname = '{}.png'.format(title)
    plt.savefig(os.path.join('/tmp/plots', fname))
    plt.close()
    files.append(fname)

events = []
events_by_year = {}
for i, evs in enumerate(report.pop('events')):
    year = report['start_year'] + i
    subevs = []
    for ev, region in evs:
        ev_name = ev if region is None else '{} in {}'.format(ev, region)
        subevs.append(ev_name)
    events_by_year[year] = subevs
    events.append((
        year,
        '<br />'.join(subevs)
    ))

report['scenarios'] = ','.join(report['scenarios'])

style = '''
* {
    box-sizing: border-box;
}
body {
    margin: 0;
}
main {
    display: flex;
}
main > div {
    padding: 1em;
}
.group {
    flex: 1;
    height: 100vh;
    overflow-y:scroll;
}
.charts {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-evenly;
}
img {
    width: 480px;
}
.meta {
    text-align: center;
}
.tag {
    border: 1px solid;
    border-radius: 0.2em;
    display: inline-flex;
}
.tag > div:first-child {
    background: #333;
    color: #fff;
}
.tag > div {
    padding: 0 0.25em;
}
.events {
    width: 210px;
    height: 100vh;
    overflow-y:scroll;
}
.event {
    display: flex;
}
.event .year {
    margin-right: 0.5em;
}
.no-events .year {
    color: #bbb;
}

#tooltip {
    font-size: 0.8em;
    position: fixed;
    background: #111;
    color: #fff;
    bottom: 0;
    left: 50%;
    transform: translate(-50%, 0);
    width: 320px;
}
'''

tag = '''
<div class="tag">
    <div>{k}</div>
    <div>{v}</div>
</div>
'''

event = '''
<div class="event {cls}">
    <div class="year">{year}</div>
    <div>{events}</div>
</div>
'''

scripts = '''
<script>
const chartWidth = 640;
const leftOffset = 100/chartWidth;
const years = 100;
const startYear = 2023
const endYear = startYear + years;

const tooltip = document.getElementById('tooltip');
[...document.querySelectorAll('.charts img')].forEach((el) => {
    el.addEventListener('mousemove', (ev) => {
        let width = el.width;
        let offset = width * leftOffset;
        const yearWidth = width * 0.0070;
        let rect = ev.target.getBoundingClientRect();
        let x = ev.clientX - rect.left;
        let y = ev.clientY - rect.top;
        let year = Math.floor(startYear + (x - offset)/yearWidth);
        let year_a = Math.max(year - 2, startYear);
        let year_b = Math.min(year + 2, endYear);
        if (year_b > year_a) {
            let events = [];
            [...Array(year_b - year_a)].forEach((_,i)=> {
                let evs = EVENTS_BY_YEAR[year_a + i];
                if (evs !== undefined) {
                    events = events.concat(evs);
                }
            });
            if (events.length > 0) {
                tooltip.innerHTML = `
                    <div><b>${year_a}-${year_b}</b></div>
                    ${events.join('<br />')}
                `;
                tooltip.style.display = 'block';
            } else {
                tooltip.style.display = 'none';
            }
        } else {
            tooltip.style.display = 'none';
        }
    });
    el.addEventListener('mouseleave', () => {
        tooltip.style.display = 'none';
    });
});
</script>
'''

html = '''
<html>
<head>
    <title>Half Earth Calibration</title>
    <style>{style}</style>
</head>
<body>
<main>
    <div id="tooltip"></div>
    <div class="group">
        <div class="meta">{meta}</div>
        <div class="charts">{charts}</div>
    </div>
    <div class="events">{events}</div>
</main>
<script>
const EVENTS_BY_YEAR = {events_by_year};
</script>
{scripts}
</body>
</html>
'''.format(
        style=style,
        scripts=scripts,
        events_by_year=json.dumps(events_by_year),
        meta='\n'.join(tag.format(k=k, v=v) for k, v in report.items()),
        charts='\n'.join('<img src="{}">'.format(fname) for fname in files),
        events='\n'.join(
            event.format(
                year=year, events=evs,
                cls='no-events' if not evs else '')
            for year, evs in events))

with open('/tmp/plots/index.html', 'w') as f:
    f.write(html)