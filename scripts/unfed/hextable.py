import json, sys, collections, statistics
for arr in ('P', 'T'):
    rows = [json.loads(l) for l in open(f'/tmp/unfed/rowsall{arr}.jsonl')]
    t = collections.defaultdict(list)
    for r in rows:
        for h in r['hexes']:
            t[(h['side'], tuple(h['hex']))].append((r['seed'], h))
    print(f'== arrangement {arr}: hexes with a man lost whose latest move was a fall-back there, or a march after a fall-back there (all units)')
    for (side, hx), lst in sorted(t.items()):
        seeds = [s for s, _ in lst]
        basis = collections.Counter()
        for _, h in lst:
            for k, v in h['by_basis'].items():
                basis[k] += v
        print(f'{side} {hx}: seeds {len(seeds)} of 100; men lost {sum(h["men_lost_in_table"] for _, h in lst)} {dict(basis)}')
        print(f'  men who fell back there (sum of the events\' men) {sum(h["men_fell_back"] for _, h in lst)}; fall-back days (min..max of first) '
              f'{min(h["fall_back_days"][0] for _, h in lst)}..{max(h["fall_back_days"][0] for _, h in lst)}; '
              f'first loss day {min(h["first_loss_day"] for _, h in lst)}..{max(h["first_loss_day"] for _, h in lst)} (median {statistics.median(h["first_loss_day"] for _, h in lst)}); '
              f'last loss day {min(h["last_loss_day"] for _, h in lst)}..{max(h["last_loss_day"] for _, h in lst)}')
        bc = collections.Counter()
        for _, h in lst:
            kinds = sorted({b['kind'] for b in h['bases_of_the_side_at_first_loss']})
            bc['+'.join(kinds) if kinds else 'no base of the side in the hex'] += 1
        print(f'  base of the side in the hex at the first loss: {dict(bc)}')
        tons = [h['tons_unloaded_from_first_fall_back'] for _, h in lst]
        print(f'  tons unloaded in the hex by the side from the first fall-back day: median {statistics.median(tons)}, min {min(tons)}, max {max(tons)}, seeds with 0: {sum(1 for x in tons if x == 0)}')
        lf = [h['last_men_fed_before_first_loss'] for _, h in lst if h['last_men_fed_before_first_loss']]
        if lf:
            print(f'  last men_fed at the hex (the event is of the hex, not of a unit) before the first loss: in {len(lf)} seeds; median day {statistics.median(x["day"] for x in lf)}; '
                  f'median men {statistics.median(x["men"] for x in lf)}, fed {statistics.median(x["fed"] for x in lf)}, unfed {statistics.median(x["unfed"] for x in lf)}; bases fed from {dict(collections.Counter(x["base"] for x in lf))}')
        else:
            print('  no men_fed event at the hex before the first loss')
        oc = collections.Counter(); zero = 0
        for _, h in lst:
            if not h['orders_naming_it']:
                zero += 1
            for k, v in h['orders_naming_it'].items():
                oc[k] += v
        print(f'  staff orders naming the hex from the first fall-back day, summed over its seeds: {dict(oc)}; seeds with none: {zero} of {len(lst)}')
