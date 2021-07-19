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


for src in glob('maps/src/*.rds'):
    print(src)
    df = readRDS(src)
    result = convert(df)
    for k, arr in result['annual_pattern'].items():
        print(f'  {k}: {arr.shape}')

    # Not saving the coordinate map because
    # after examining them most if not all of them
    # seem to use roughly square cells, so we can just calculate
    # cell coordinates on-the-fly.
    # result['coordinate_map']
    outfile = src.replace('/src/', '/').replace('.rds', '.npz')
    np.savez(outfile, **result['annual_pattern'])