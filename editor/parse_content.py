import json
import textwrap
from collections import defaultdict

ids = {}
flags = {}
rust_output = []
specs = {
    'Region': ['name', 'income_level', 'health', 'outlook'],
    'Industry': ['name', 'resources', 'byproducts'],
    'Process': ['name', 'output', 'mix_share', 'feedstock', 'feedstock_amount', 'resources', 'byproducts', 'locked'],
    'Project': ['name', 'years', 'effects', 'locked'], # TODO construction, maintenance/ongoing, outcomes
    'Event': ['name', 'locked', 'local', 'repeats', 'effects', 'probabilities'],
    'Probability': ['likelihood', 'conditions'],
}
defaults = {
    'locked': False,
    'local': False,
    'repeats': False,
    'effects': [],
    'probabilities': [],
    'conditions': [],
    'resources': {},
    'byproducts': {},
    'mix_share': 0,

    # These will trigger type errors
    # in Rust, so we'll be notified
    # they need to be set
    'name': 0,
    'years': '',
    'feedstock': 0,
    'feedstock_amount': '',
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
    'RunsPlayed':    lambda e: (comps[e['comparator']], value(e)),
    'Flag':          lambda e: ('Flag::{}'.format(flags[e['entity']]),),
}

def define_effect(effect):
    effect_params = [
            '0.' if isinstance(v, float) and v == 0 else str(v)
            for v in effects[effect['type']](effect)]
    return 'Effect::{}({})'.format(
            effect['type'], ', '.join(effect_params))

def define_probability(prob):
    typ = 'Likelihood::{}'.format(prob['type'])
    return define_struct('Probability', {
        'likelihood': typ,
        'conditions': prob['conditions'],
    })

def define_condition(cond):
    cond_params = [
            '0.' if isinstance(v, float) and v == 0 else str(v)
            for v in conds[cond['type']](cond)]
    return 'Condition::{}({})'.format(
            cond['type'], ', '.join(cond_params))



def define_field(k, v):
    if k == 'income_level':
        return 'income: Income::{}'.format(v.replace('-', ''))
    elif k == 'name':
        v = '"{}"'.format(v)
    elif k == 'output':
        return 'output: Output::{}'.format(v)
    elif k == 'feedstock':
        return 'feedstock: Feedstock::{}'.format(v)
    elif k == 'byproducts':
        return 'byproducts: byproducts!(\n{}\n)'.format(
                    indent(define_fields(v.items())))
    elif k == 'resources':
        return 'resources: resources!(\n{}\n)'.format(
                    indent(define_fields(v.items())))
    elif k == 'effects':
        return 'effects: vec![\n{}\n]'.format(
                    indent('\n'.join(define_effect(e) for e in v)))
    elif k == 'probabilities':
        return 'probabilities: vec![\n{}\n]'.format(
                    indent('\n'.join(define_probability(e) for e in v)))
    elif k == 'conditions':
        return 'conditions: vec![\n{}\n]'.format(
                    indent('\n'.join(define_condition(e) for e in v)))
    elif k == 'years':
        return 'years: {}'.format(v)
    elif k in ['locked', 'local', 'repeats']:
        return '{}: {}'.format(k, 'true' if v else 'false')
    elif v == 0:
        v = '0.'
    return '{}: {}'.format(k.lower(), v)

def define_fields(fields):
    fields = [define_field(k, v) for k, v in fields]
    return ',\n'.join(fields)

def define_struct(typ, data):
    fields = [(k, data.get(k) or defaults[k]) for k in specs[typ]]
    return '''{} {{\n{}\n}}'''.format(typ, indent(define_fields(fields)))

def define_structs(typ, items):
    return ',\n'.join(define_struct(typ, item) for item in items)

def define_array(name, typ):
    items = items_by_type[typ]
    return 'const {}: [{}; {}] = [\n{}\n];'.format(name, typ,
        len(items), indent(define_structs(typ, items)))

def indent(text, levels=1):
    return textwrap.indent(text, 4 * levels * ' ')


if __name__ == '__main__':
    items = json.load(open('data.json'))
    items_by_type = defaultdict(list)
    for id, item in items.items():
        if item.get('deleted'): continue
        typ = item['_type']
        id_ = len(items_by_type[typ])
        ids[id] = id_
        items_by_type[typ].append(item)
        if typ == 'Flag':
            flags[id] = item['name']

    flag_struct_def = 'pub struct Flag {{{}}}'.format(
        indent(',\n'.join('{}: bool'.format(
            item['name']) for item in items_by_type['Flag'])))
    rust_output.append(flag_struct_def)

    const_defs = []
    for const in items_by_type['Const']:
        if 'name' not in const: continue
        const_def = 'const {}: f32 = {};'.format(const['name'].upper(), const['value'])
        const_defs.append(const_def)
    rust_output.append('\n'.join(const_defs))

    rust_output.append(define_array('REGIONS', 'Region'))
    rust_output.append(define_array('INDUSTRIES', 'Industry'))
    rust_output.append(define_array('PROCESSES', 'Process'))
    rust_output.append(define_array('PROJECTS', 'Project'))
    rust_output.append(define_array('EVENTS', 'Event'))

    import ipdb; ipdb.set_trace()

    with open('../engine/src/content.rs', 'w') as f:
        f.write('\n\n'.join(rust_output))