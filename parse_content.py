# This is a mess! :(
import os
import re
import json
import textwrap
import urllib.request
from PIL import Image
from collections import defaultdict

BASE_WORLD_OUTLOOK = 20.
BASE_REGIONAL_OUTLOOK = 10.
BASE_REGIONAL_HABITABILITY = 10.

ids = {}
flags = {}
rust_output = []

regions_to_patterns = json.load(open('assets/src/scaling_patterns/out/regions_pscl.json'))

consts_template = \
'''// Do not edit this file. Use `parse_content.py` to regenerate it.
use crate::regions::Income;
use crate::kinds::{FeedstockMap, ResourceMap, OutputMap};
'''

content_template = \
'''// Do not edit this file. Use `parse_content.py` to regenerate it.
use crate::world::World;
use crate::events::Phase;
use crate::game::Difficulty;
use crate::industries::Industry;
use crate::regions::{Region, Income, Latitude};
use crate::projects::{Project, Outcome, Upgrade, Factor, Cost};
use crate::production::{Process, ProcessFeature};
use crate::kinds::{Resource, Output, Feedstock, Byproduct, ByproductMap, ResourceMap};
use crate::events::{Event, Aspect, Effect, Flag, Probability, Likelihood, Condition, Comparator, WorldVariable, LocalVariable, PlayerVariable};
use crate::projects::{Status as ProjectStatus, Type as ProjectType, Group as ProjectGroup};
use crate::npcs::{NPC, NPCRelation};
'''

world_fn_template = \
'''
pub fn world(difficulty: Difficulty) -> World {{
    let mut world = World::default();
    world.regions = regions();
    match difficulty {{
        Difficulty::Easy => {{\n{easy}
        }},
        Difficulty::Normal => {{\n{normal}
        }},
        Difficulty::Hard => {{\n{hard}
        }}
    }}
    world
}}
'''

byproduct_map = {
    'CO2': 'Co2',
    'CH4': 'Ch4',
    'N2O': 'N2o',
    'Biodiversity': 'Biodiversity',
}

# Required keys and defaults
# `None` means there is no default
specs = {
    'World': {
        'year': 0,
        'extinction_rate': 0.,
        'temperature': 0.,
        'sea_level_rise': 0.,
        'base_outlook': BASE_WORLD_OUTLOOK,
    },
    'Region': {
        'id': None,
        'name': None,
        'temp_lo': 0.,
        'temp_hi': 0.,
        'precip_lo': 0.,
        'precip_hi': 0.,
        'income_level': None,
        'latitude': None,
        'development': 0,
        'outlook': BASE_REGIONAL_OUTLOOK,
        'population': None,
        'base_habitability': BASE_REGIONAL_HABITABILITY,
        'seceded': 'false',
        'flags': 'vec![]',
        'pattern_idxs': None,
    },
    'Industry': {
        'id': None,
        'name': None,
        'resources': {},
        'byproducts': {},
        'resource_modifiers': {},
        'byproduct_modifiers': {},
        'demand_modifier': 1.0
    },
    'Process': {
        'id': None,
        'name': None,
        'output': None,
        'limit': None,
        'mix_share': 0,
        'feedstock': None,
        'resources': {},
        'byproducts': {},
        'locked': 'false',
        'features': {},
        'output_modifier': 0.,
        'byproduct_modifiers': {},
        'supporters': [],
        'opposers': [],
    },
    'Project': {
        'id': None,
        'name': None,
        'cost': 0,
        'base_cost': 0,
        'progress': 0.,
        'level': 0,
        'required_majority': 0.,
        'completed_at': 0,
        'effects': [],
        'type': '',
        'locked': 'false',
        'status': 'ProjectStatus::Inactive',
        'group': 'Other',
        'ongoing': 'false',
        'gradual': 'false',
        'outcomes': [],
        'estimate': 0,
        'points': 0,
        'cost_modifier': 1.0,
        'upgrades': 'vec![]',
        'supporters': [],
        'opposers': [],
        'active_outcome': None,
    },
    'Event': {
        'id': None,
        'name': None,
        'type': None,
        'locked': 'false',
        'regional': 'false',
        'effects': [],
        'probabilities': [],
        'prob_modifier': 1.0,
        'dialogue': None,
        'intensity': 0,
        'aspect': None
    },
    'NPC': {
        'id': None,
        'name': None,
        'relationship': 3,
        'locked': 'false',
        'support': 100,
        'seats': 0.,
    },
    'Probability': {
        'likelihood': None,
        'conditions': [],
    },
    'Outcome': {
        'effects': [],
        'probability': [],
    },
    'Upgrade': {
        'active': 'false',
        'cost': 0,
        'effects': [],
    },
    'FeedstockMap': {
        'oil': 0.,
        'coal': 0.,
        'uranium': 0.,
        'lithium': 0.,
        'natural_gas': 0.,
        'soil': 0.,
        'other': 0.,
        'thorium': 0.,
    },
    'OutputMap': {
        'fuel': 0.,
        'electricity': 0.,
        'animal_calories': 0.,
        'plant_calories': 0.,
    },
    'ResourceMap': {
        'fuel': 0.,
        'electricity': 0.,
        'land': 0.,
        'water': 0.,
    }
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

# Nearest multiple of 5
def nearest_multiple(v):
    return 5. * round(v/5.)


effects = {
    'UnlocksProject':   lambda e: (ids[e['entity']],),
    'UnlocksProcess':   lambda e: (ids[e['entity']],),
    'UnlocksNPC':       lambda e: (ids[e['entity']],),
    'AddEvent':         lambda e: (ids[e['entity']],),
    'TriggerEvent':     lambda e: (ids[e['entity']], e['params']['Delay (years)']),
    'LocalVariable':    lambda e: ('LocalVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'WorldVariable':    lambda e: ('WorldVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'PlayerVariable':   lambda e: ('PlayerVariable::{}'.format(e['subtype']), param(e, 'Change')),
    'Demand':           lambda e: ('Output::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'DemandAmount':     lambda e: ('Output::{}'.format(e['subtype']), param(e, 'Change')),
    'Output':           lambda e: ('Output::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'OutputForFeature': lambda e: ('ProcessFeature::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'OutputForProcess': lambda e: (ids[e['entity']], param(e, 'PercentChange')/100),
    'CO2ForFeature':    lambda e: ('ProcessFeature::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'ProcessLimit':     lambda e: (ids[e['entity']], param(e, 'Change')),
    'Resource':         lambda e: ('Resource::{}'.format(e['subtype']), param(e, 'Amount')),
    'Feedstock':        lambda e: ('Feedstock::{}'.format(e['subtype']), param(e, 'PercentChange')/100),
    'SetProjectStatus': lambda e: (ids[e['entity']], 'ProjectStatus::{}'.format(e['subtype']), int(param(e, 'Duration'))),
    'ProjectRequest':   lambda e: (ids[e['entity']], 'true' if e['subtype'] == 'Implement' else 'false', int(param(e, 'Bounty'))),
    'ProcessRequest':   lambda e: (ids[e['entity']], 'true' if e['subtype'] == 'Unban' else 'false', int(param(e, 'Bounty'))),
    'AddRegionFlag':    lambda e: ('"{}".to_string()'.format(e['params'].get('Flag')),),
    'AddFlag':          lambda e: ('Flag::{}'.format(e['params'].get('Flag')),),
    'RegionHabitability': lambda e:  ('Latitude::{}'.format(e['subtype']), param(e, 'Change')),
    'RegionLeave':      lambda _: (),
    'Migration':        lambda _: (),
    'GameOver':         lambda _: (),
    'AutoClick':        lambda e: (ids[e['entity']], param(e, 'Chance')),
    'NPCRelationship':  lambda e: (ids[e['entity']], int(param(e, 'Change'))),
    'ModifyIndustryByproducts':  lambda e: (ids[e['entity']], 'Byproduct::{}'.format(byproduct_map[e['subtype']]), param(e, 'Multiplier')),
    'ModifyIndustryResources':   lambda e: (ids[e['entity']], 'Resource::{}'.format(e['subtype']), param(e, 'Multiplier')),
    'ModifyIndustryResourcesAmount':   lambda e: (ids[e['entity']], 'Resource::{}'.format(e['subtype']), param(e, 'Amount')),
    'ModifyEventProbability':    lambda e: (ids[e['entity']], param(e, 'Change')),
    'ModifyIndustryDemand':      lambda e: (ids[e['entity']], param(e, 'Change')),
    'DemandOutlookChange':       lambda e: ('Output::{}'.format(e['subtype']), param(e, 'Multiplier')),
    'IncomeOutlookChange':       lambda e: (param(e, 'Multiplier'),),
    'ProjectCostModifier':       lambda e: (ids[e['entity']], param(e, 'Change')),
    'ProtectLand':               lambda e: (param(e, 'Percent'),),
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
    'PlayerVariable':    lambda e: ('PlayerVariable::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'Demand':           lambda e: ('Output::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'OutputDemandGap':  lambda e: ('Output::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'ResourcePressure': lambda e: ('Resource::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'ResourceDemandGap':lambda e: ('Resource::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'FeedstockYears':        lambda e: ('Feedstock::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'ProcessMixShare':  lambda e: (ids[e['entity']], comps[e['comparator']], value(e)),
    'ProcessMixShareFeature': lambda e: ('ProcessFeature::{}'.format(e['subtype']), comps[e['comparator']], value(e)),
    'ProjectActive':    lambda e: (ids[e['entity']], 'ProjectStatus::Active'),
    'ProjectInactive':    lambda e: (ids[e['entity']], 'ProjectStatus::Inactive'),
    'ProjectFinished':    lambda e: (ids[e['entity']], 'ProjectStatus::Finished'),
    'ProjectStalled':    lambda e: (ids[e['entity']], 'ProjectStatus::Stalled'),
    'ProjectHalted':    lambda e: (ids[e['entity']], 'ProjectStatus::Halted'),
    'ProjectBuilding':    lambda e: (ids[e['entity']], 'ProjectStatus::Building'),
    'RunsPlayed':    lambda e: (comps[e['comparator']], e['value']),
    'HeavyProjects':    lambda e: (comps[e['comparator']], e['value']),
    'ProtectLand':    lambda e: (comps[e['comparator']], e['value']),
    'RegionFlag':    lambda e: ('"{}".to_string()'.format(e['value']),),
    'HasFlag':          lambda e: ('Flag::{}'.format(e['value']),),
    'NPCRelationship':  lambda e: (ids[e['entity']], 'NPCRelation::{}'.format(e['subtype'])),
}

effect_keys = {
  'LocalVariable': ['subtype', 'params'],
  'WorldVariable': ['subtype', 'params'],
  'PlayerVariable': ['subtype', 'params'],
  'Demand': ['subtype', 'params'],
  'DemandAmount': ['subtype', 'params'],
  'Output': ['subtype', 'params'],
  'OutputForFeature': ['subtype', 'params'],
  'OutputForProcess': ['entity', 'params'],
  'CO2ForFeature': ['subtype', 'params'],
  'Resource': ['subtype', 'params'],
  'Feedstock': ['subtype', 'params'],
  'TriggerEvent': ['entity', 'params'],
  'AddEvent': ['entity'],
  'UnlocksProject': ['entity'],
  'UnlocksProcess': ['entity'],
  'UnlocksNPC': ['entity'],
  'SetProjectStatus': ['entity', 'subtype', 'params'],
  'RegionLeave': [],
  'Migration': [],
  'GameOver': [],
  'ProjectRequest': ['entity', 'subtype', 'params'],
  'ProcessRequest': ['entity', 'subtype', 'params'],
  'AddFlag': ['params'],
  'AddRegionFlag': ['params'],
  'AutoClick': ['entity', 'params'],
  'NPCRelationship': ['entity', 'params'],
  'ModifyIndustryByproducts': ['entity', 'subtype', 'params'],
  'ModifyIndustryResources': ['entity', 'subtype', 'params'],
  'ModifyIndustryResourcesAmount': ['entity', 'subtype', 'params'],
  'ModifyIndustryDemand': ['entity', 'params'],
  'ModifyEventProbability': ['entity', 'params'],
  'DemandOutlookChange': ['subtype', 'params'],
  'IncomeOutlookChange': ['params'],
  'ProjectCostModifier': ['entity', 'params'],
  'ProtectLand': ['params'],
  'ProcessLimit': ['entity', 'params'],
  'RegionHabitability': ['subtype', 'params'],
};



def define_effect(effect):
    try:
        # Sometimes we have effects that aren't fully-defined
        # and this will break. So just skip them for now.
        effect_params = [
                '0.' if isinstance(v, float) and v == 0 else str(v)
                for v in effects[effect['type']](effect)]
    except:
        return None
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

def define_upgrade(u):
    return define_struct('Upgrade', {
        'active': 'false',
        'cost': u['cost'],
        'effects': u['effects'],
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
    if 'Project' in cond_type and cond_type != 'HeavyProjects':
        cond_type = 'ProjectStatus'
    return 'Condition::{}({})'.format(
            cond_type, ', '.join(cond_params))

def define_field(k, v, item):
    if k == 'active_outcome':
        return 'active_outcome: None'
    if k == 'income_level':
        return 'income: Income::{}'.format(v.replace('-', ''))
    if k == 'latitude':
        return 'latitude: Latitude::{}'.format(v)
    if k == 'year':
        return 'year: {}'.format(v)
    elif k == 'name' or k == 'text':
        v = '"{}"'.format(v)
    elif k == 'type' and item['_type'] == 'Project':
        return 'kind: ProjectType::{}'.format(v)
    elif k == 'group':
        return 'group: ProjectGroup::{}'.format(v)
    elif k == 'type' and item['_type'] == 'Event':
        if item.get('subphase') is not None:
            return 'phase: Phase::{}{}'.format(v, item['subphase'])
        else:
            return 'phase: Phase::{}'.format(v)
    elif k == 'output':
        return 'output: Output::{}'.format(v)
    elif k == 'limit':
        if not v:
            return 'limit: None'
        else:
            return 'limit: Some({})'.format(format_float(v))
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
    elif k == 'resource_modifiers':
        return 'resource_modifiers: resources!()'
    elif k == 'byproduct_modifiers':
        return 'byproduct_modifiers: byproducts!()'
    elif k == 'aspects':
        aspects = ['Aspect::{}'.format(a) for a in v]
        return 'aspects: vec![\n{}\n]'.format(
                    indent(',\n'.join(aspects)))
    elif k == 'aspect':
        if v is not None:
            return 'aspect: Some(Aspect::{})'.format(v)
        else:
            return 'aspect: None'
    if k == 'cost':
        if item.get('_type') == 'Project':
            return 'cost: 0'
        else:
            return 'cost: {}'.format(v)
    if k == 'supporters':
        return 'supporters: vec![{}]'.format(
                ', '.join(str(ids[id]) for id in v))
    if k == 'opposers':
        return 'opposers: vec![{}]'.format(
                ', '.join(str(ids[id]) for id in v))
    if k == 'train_cost' or k == 'establish_cost':
        return '{}: {}'.format(k, v)
    if k == 'intensity':
        return '{}: {}'.format(k, v)
    if k == 'level':
        return 'level: 0'
    if k == 'completed_at':
        return 'completed_at: 0'
    elif k == 'base_cost':
        v = item.get('cost', 0)
        if item.get('dynamic_cost', False):
            factor = item.get('dynamic_cost_factor')
            if factor in valid_outputs:
                variant = 'Factor::Output(Output::{})'.format(factor)
            elif factor == 'Time':
                variant = 'Factor::Time'
            elif factor == 'Income':
                variant = 'Factor::Income'
            else:
                raise Exception('Unrecognized dynamic cost factor: {}'.format(factor))
            v = '{}.'.format(v) if isinstance(v, int) else v
            return 'base_cost: Cost::Dynamic({}, {})'.format(v, variant)
        else:
            return 'base_cost: Cost::Fixed({})'.format(v)
    elif k == 'effects':
        return 'effects: vec![\n{}\n]'.format(
                    indent(',\n'.join(filter(None, (define_effect(e) for e in v)))))
    elif k == 'probability':
        return 'probability: {}'.format(define_probability(v))
    elif k == 'probabilities':
        return 'probabilities: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_probability(e) for e in v)))
    elif k == 'conditions':
        return 'conditions: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_condition(e) for e in v)))
    elif k == 'regional':
        regional = False
        for prob in item.get('probabilities', []):
            for cond in prob.get('conditions', []):
                if cond['type'] in ['LocalVariable', 'RegionFlag']:
                    regional = True
        return 'regional: {}'.format('true' if regional else 'false')
    elif k == 'pattern_idxs':
        return 'pattern_idxs: vec![{}]'.format(str(regions_to_patterns[item['name']])[1:-1])
    elif k == 'upgrades':
        if isinstance(v, list):
            return 'upgrades: vec![\n{}\n]'.format(
                        indent(',\n'.join(define_upgrade(e) for e in v)))
    elif k == 'outcomes':
        outcomes = [c for c in v if c.get('text')]
        return 'outcomes: vec![\n{}\n]'.format(
                    indent(',\n'.join(define_outcome(o) for o in outcomes)))
    elif k in ['cost', 'points', 'estimate']: # Keep as integer
        pass
    elif k == 'points':
        return 'points: {}'.format(v)
    elif k == 'relationship':
        return 'relationship: {}'.format(v)
    elif k == 'mix_share':
        return 'mix_share: {}'.format(int(nearest_multiple(v)/5))
    elif k == 'features':
        feats = ['ProcessFeature::{}'.format(feat) for feat, on in v.items() if on]
        return 'features: vec![\n{}\n]'.format(
                    indent(',\n'.join(feats)))
    elif k == 'dialogue':
        if not v:
            return 'branches: vec![]'
        branches = extract_branches(v)
        return 'branches: vec![{}]'.format(', '.join(branches))
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
    except Exception:
        print('Error defining struct of type "{}":\n'.format(typ),
                json.dumps(data, indent=2, sort_keys=True))
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

def extract_branches(dialogue):
    branches = []
    branch_id = 0
    for l in dialogue['lines'].values():
        if isinstance(l['next'], list):
            for j, b in enumerate(l['next']):
                b['id'] = branch_id
                branch_id += 1
                effects = 'vec![{}]'.format(', '.join(define_effect(e) for e in b.get('effects', [])))
                conditions = 'vec![{}]'.format(', '.join(define_condition(e) for e in b.get('conditions', [])))
                branches.append('({}, {})'.format(effects, conditions))
    return branches

def extract_dialogue(dialogue):
    if not dialogue: return {
        'root': 0,
        'lines': {
            '0': {
                'id': 0,
                'speaker': 'Gossy',
                'text': 'This is placeholder for missing dialogue text',
                'next': None
            }
        }
    }
    keys_to_ids = {k: i for i, k in enumerate(dialogue['lines'].keys())}

    branch_id = 0
    dialogue['root'] = keys_to_ids[str(dialogue['root'])]
    dialogue['lines'] = {keys_to_ids[k]: l for k, l in dialogue['lines'].items()}
    for l in dialogue['lines'].values():
        l['id'] = keys_to_ids[str(l['id'])]
        if isinstance(l['next'], list):
            for j, b in enumerate(l['next']):
                b['id'] = branch_id
                branch_id += 1
                if 'conditions' in b:
                    del b['conditions']
                if 'effects' in b:
                    del b['effects']
                if l.get('decision', False):
                    b['line_id'] = keys_to_ids[b['line_id']]
                    if dialogue['lines'][b['line_id']]['text'] == '':
                        b['line_id'] = None
                else:
                    b['line_id'] = keys_to_ids[b['line_id']]
        elif l['next'] != None:
            l['next'] = keys_to_ids[l['next']]

    return dialogue

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
        return 'pub const {}: f32 = {};'.format(
                const['name'].upper(),
                format_float(const['value']))
    elif const['type'] == 'float list':
        vals = const['value'].split('\n')
        return 'pub const {}: [f32; {}] = [\n{}\n];'.format(
            const['name'].upper(), len(vals), indent(',\n'.join(vals)))
    else:
        raise Exception('Unrecognized const type:', const['type'])

def define_content_fn(name, typ):
    return 'pub fn {}() -> Vec<{}> {{\n{}\n}}'.format(
        name, typ, indent('vec![\n{}\n]'.format(
            indent(define_structs(typ, items_by_type[typ])))))

cond_to_factor = {
    'WorldVariable': {
        'Temperature': 'warming',
        'Outlook': 'contentedness',
        'ExtinctionRate': 'extinction_rate',
        'SeaLevelRise': 'sea_level_rise',
    },
    'LocalVariable': {
        'Outlook': 'contentedness',
        'Habitability': 'habitability',
    },
    'ProcessMixShareFeature': {
        'IsCCS': 'IsCCS',
        'MakesNuclearWaste': 'MakesNuclearWaste',
        'CanMeltdown': 'CanMeltdown',
        'IsFossil': 'IsFossil',
        'UsesPesticides': 'UsesPesticides',
        'UsesLivestock': 'UsesLivestock',
    }
}
def condition_to_factor(cond):
    subtypes = cond_to_factor.get(cond['type'], {})
    return subtypes.get(cond['subtype'])

def parse_effect(e):
    effect = {
        'type': e['type'],
    }
    for k in effect_keys[e['type']]:
        if k == 'entity':
            effect[k] = ids.get(e.get('entity')),
        elif k == 'params':
            effect['param'] = get_param(e)
        else:
            effect[k] = e[k]
    return effect

def to_jpg(path, outpath, width=None):
    im = Image.open(path)
    if width is not None:
        w, h = im.size
        ratio = h/w
        new_h = int(ratio * width)
        im = im.resize((width, new_h), Image.ANTIALIAS)
    im.convert('RGB').save(outpath, quality=30)


def indent(text, levels=1):
    return textwrap.indent(text, 4 * levels * ' ')



if __name__ == '__main__':
    # For downloading missing images
    existing_images = os.listdir('editor/uploads')
    missing_images = []

    # Parse items into groups and so on
    items = json.load(open('editor/data.json'))
    items_by_type = defaultdict(list)
    for id, item in items.items():
        if item.get('deleted'): continue
        typ = item['_type']
        id_ = len(items_by_type[typ])
        ids[id] = id_
        items_by_type[typ].append(item)

        # Convert non-starter world events
        # by adding an additional condition
        # that the year must be > 2025
        if typ == 'Event':
            if item.get('type') == 'World' and not item.get('starter', False):
                for probability in item.get('probabilities', []):
                    probability['conditions'].append({
                        'comparator': '>',
                        'type': 'WorldVariable',
                        'subtype': 'Year',
                        'value': '2025'
                    })
        if 'image' in item:
            fname = item['image']['image']
            if fname not in existing_images:
                missing_images.append(fname)

    # Fetch images
    if missing_images:
        print('Downloading missing images...')
        for fname in missing_images:
            img_url = 'http://half-earth-editor.frnsys.com/image/{}'.format(fname)
            urllib.request.urlretrieve(img_url, 'editor/uploads/{}'.format(fname))

    # Define constants
    rust_output = [consts_template]
    const_defs = []
    feedstock_reserves = {}
    starting_resources = {}
    income_level_consts = defaultdict(dict)
    for const in items_by_type['Const']:
        if 'name' not in const: continue

        if 'income' in const['name']:
            key, income_level = const['name'].split('__')
            income_level_consts[key][income_level] = const['value']
            continue
        if 'reserves' in const['name']:
            feedstock, _ = const['name'].split('__')
            feedstock_reserves[feedstock] = const['value']
            continue
        if 'available' in const['name']:
            _, resource = const['name'].split('__')
            starting_resources[resource] = const['value']
            continue

        const_def = define_const(const)
        const_defs.append(const_def)

    rust_output.append(
        'pub const FEEDSTOCK_RESERVES: FeedstockMap<f32> = {};'.format(
            define_struct('FeedstockMap', feedstock_reserves)))
    rust_output.append(
        'pub const STARTING_RESOURCES: ResourceMap<f32> = {};'.format(
            define_struct('ResourceMap', starting_resources)))

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
        income_arms.append('Income::{} => {} + {}*year + {}*year.powf(2.0) + {}*year.powf(3.0)'.format(
            variants[k],
            *coefs
        ))
    fn_def = ('pub fn income_pop_change(year: f32, income: &Income) -> f32 {{\n    match income {{\n{}\n    }}\n}}'.format(
        indent(',\n'.join(income_arms), levels=2)))
    rust_output.append(fn_def)

    groups = []
    demand_levels = {}
    for income in incomes:
        outputs = {}
        for k in valid_outputs:
            outputs[camel_to_snake(k)] = income_level_consts[camel_to_snake(k)][income]
        demand_levels[income] = {k: float(v) for k, v in outputs.items()}
        groups.append(define_struct('OutputMap', outputs))
    demand_levels = [demand_levels[income] for income in incomes]

    for k in valid_outputs:
        income_level_consts.pop(camel_to_snake(k))

    output_demand = 'pub const OUTPUT_DEMAND: [OutputMap<f32>; 4] = [\n{}\n];'.format(
            indent(',\n'.join(groups)))
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
    world_adjustments = []
    for world in sorted(items_by_type['World'], key=lambda w: w['year']):
        world_adjustments.append(
            indent('\n'.join('world.{} = {};'.format(k, world.get(k, default))
                for k, default in specs['World'].items()), levels=3))

    world_fn = world_fn_template.format(
        easy=world_adjustments[0],
        normal=world_adjustments[1],
        hard=world_adjustments[2])
    rust_output.append(world_fn)

    rust_output.append(define_content_fn('regions', 'Region'))
    rust_output.append(define_content_fn('industries', 'Industry'))
    rust_output.append(define_content_fn('processes', 'Process'))
    rust_output.append(define_content_fn('projects', 'Project'))
    rust_output.append(define_content_fn('events', 'Event'))
    rust_output.append(define_content_fn('npcs', 'NPC'))
    with open('engine/src/content.rs', 'w') as f:
        f.write('\n\n'.join(rust_output))

    # Javascript exports
    icons = set()
    icon_events = {}

    def get_param(e):
        if e.get('params'):
            ef = effects[e['type']](e)
            if ef:
                return ef[-1]
        return None

    all_events = {}
    for ev in items_by_type['Event']:
        id = ev['id']
        image = ev.get('image', {})
        fname = image.get('image', None)
        attribution = image.get('attribution', None)
        factors = set()
        for p in ev.get('probabilities', []):
            for cond in p['conditions']:
                factor = condition_to_factor(cond)
                if factor is not None: factors.add(factor)
        event = {
            'name': ev.get('name', ''),
            'arc': ev.get('arc', ''),
            'dialogue': extract_dialogue(ev.get('dialogue', {})),
            'image': {
                'fname': fname.replace('.png', '.jpg') if fname is not None else None,
                'attribution': attribution,
            },
            'factors': list(factors),
            'effects': [parse_effect(e) for e in ev.get('effects', [])]
        }

        if fname:
            frm = 'editor/uploads/{}'.format(fname)
            to = 'assets/content/images/{}'.format(fname.replace('.png', '.jpg'))
            to_jpg(frm, to, width=600)

        all_events[id] = event

        if ev.get('type') == 'Icon':
            id = ev['id']
            # kinda hacky
            valid_subtypes = ['Outlook', 'Emissions', 'Electricity', 'PlantCalories']
            param_keys = {'Outlook': 'Change', 'Emissions': 'Change', 'Electricity': 'PercentChange', 'PlantCalories': 'PercentChange'}
            if len(ev['effects']) > 1:
                raise Exception('Icon events should have only one effect')

            effs = []
            for e in ev['effects']:
                st = e['subtype']
                if st not in valid_subtypes: continue
                val =int(e['params'][param_keys[e['subtype']]])
                effs.append((st, val))
            icon_events[id] = {
                'name': ev['name'],
                'icon': ev['icon'],
                'aspect': ev['aspect'],
                'intensity': ev['intensity'],
                'effects': [parse_effect(e) for e in ev.get('effects', [])]
            }
            icons.add(ev['icon'])

    with open('assets/content/icon_events.json', 'w') as f:
        json.dump(icon_events, f)

    with open('assets/content/icons.json', 'w') as f:
        json.dump(list(icons), f)
    for icon in icons:
        if not os.path.exists('assets/icons/pips/{}.png'.format(icon)):
            print('Missing icon:', icon)

    with open('assets/content/events.json', 'w') as f:
        json.dump(all_events, f)

    projects = []
    for p in items_by_type['Project']:
        id = p['id']
        image = p.get('image', {})
        fname = image.get('image', None)
        attribution = image.get('attribution', None)
        project = {
            'name': p['name'],
            'image': {
                'fname': fname.replace('.png', '.jpg') if fname is not None else None,
                'attribution': attribution,
            },
            'description': p.get('description', ''),
            'effects': [parse_effect(e) for e in p.get('effects', [])],
            'upgrades': [{
                'cost': u['cost'],
                'effects': [{
                    'type': e['type'],
                    'subtype': e.get('subtype'),
                    'entity': ids.get(e.get('entity')),
                    'param': get_param(e)
                } for e in u['effects']]
            } for u in p.get('upgrades', [])],
            'outcomes': [{
                'text': u.get('text', ''),
                'effects': [parse_effect(e) for e in u['effects']]
            } for u in p.get('outcomes', [])]
        }
        if fname:
            frm = 'editor/uploads/{}'.format(fname)
            to = 'assets/content/images/{}'.format(fname.replace('.png', '.jpg'))
            to_jpg(frm, to, width=600)
        projects.append(project)
    with open('assets/content/projects.json', 'w') as f:
        json.dump(projects, f)

    processes = []
    # Check process allocations are correct
    processes_by_output = defaultdict(list)
    for p in items_by_type['Process']:
        id = p['id']
        image = p.get('image', {})
        fname = image.get('image', None)
        attribution = image.get('attribution', None)
        process = {
            'image': {
                'fname': fname.replace('.png', '.jpg') if fname is not None else None,
                'attribution': attribution,
            },
            'description': p.get('description', ''),
        }
        if fname:
            frm = 'editor/uploads/{}'.format(fname)
            to = 'assets/content/images/{}'.format(fname.replace('.png', '.jpg'))
            to_jpg(frm, to, width=600)
        processes.append(process)
        processes_by_output[p['output']].append(p)
    with open('assets/content/processes.json', 'w') as f:
        json.dump(processes, f)

    for ps in processes_by_output.values():
        allocation = [nearest_multiple(p['mix_share'])/5 for p in ps]
        assert sum(allocation) == 100/5

    industries = []
    for p in items_by_type['Industry']:
        id = p['id']
        image = p.get('image', {})
        fname = image.get('image', None)
        attribution = image.get('attribution', None)
        industry = {
            'image': {
                'fname': fname.replace('.png', '.jpg') if fname is not None else None,
                'attribution': attribution,
            },
        }
        if fname:
            frm = 'editor/uploads/{}'.format(fname)
            to = 'assets/content/images/{}'.format(fname.replace('.png', '.jpg'))
            to_jpg(frm, to, width=600)
        industries.append(industry)
    with open('assets/content/industries.json', 'w') as f:
        json.dump(industries, f)

    regions = []
    for p in items_by_type['Region']:
        id = p['id']
        image = p.get('image', {})
        fname = image.get('image', None)
        attribution = image.get('attribution', None)
        region = {
            'image': {
                'fname': fname.replace('.png', '.jpg') if fname is not None else None,
                'attribution': attribution,
            },
        }
        if fname:
            frm = 'editor/uploads/{}'.format(fname)
            to = 'assets/content/images/{}'.format(fname.replace('.png', '.jpg'))
            to_jpg(frm, to, width=600)
        regions.append(region)
    with open('assets/content/regions.json', 'w') as f:
        json.dump(regions, f)


    npcs = []
    for p in items_by_type['NPC']:
        id = p['id']
        npc = {
            'name': p['name'],
            'description': p.get('description', ''),
            'color': p.get('color', ''),
        }
        npcs.append(npc)
    with open('assets/content/npcs.json', 'w') as f:
        json.dump(npcs, f)


    all_effects = []
    def find_effects(item):
        effects = item.get('effects', [])
        for v in item.values():
            if isinstance(v, dict):
                effects += find_effects(v)
            elif isinstance(v, list):
                for x in v:
                    if isinstance(x, dict):
                        effects += find_effects(x)
        return effects
    for item in items.values():
        all_effects += find_effects(item)

    flag_descs = {}
    for effect in all_effects:
        if effect['type'] == 'AddFlag':
            flag_descs[effect['params']['Flag']] = effect['params']['Description']
    with open('assets/content/flags.json', 'w') as f:
        json.dump(flag_descs, f)

    # Create default emissions for everything else
    # Just use the last value
    rcp = json.load(open('assets/hector/rcp26.to_2050.json', 'r'))
    defaults = {}
    for k, vals in rcp['data'].items():
        defaults[k] = vals[-1]
    with open('assets/hector/rcp26.default_emissions.json', 'w') as f:
        json.dump(defaults, f)

    with open('src/consts.json', 'w') as f:
        json.dump({
            'demand_levels': demand_levels,
            'base_outlook': BASE_REGIONAL_OUTLOOK,
            'base_world_outlook': BASE_WORLD_OUTLOOK,
            'base_habitability': BASE_REGIONAL_HABITABILITY,
            'starting_resources': {k: float(v) for k, v in starting_resources.items()},
        }, f)

    d = json.load(open('assets/surface/regions_to_tiles.json'))

    region_names = [r['name'] for r in items_by_type['Region']]
    tiles_to_regions = {}
    for region, tags in d.items():
        for tag, tiles in tags.items():
            for tile in tiles:
                tiles_to_regions[tile] = region_names.index(region)

    with open('assets/surface/tiles_to_regions.json', 'w') as f:
        json.dump(tiles_to_regions, f)