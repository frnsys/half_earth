import json
import textwrap
from collections import defaultdict

ids = {}
flags = {}
rust_output = []
imports = [
    'crate::effects::Effect',
    'crate::projects::Project',
    'crate::industries::Industry',
    'crate::events::{Event, Choice}',
    'crate::regions::{Region, Income}',
    'crate::probability::{Probability, Likelihood}',
    'crate::condition::{Condition, Comparator, Flag}',
    'crate::production::{Process, ProcessFeature, Feedstock}',
    'crate::kinds::{Resource, Output, ByproductMap, ResourceMap}',
    'crate::variables::{WorldVariable, LocalVariable, PlayerVariable}',
    'crate::projects::Status as ProjectStatus',
]
specs = {
    'Region': ['name', 'income_level', 'health', 'outlook', 'population', 'base_habitability'],
    'Industry': ['name', 'resources', 'byproducts'],
    'Process': ['id', 'name', 'output', 'mix_share', 'feedstock', 'resources', 'byproducts', 'locked', 'banned'],
    'Project': ['id', 'name', 'years', 'effects', 'locked', 'status', 'ongoing', 'resources', 'byproducts'], # TODO outcomes
    'Event': ['id', 'name', 'locked', 'local', 'repeats', 'effects', 'probabilities', 'choices'],
    'Probability': ['likelihood', 'conditions'],
    'Choice': ['effects', 'conditions']
}
valid_resources = ['Land', 'Water', 'Fuel', 'Electricity']
valid_byproducts = ['CO2', 'CH4', 'N2O', 'Biodiveristy']
defaults = {
    'locked': 'false',
    'local': 'false',
    'repeats': 'false',
    'effects': [],
    'probabilities': [],
    'conditions': [],
    'choices': [],
    'resources': {},
    'byproducts': {},
    'mix_share': 0,
    'banned': 'false',
    'ongoing': 'false',
    'status': 'ProjectStatus::Inactive',
    'base_habitability': 100,

    # These will trigger type errors
    # in Rust, so we'll be notified
    # they need to be set
    'name': 0,
    # 'years': '',
    'years': 10,
    'feedstock_amount': 1,
}

def param(e, k):
    return float(e['params'].get(k) or 0)

def value(e):
    return float(e.get('value') or 0)

effects = {
    'UnlocksProject':   lambda e: (ids[e['entity']],),
    'UnlocksProcess':   lambda e: (ids[e['entity']],),
    'AddEvent':         lambda e: (ids[e['entity']],),
    'TriggerEvent':     lambda e: (ids[e['entity']], e['params']['Delay (months)']),
    'LocalVariable':    lambda e: ('LocalVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'WorldVariable':    lambda e: ('WorldVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'PlayerVariable':   lambda e: ('PlayerVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'Demand':           lambda e: ('Output::{}'.format(e['subtype']), param(e, 'PercentChange')),
    'Output':           lambda e: ('Output::{}'.format(e['subtype']), param(e, 'PercentChange')),
    'OutputForFeature': lambda e: ('ProcessFeature::{}'.format(e['subtype']), param(e, 'PercentChange')),
    'Resource':         lambda e: ('Resource::{}'.format(e['subtype']), param(e, 'PercentChange')),
    'Feedstock':        lambda e: ('Feedstock::{}'.format(e['subtype']), param(e, 'PercentChange')),
    'SetFlag':          lambda e: ('Flag::{}'.format(flags[e['entity']]),),
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
    elif k == 'years':
        return 'years: {}'.format(v)
    elif k in ['locked', 'local', 'repeats']:
        return '{}: {}'.format(k, 'true' if v else 'false')
    elif is_float(v) and k != 'id':
        v = format_float(v)
    elif isinstance(v, bool):
        v = 'true' if v else 'false'
    return '{}: {}'.format(k.lower(), v)

def define_fields(fields, item):
    fields = [define_field(k, v, item) for k, v in fields]
    return ',\n'.join(fields)

def get_or(data, k):
    return data[k] if k in data else defaults[k]

def define_struct(typ, data):
    fields = [(k, get_or(data, k)) for k in specs[typ]]
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
    return 'const {}: [{}; {}] = [\n{}\n];'.format(name, typ,
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

def indent(text, levels=1):
    return textwrap.indent(text, 4 * levels * ' ')


if __name__ == '__main__':
    rust_output.append('// Do not edit this file. Use `editor/parse_content.py` to regenerate it.')
    rust_output.append('\n'.join('use {};'.format(imp) for imp in imports))

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

    const_defs = []
    income_level_consts = defaultdict(dict)
    for const in items_by_type['Const']:
        if 'name' not in const: continue

        if 'income' in const['name']:
            key, income_level = const['name'].split('__')
            income_level_consts[key][income_level] = const['value']
            continue

        const_def = define_const(const)
        const_defs.append(const_def)

    for key, income_levels in income_level_consts.items():
        if key == 'pop_change_coefs':
            sub_arrs = [
                '[\n{}\n]'.format(indent(',\n'.join(income_levels[k].split('\n'))))
                for k in [
                    'low_income',
                    'lower_middle_income',
                    'upper_middle_income',
                    'high_income']]

            const_def = 'const INCOME_POP_CHANGE_COEFS: [[f32; 4]; 4] = [\n{}\n];'.format(
                    indent(',\n'.join(sub_arrs)))
        else:
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

    rust_output.append(define_array('REGIONS', 'Region'))
    rust_output.append(define_array('INDUSTRIES', 'Industry'))
    rust_output.append(define_array('PROCESSES', 'Process'))
    rust_output.append(define_array('PROJECTS', 'Project'))
    rust_output.append(define_array('EVENTS', 'Event'))

    # import ipdb; ipdb.set_trace()

    with open('engine/src/content.rs', 'w') as f:
        f.write('\n\n'.join(rust_output))