import numpy as np
from glob import glob
import rpy2.robjects as robjects
from rpy2.robjects import pandas2ri

pandas2ri.activate()
readRDS = robjects.r['readRDS']

# Recursively convert R data to dicts
def convert(r_df):
    d = { key : r_df.rx2(key) for key in r_df.names }
    for k, v in d.items():
        if type(v) == robjects.vectors.ListVector:
            d[k] = convert(v)
    return d


for src in glob('data/src/*.rds'):
    print(src)
    df = readRDS(src)
    result = convert(df)
    for k, arr in result['annual_pattern'].items():
        print(f'  {k}: {arr.shape}')

    # Don't use this for applying scaling patterns,
    # so just delete it. It takes up a lot of space.
    del result['annual_pattern']['r']

    # Not saving the coordinate map because
    # after examining them most if not all of them
    # seem to use roughly square cells, so we can just calculate
    # cell coordinates on-the-fly.
    # result['coordinate_map']
    outfile = src.replace('/src/', '/npz/').replace('.rds', '.npz')
    np.savez(outfile, **result['annual_pattern'])

# Highest resolution scaling pattern (320x160)
patterns = {
    'temp': np.load('data/npz/tas_Amon_MRI-ESM1_esmrcp85_r1i1p1_200601-210012_pattern.npz'),
    'precip': np.load('data/npz/pr_Amon_MRI-ESM1_rcp85_r1i1p1_200601-210012_pattern.npz')
}

# Create an include file for Rust
with open('data/temp_pattern.in', 'w') as f:
    rs_w = 'static TEMP_PATTERN_W: [f64; {size}] = [{vals}];'.format(
            size=patterns['temp']['w'].shape[0],
            vals='{}'.format(list(patterns['temp']['w'].flatten()))[1:-1])
    rs_b = 'static TEMP_PATTERN_B: [f64; {size}] = [{vals}];'.format(
            size=patterns['temp']['b'].shape[0],
            vals='{}'.format(list(patterns['temp']['b'].flatten()))[1:-1])
    f.write('\n'.join([rs_w, rs_b]))