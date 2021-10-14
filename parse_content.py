import re
import json
import textwrap
from collections import defaultdict

ids = {}
flags = {}
rust_output = []

consts_template = '''// Do not edit this file. Use `parse_content.py` to regenerate it.
use crate::regions::Income;
use crate::kinds::{FeedstockMap, OutputMap};
'''

content_template = '''// Do not edit this file. Use `parse_content.py` to regenerate it.
use crate::world::World;
use crate::industries::Industry;
use crate::regions::{Region, Income};
use crate::projects::{Project, Outcome};
use crate::production::{Process, ProcessFeature};
use crate::kinds::{Resource, Output, Feedstock, ByproductMap, ResourceMap};
use crate::events::{Event, Choice, Effect, Probability, Likelihood, Condition, Comparator, Flag, WorldVariable, LocalVariable, PlayerVariable};
use crate::projects::Status as ProjectStatus;
'''

# Required keys and defaults
# `None` means there is no default
specs = {
    'World': {
        'year': 0,
        'extinction_rate': 0.,
        'temperature': 0.,
    },
        # These are all computed later
        # 'population', 'contentedness', 'health', 'outlook', 'sea_level_rise', 'water_stress', 'precipitation'],
    'Region': {
        'id': None,
        'name': None,
        'income_level': None,
        'health': 100,
        'outlook': 100,
        'population': None,
        'base_habitability': 100,
        'base_contentedness': 0,
        'seceded': 'false',
    },
    'Industry': {
        'name': None,
        'resources': {},
        'byproducts': {},
    },
    'Process': {
        'id': None,
        'name': None,
        'output': None,
        'mix_share': 0,
        'feedstock': None,
        'resources': {},
        'byproducts': {},
        'locked': 'false',
        'banned': 'false',
        'features': {},
        'output_modifier': 1.0,
    },
    'Project': {
        'id': None,
        'name': None,
        'years': 10,
        'effects': [],
        'locked': 'false',
        'status': 'ProjectStatus::Inactive',
        'ongoing': 'false',
        'outcomes': [],
    },
    'Event': {
        'id': None,
        'name': None,
        'locked': 'false',
        'local': 'false',
        'repeats': 'false',
        'effects': [],
        'probabilities': [],
        'choices': [],
    },
    'Probability': {
        'likelihood': None,
        'conditions': [],
    },
    'Choice': {
        'effects': [],
        'conditions': [],
    },
    'Outcome': {
        'effects': [],
        'probability': [],
    },
    'FeedstockMap': {
        'oil': 0.,
        'coal': 0.,
        'uranium': 0.,
        'lithium': 0.,
        'natural_gas': 0.,
        'soil': 0.,
        'other': 0.,
    },
}
valid_resources = ['Land', 'Water', 'Fuel', 'Electricity']
valid_byproducts = ['CO2', 'CH4', 'N2O', 'Biodiveristy']
valid_outputs = ['Electricity', 'Fuel', 'AnimalCalories', 'PlantCalories']
incomes = ['low_income', 'lower_middle_income', 'upper_middle_income', 'high_income']

def param(e, k):
    return float(e['params'].get(k) or 0)

def value(e):
    return float(e.get('value') or 0)

def camel_to_snake(v):
    return re.sub(r'(?<!^)(?=[A-Z])', '_', v).lower()

effects = {
    'UnlocksProject':   lambda e: (ids[e['entity']],),
    'UnlocksProcess':   lambda e: (ids[e['entity']],),
    'AddEvent':         lambda e: (ids[e['entity']],),
    'TriggerEvent':     lambda e: (ids[e['entity']], e['params']['Delay (months)']),
    'LocalVariable':    lambda e: ('LocalVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'WorldVariable':    lambda e: ('WorldVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'PlayerVariable':   lambda e: ('PlayerVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'Demand':           lambda e: ('Output::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'Output':           lambda e: ('Output::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'OutputForFeature': lambda e: ('ProcessFeature::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'Resource':         lambda e: ('Resource::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'Feedstock':        lambda e: ('Feedstock::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'SetFlag':          lambda e: ('Flag::{}'.format(flags[e['entity']]),),
    'SetProjectStatus': lambda e: (ids[e['entity']], 'ProjectStatus::{}'.format(e['subtype']),),
    'RegionLeave':      lambda _: (),
    'Migration':        lambda _: (),
}
comps = {
    '<': 'Comparator::Less',
    '<=': 'Comparator::LessEqual',
    '==': 'Comparator::Equal',
    '!=': 'Comparator::NotEqual',
    '>=': 'Comparator::GreaterEqual',
    '>': 'Comparator::Greater',
}
conds = {
    'LocalVariable':    lambda e: ('LocalVariable::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'WorldVariable':    lambda e: ('WorldVariable::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'Demand':           lambda e: ('Output::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'Output':           lambda e: ('Output::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'OutputDemandGap':  lambda e: ('Output::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'Resource':         lambda e: ('Resource::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'ResourceDemandGap':lambda e: ('Resource::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'Feedstock':        lambda e: ('Feedstock::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'ProcessMixShare':  lambda e: (ids[e['entity']], comps[e['comparator']], value(e)),
    'ProcessMixShareFeature': lambda e: ('ProcessFeature::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'ProjectActive':    lambda e: (ids[e['entity']], 'ProjectStatus::Active'),
    'ProjectInactive':    lambda e: (ids[e['entity']], 'ProjectStatus::Inactive'),
    'ProjectFinished':    lambda e: (ids[e['entity']], 'ProjectStatus::Finished'),
    'ProjectStalled':    lambda e: (ids[e['entity']], 'ProjectStatus::Stalled'),
    'ProjectHalted':    lambda e: (ids[e['entity']], 'ProjectStatus::Halted'),
    'RunsPlayed':    lambda e: (comps[e['comparator']], e['value']),
    'Flag':          lambda e: ('Flag::{}'.format(flags[e['entity']]),),
}

def define_effect(effect):
    effect_params = [
            '0.' if isinstance(v, float) and v == 0 else str(v)
            for v in effects[effect['type']](effect)]
    if not effect_params:
        return 'Effect::{}'.format(effect['type'])
    else:
        return 'Effect::{}({})'.format(
                effect['type'], ', '.join(effect_params))

def define_probability(prob):
    typ = 'Likelihood::{}'.format(prob['type'])
    return define_struct('Probability', {
        'likelihood': typ,
        'conditions': prob['conditions'],
    })

def define_choice(choice):
    return define_struct('Choice', {
        # Don't include text, it will be fetched
        # when the event occurs
        # 'text': choice['text'],
        'effects': choice['effects'],
        'conditions': choice.get('conditions', []),
    })

def define_outcome(outcome):
    return define_struct('Outcome', {
        # Don't include text, it will be fetched
        # when the outcome occurs
        # 'text': outcome['text'],
        'effects': outcome['effects'],
        'probability': outcome['probability']
    })

def define_condition(cond):
    cond_params = [
            '0.' if isinstance(v, float) and v == 0 else str(v)
            for v in conds[cond['type']](cond)]

    cond_type = cond['type']
    if 'Project' in cond_type:
        cond_type = 'ProjectStatus'
    return 'Condition::{}({})'.format(
            cond_type, ', '.join(cond_params))

def define_field(k, v, item):
    if k == 'income_level':
        return 'income: Income::{}'.format(v.replace('-', ''))
    if k == 'year':
        return 'year: {}'.format(v)
    elif k == 'name' or k == 'text':
        v = '"{}"'.format(v)
    elif k == 'output':
        return 'output: Output::{}'.format(v)
    elif k == 'feedstock':
        amount = item.get('feedstock_amount', 0)
        return 'feedstock: (Feedstock::{}, {})'.format(v, format_float(amount))
    elif k == 'byproducts':
        fields = filter(lambda x: x[0] in valid_byproducts, v.items())
        return 'byproducts: byproducts!(\n{}\n)'.format(
                    indent(define_fields(fields, item)))
    elif k == 'resources':
        fields = filter(lambda x: x[0] in valid_resources, v.items())
        return 'resources: resources!(\n{}\n)'.format(
                    indent(define_fields(fields, item)))
    elif k == 'effects':
        return 'effects: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_effect(e) for e in v)))
    elif k == 'probability':
        return 'probability: {}'.format(define_probability(v))
    elif k == 'probabilities':
        return 'probabilities: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_probability(e) for e in v)))
    elif k == 'conditions':
        return 'conditions: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_condition(e) for e in v)))
    elif k == 'choices':
        choices = [c for c in v if c['text']]
        return 'choices: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_choice(c) for c in choices)))
    elif k == 'outcomes':
        outcomes = [c for c in v if c.get('text')]
        return 'outcomes: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_outcome(o) for o in outcomes)))
    elif k == 'years':
        return 'years: {}'.format(v)
    elif k == 'mix_share':
        return 'mix_share: {}'.format(v/100)
    elif k == 'features':
        feats = ['ProcessFeature::{}'.format(feat) for feat, on in v.items() if on]
        return 'features: vec![\n{}\n]'.format(
                    indent(',\n'.join(feats)))
    elif is_float(v) and k != 'id':
        v = format_float(v)
    elif isinstance(v, bool):
        v = 'true' if v else 'false'
    return '{}: {}'.format(k.lower(), v)

def define_fields(fields, item):
    fields = [define_field(k, v, item) for k, v in fields]
    return ',\n'.join(fields)

def define_struct(typ, data):
    fields = [(k, data[k] if k in data else default)
            for k, default in specs[typ].items()]
    try:
        return '''{} {{\n{}\n}}'''.format(typ, indent(define_fields(fields, data)))
    except Exception as e:
        print('Error defining struct of type "{}":'.format(typ), data)
        print(e)
        raise

def define_structs(typ, items):
    structs = []
    for i, item in enumerate(items):
        item['id'] = i
        structs.append(define_struct(typ, item))
    return ',\n'.join(structs)

def define_array(name, typ):
    items = items_by_type[typ]
    return 'pub const {}: [{}; {}] = [\n{}\n];'.format(name, typ,
        len(items), indent(define_structs(typ, items)))

def format_float(num):
    num = float(num)
    if round(num) == num:
        return '{:.1f}'.format(num)
    else:
        return str(num)

def is_float(num):
    if isinstance(num, bool):
        return False
    try:
        float(num)
        return True
    except ValueError:
        return False

def define_const(const):
    if const['type'] == 'float':
        return 'const {}: f32 = {};'.format(
                const['name'].upper(),
                format_float(const['value']))
    elif const['type'] == 'float list':
        vals = const['value'].split('\n')
        return 'const {}: [f32; {}] = [\n{}\n];'.format(
            const['name'].upper(), len(vals), indent(',\n'.join(vals)))
    else:
        raise Exception('Unrecognized const type:', const['type'])

def define_content_fn(name, typ):
    return 'pub fn {}() -> Vec<{}> {{\n{}\n}}'.format(
        name, typ, indent('vec![\n{}\n]'.format(
            indent(define_structs(typ, items_by_type[typ])))))

def indent(text, levels=1):
    return textwrap.indent(text, 4 * levels * ' ')


if __name__ == '__main__':
    # Parse items into groups and so on
    items = json.load(open('content.json'))
    items_by_type = defaultdict(list)
    for id, item in items.items():
        if item.get('deleted'): continue
        typ = item['_type']
        id_ = len(items_by_type[typ])
        ids[id] = id_
        items_by_type[typ].append(item)
        if typ == 'Flag':
            flags[id] = item['name']

    # Define constants
    rust_output = [consts_template]
    const_defs = []
    feedstock_reserves = {}
    income_level_consts = defaultdict(dict)
    for const in items_by_type['Const']:
        if 'name' not in const: continue

        if 'income' in const['name']:
            key, income_level = const['name'].split('__')
            income_level_consts[key][income_level] = const['value']
            continue
        if 'reserves' in const['name']:
            feedstock, _ = const['name'].split('_')
            feedstock_reserves[feedstock] = const['value']
            continue

        const_def = define_const(const)
        const_defs.append(const_def)

    rust_output.append(
        'const FEEDSTOCK_RESERVES: FeedstockMap<f32> = {};'.format(
            define_struct('FeedstockMap', feedstock_reserves)))

    # Population change function
    income_levels = income_level_consts.pop('pop_change_coefs')
    income_arms = []
    variants = {
        'low_income': 'Low',
        'lower_middle_income': 'LowerMiddle',
        'upper_middle_income': 'UpperMiddle',
        'high_income': 'High',
    }
    for k in incomes:
        coefs = income_levels[k].split('\n')
        income_arms.append('Income::{} => {} + {}*pop + {}*pop.powf(2.0) + {}*pop.powf(3.0)'.format(
            variants[k],
            *coefs
        ))
    fn_def = ('pub fn income_pop_change(pop: f32, income: &Income) -> f32 {{\n    match income {{\n{}\n    }}\n}}'.format(
        indent(',\n'.join(income_arms), levels=2)))
    rust_output.append(fn_def)

    fields = {}
    for k in valid_outputs:
        vals = income_level_consts.pop(camel_to_snake(k))
        sub_arr = [vals[k] for k in incomes]
        fields[camel_to_snake(k)] = '[\n{}\n]'.format(indent(',\n'.join(sub_arr)))
    output_demand = 'pub const OUTPUT_DEMAND: OutputMap<[f32; 4]> = OutputMap {{\n{}\n}};'.format(
            indent(define_fields(fields.items(), None)))
    rust_output.append(output_demand)

    for key, income_levels in income_level_consts.items():
        const = {
            'name': '{}_BY_INCOME'.format(key),
            'type': 'float list',
            'value': '\n'.join([
                income_levels['low_income'],
                income_levels['lower_middle_income'],
                income_levels['upper_middle_income'],
                income_levels['high_income'],
            ])
        }
        const_def = define_const(const)
        const_defs.append(const_def)

    rust_output.append('\n'.join(const_defs))

    with open('engine/src/consts.rs', 'w') as f:
        f.write('\n\n'.join(rust_output))

    # Define content functions
    rust_output = [content_template]
    rust_output.append(define_array('WORLDS', 'World'))
    rust_output.append(define_content_fn('regions', 'Region'))
    rust_output.append(define_content_fn('industries', 'Industry'))
    rust_output.append(define_content_fn('processes', 'Process'))
    rust_output.append(define_content_fn('projects', 'Project'))
    rust_output.append(define_content_fn('events', 'Event'))
    with open('engine/src/content.rs', 'w') as f:
        f.write('\n\n'.join(rust_output))