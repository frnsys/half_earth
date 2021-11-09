import math

def ban_process_cost(mix_share):
    return nearest_multiple(round((100*mix_share)**(3/4)), 5)

def promote_process_cost(mix_share):
    return nearest_multiple(round((100*(1-mix_share))**(3/4)), 5)

def nearest_multiple(v, base):
    return base * round(v/base)

for i in range(0, 10):
    mix_share = (i * 10)/100
    print('Mix Share:', mix_share)
    print(' Ban: ', ban_process_cost(mix_share))
    print(' Promote: ', promote_process_cost(mix_share))