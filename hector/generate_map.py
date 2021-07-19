import pyhector
import numpy as np

# Highest resolution scaling pattern
size = (320, 160)
patterns = {
    'temp': np.load('maps/tas_Amon_MRI-ESM1_esmrcp85_r1i1p1_200601-210012_pattern.npz'),
    'precip': np.load('maps/pr_Amon_MRI-ESM1_rcp85_r1i1p1_200601-210012_pattern.npz')
}

def pscl_apply(pscl, tgav):
    """
    - pscl: scaling pattern
    - tgav: N x 1 matrix of global mean temperatures

    Ported from:
    - <https://rdrr.io/github/JGCRI/fldgen/man/pscl_apply.html>
    - <https://rdrr.io/github/JGCRI/fldgen/src/R/meanfield.R>
    """
    # Each value in tgav is a year
    # Each value in pscl['w'] is a cell
    # Each col in tscl is a year, each row is a cell
    tscl = tgav * pscl['w']

    # Each row is a year, each col is a cell
    return (tscl + pscl['b']).T


if __name__ == '__main__':
    output = pyhector.run(pyhector.rcp45, {'core': {'endDate': 2100}})

    # TODO The hectorui code adds 15 here which gives
    # the expected temperatures...but idk why
    tgav = output['temperature.Tgav'].values + 15
    result = pscl_apply(patterns['temp'], tgav)

    import ipdb; ipdb.set_trace()