import os
import math
import json
import pandas as pd
import matplotlib.pyplot as plt
from collections import defaultdict

print('Plotting...')

N_COLORS = 10
LINE_STYLES = ['solid', 'dashed', 'dashdot', 'dotted']

df = pd.read_csv('/tmp/calibration.csv')

report = json.load(open('/tmp/calibration.json'))

if not os.path.exists('/tmp/plots'):
    os.makedirs('/tmp/plots')

ranges = {
    'Temperature': {
        'min': 0,
        'max': 5
    },
    'Habitability': {
        'min': 0,
        'max': 15
    },
    'World Outlook': {
        'min': 0,
        'max': 50
    },
    'CO2 Emissions (Gt)': {
        'min': 0,
    },
    'CO2eq Emissions': {
        'min': 0,
    },
    'Fuel (TWh)': {
        'min': 0,
    },
    'Electricity (TWh)': {
        'min': 0,
    },
    'Land': {
        'min': 0,
    },
    'Extinction Rate': {
        'min': 0,
        'max': 100
    },
    'Cal per Capita per Day': {
        'min': 0,
    }
}

groups = {
    'General': [
        'Population (b)', 'Temperature', 'Habitability',
        'Extinction Rate', 'Mean Income Level'],
    'Emissions': [
        'CO2eq Emissions', 'CO2 Emissions (Gt)',
        'CH4 Emissions (Mt)', 'N2O Emissions (Mt)'],
    'Production': [
        'Demand & Consumed', 'Cal per Capita per Day',
        'Fuel (TWh)', 'Electricity (TWh)',
        'Animal Calories (Tcals)', 'Plant Calories (Tcals)',
        'Water', 'Land',
    ],
    'Electricity': ['Electricity (TWh)'],
    'Fuel': ['Fuel (TWh)'],
    'PlantCalories': ['Plant Calories (Tcals)'],
    'AnimalCalories': ['Animal Calories (Tcals)'],
    'Outlook': ['World Outlook'],
    'Events': ['Events', ],
}

plots = {
    'Population (b)': ['Population (b)', 'Pop Ref (2100, bn people)'],
    'Events': ['Events'],
    'Temperature': ['Temperature'],
    'World Outlook': ['World Outlook'],
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

icon_event_groups = {
    'Flooding': [
        'Flooding', 'Severe Flooding', 'Extreme Flooding',
    ],
    'Storms': [
        'Severe Hurricane',
        'Large Derecho Storm'
    ],
    'Wildfires': [
        'Wildfires', 'Severe Wildfires',
    ],
    'Social Unrest': ['Protests', 'Riots', 'Revolts'],
    'Heatwaves': ['Heatwaves'],
    'Crop Failures': ['Crop Failures'],
    'Disease Outbreak': ['Disease Outbreak'],
    'Attacks': ['Doom Cult Attacks'],
}

region_groups = {
    'Asia': [
        'Central Asia',
        'Eastern Asia',
        'South-eastern Asia',
        'Southern Asia',
        'Western Asia',
    ],
    'Africa': [
        'Eastern Africa',
        'Central Africa',
        'Northern Africa',
        'Southern Africa',
        'Southern Asia',
    ],
    'Europe & Neo-Europe': [
        'Eastern Europe',
        'Northern Europe',
        'Southern Europe',
        'Western Europe',
        'Northern America',
        'Australasia',
    ],
    'Americas & Islands': [
        'Central America',
        'Southern America',
        'Caribbean',
        'Oceania',
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
        name = 'Process-{}-{}'.format(output, category)
        plots[name] = cols
        groups[output].append(name)
        if category == 'Mix Share':
            ranges[name] = {'min': 0, 'max': 1}

all_icon_events = set()
icon_event_history = report.pop('icon_events')
for evs in icon_event_history:
    for name, _ in evs:
        all_icon_events.add(name)

icon_events_by_year = {}
icon_events_by_year_output = {}
for i, icon_events in enumerate(icon_event_history):
    year = report['start_year'] + i
    counts = defaultdict(int)
    for ev, region in icon_events:
        counts[ev] += 1
        all_icon_events.add(ev)

    evs = []
    for name, count in counts.items():
        evs.append('{}x {}'.format(count, name))
    icon_events_by_year_output[year] = '<br />'.join(evs)
    icon_events_by_year[year] = {k: counts[k] for k in all_icon_events}
icon_events_df = pd.DataFrame.from_dict(icon_events_by_year, orient='index')
df = df.set_index('Year').join(icon_events_df)

for title, cols in icon_event_groups.items():
    plots[title] = cols
    groups['Events'].append(title)

for title, cols in region_groups.items():
    plots[title] = ['Outlook:{}'.format(col) for col in cols]
    groups['Outlook'].append(title)
    ranges[title] = {
        'min': 0,
        'max': 20
    }

files = {}
for group, titles in groups.items():
    files[group] = []
    for title in titles:
        cols = plots[title]
        plt.title(title)
        plt.margins(0)
        plt.xlim(report['start_year'], report['start_year']+100)

        for i, col in enumerate(cols):
            try:
                vals = df[col]
            except KeyError:
                print('Missing column:', col)
                continue
            linestyle = math.floor(i/N_COLORS)
            if title == 'Events':
                plt.scatter(df.index, vals, label=col, s=2)
            else:
                plt.plot(df.index, vals, label=col, linestyle=LINE_STYLES[linestyle])
        plt.legend(fontsize=6)

        rng = ranges.get(title, {})
        plt.ylim(bottom=rng.get('min'), top=rng.get('max'))

        ax = plt.gca()
        n_years = 100
        ax_width = ax.get_window_extent().width
        year_width = ax_width/n_years
        fig = plt.gcf()
        fig_width, fig_height = fig.get_size_inches() * fig.dpi
        ax_fig_width_ratio = ax_width/fig_width
        # print(ax_fig_width_ratio)
        # import ipdb; ipdb.set_trace()

        fname = '{}.png'.format(title)
        plt.savefig(os.path.join('/tmp/plots', fname))

        plt.close()
        files[group].append(fname)


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
.chart-group {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-evenly;
    display: none;
}
img {
    width: 480px;
}
.meta {
    text-align: center;
    position: sticky;
    top: 0;
    z-index: 10;
}
.tag {
    border: 1px solid;
    border-radius: 0.2em;
    display: inline-flex;
    background: #fff;
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
.icon-events {
    font-size: 0.7em;
    color: #777;
}

.line {
    top: 0;
    height: 100%;
    display: flex;
    font-size: 0.9em;
    border-left: 1px solid #000;
    position: absolute;
    flex-direction: column;
    justify-content: space-around;
    pointer-events: none;
    padding-left: 0.25em;
    display: none;
}

.chart-group-tabs {
    display: flex;
    justify-content: space-evenly;
    margin: 0.5em 0;
}
.chart-group-tabs > div {
    border: 1px solid;
    border-radius: 0.2em;
    padding: 0 0.2em;
    cursor: pointer;
    background: #fff;
}
.chart-group-tabs > div.selected {
    background: #333;
    color: #fff;
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
    <div>
        <div>{events}</div>
        <div class="icon-events">{icon_events}</div>
    </div>
</div>
'''

scripts = '''
<script>
const chartGroups = [...document.querySelectorAll('.chart-group')];
const chartTabs = [...document.querySelectorAll('.chart-group-tabs > div')];
chartTabs.forEach((el, i) => {
    el.addEventListener('click', () => {
        let sel = document.querySelector('.chart-group-tabs .selected');
        if (sel) sel.classList.remove('selected');
        el.classList.add('selected');
        chartGroups.forEach((g, j) => {
            if (i == j) {
                g.style.display = 'flex';
            } else {
                g.style.display = 'none';
            }
        });
    });
});
chartTabs[0].click();

[...document.querySelectorAll('.charts img')].forEach((el) => {
    el.parentElement.style.position = 'relative';

    let lineEl = document.createElement('div');
    lineEl.classList.add('line');
    el.parentElement.appendChild(lineEl);

    el.addEventListener('mousemove', (ev) => {
        let width = el.width;
        let axes_width = el.width * 0.775;
        lineEl.style.display = 'flex';
        let rect = ev.target.getBoundingClientRect();
        let x = ev.clientX - rect.left - (el.width * 0.125);
        let y = ev.clientY - rect.top;
        let i = x/axes_width;
        let year = 2022 + Math.floor(i*100);
        let events = EVENTS_BY_YEAR[year] || [];

        lineEl.style.left = `${x + (el.width * 0.125)}px`;
        lineEl.innerHTML = `<div>
            <u>${year}</u><br />
            ${events.join('<br />')}
        </div>`;
    });
    el.addEventListener('mouseleave', () => {
        lineEl.style.display = 'none';
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
    <div class="group">
        <div class="meta">
            <div>{meta}</div>
            <div class="chart-group-tabs">{group_tabs}</div>
        </div>
        <div class="charts">{chart_groups}</div>
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
        group_tabs='\n'.join('<div>{}</div>'.format(g) for g in groups.keys()),
        chart_groups='\n'.join(
            '<div class="chart-group">{}</div>'.format('\n'.join('<div><img src="{}"></div>'.format(fname) for fname in fnames))
            for fnames in files.values()),
        events='\n'.join(
            event.format(
                year=year, events=evs,
                icon_events=icon_events_by_year_output[year],
                cls='no-events' if not evs else '')
            for year, evs in events))

with open('/tmp/plots/index.html', 'w') as f:
    f.write(html)