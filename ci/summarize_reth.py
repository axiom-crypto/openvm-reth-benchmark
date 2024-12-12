import json
import argparse
import re
from tabulate import tabulate

METRIC_TAGS = {
    'halo2_proof_time_ms': 'Halo2 Prove',
    'execute_time_ms': 'Execution',
    'stark_prove_excluding_trace_time_ms': 'STARK Prove',
    'trace_gen_time_ms': 'Tracegen',
}

LABEL_PRIORITY = {
    'group': 0,
    'idx': 1,
    'segment': 2,
    'hgt': 3,
    'block_number': 4,
}

LABEL_TAG = {
    'group': '',
    'idx': 'idx=',
    'segment': 'seg=',
    'hgt': 'hgt=',
    'block_number': 'blk=',
}

def process_label(label):
    if 'height' in label:
        label = re.split('_height', label)[0]
    if label in ['leaf_verifier', 'internal_verifier', 'root_verifier']:
        label = re.split('_verifier', label)[0]
    return label

def main():
    argparser = argparse.ArgumentParser()
    argparser.add_argument('metrics_json', type=str, help="Path to the metrics JSON")
    argparser.add_argument('--out', type=str, help="Path to the output file")
    argparser.add_argument('--print', action='store_true', help="Print the output to the console", default=False)
    argparser.add_argument('--print-raw', action='store_true', help="Print the raw metrics to the console", default=False)
    args = argparser.parse_args()

    with open(args.metrics_json, 'r') as f:
        x = json.load(f)

    x = x['gauge']                # contains all the timing metrics, in ms
    x = [y for y in x if y['metric'] not in ['halo2_total_cells', 'halo2_keygen_time_ms']]
    for y in x:
        if 'group' not in [a[0] for a in y['labels']]:
            y['labels'].append(['group', 'dummy'])
    metrics = set([y['metric'] for y in x])

    block_number = 0
    z = {}
    for m in metrics:
        tag = METRIC_TAGS[m]
        z[tag] = {}
        group_sum = 0
        for y in x:
            if y['metric'] == m:
                y_tags = []
                for xx in sorted(y['labels'], key=lambda a: LABEL_PRIORITY[a[0]]):
                    if not (xx[0] in ['block_number']):
                        y_tags.append(LABEL_TAG[xx[0]] + process_label(xx[1]))
                    else:
                        block_number = int(xx[1])
                y_str = '|'.join(y_tags)
                sec = float(y['value']) / 1000
                z[tag][y_str] = [sec, sec / 60]
                group_sum += sec
        z[tag]['Total'] = [group_sum, group_sum / 60]

    parallel = {}
    for key in ['Execution', 'Tracegen', 'STARK Prove', 'Halo2 Prove']:
        parallel[key] = []

    for grp in ['reth_block', 'dummy', 'leaf', 'internal', 'root']:
        val = max([a['value'] for a in x if a['metric'] == 'execute_time_ms' and ['group', grp] in a['labels']], default=0)
        parallel['Execution'].append([grp, float(val) / 1000, float(val) / 60000])

    for grp in ['reth_block', 'dummy', 'leaf', 'internal', 'root']:
        val = max([a['value'] for a in x if a['metric'] == 'trace_gen_time_ms' and ['group', grp] in a['labels']], default=0)
        parallel['Tracegen'].append([grp, float(val) / 1000, float(val) / 60000])

    for grp in ['reth_block', 'dummy', 'leaf', 'internal', 'root']:
        val = max([a['value'] for a in x if a['metric'] == 'stark_prove_excluding_trace_time_ms' and ['group', grp] in a['labels']], default=0)
        parallel['STARK Prove'].append([grp, float(val) / 1000, float(val) / 60000])

    for grp in ['halo2_outer', 'halo2_wrapper']:
        val = max([a['value'] for a in x if a['metric'] == 'halo2_proof_time_ms' and ['group', grp] in a['labels']], default=0)
        parallel['Halo2 Prove'].append([grp, float(val) / 1000, float(val) / 60000])

    for tag, tbl in parallel.items():
        tbl.append(['Total', sum([a[1] for a in tbl]), sum([a[2] for a in tbl])])

    total_sum = sum([z[tag]['Total'][0] for tag in z])

    total_table = [
        ['Total', total_sum, total_sum / 60, sum([a[-1][1] for a in parallel.values()]), sum([a[-1][2] for a in parallel.values()])],
        ['Execution', z['Execution']['Total'][0], z['Execution']['Total'][1], parallel['Execution'][-1][1], parallel['Execution'][-1][2]],
        ['Tracegen', z['Tracegen']['Total'][0], z['Tracegen']['Total'][1], parallel['Tracegen'][-1][1], parallel['Tracegen'][-1][2]],
        ['STARK Prove', z['STARK Prove']['Total'][0], z['STARK Prove']['Total'][1], parallel['STARK Prove'][-1][1], parallel['STARK Prove'][-1][2]],
        ['Halo2 Prove', z['Halo2 Prove']['Total'][0], z['Halo2 Prove']['Total'][1], parallel['Halo2 Prove'][-1][1], parallel['Halo2 Prove'][-1][2]],
    ]
    for key, value in z['Tracegen'].items():
        if key in z['STARK Prove']:
            z['STARK Prove'][key].extend(value)
        else:
            split = re.split('\|seg=0', key)
            if len(split) > 0:
                key = split[0] + split[1]
            else:
                key = split[0]
            if key in z['STARK Prove']:
                z['STARK Prove'][key].extend(value)
            else:
                print("ERROR: Key not found {}".format(key))

    for key, value in z['STARK Prove'].items():
        if len(value) != 4:
            value.extend([0, 0])

    exec_table = list(map(lambda a: [a[0], a[1][0], a[1][1]], z['Execution'].items()))
    stark_table = list(map(lambda a: [a[0], a[1][0], a[1][1], a[1][2], a[1][3]], z['STARK Prove'].items()))
    halo2_table = list(map(lambda a: [a[0], a[1][0], a[1][1]], z['Halo2 Prove'].items()))

    if args.out:
        with open(args.out, 'w') as f:
            print(tabulate(total_table, headers=['Block ' + str(block_number), 'time (s)', 'time (m)', 'partime (s)', 'partime (m)'], tablefmt="pipe", floatfmt=".2f"), file=f)
            print(file=f)
            print(tabulate(exec_table, headers=['Block ' + str(block_number), 'Execution (s)', 'Execution (m)'], tablefmt="pipe", floatfmt=".2f"), file=f)
            print(file=f)
            print(tabulate(stark_table, headers=['Block ' + str(block_number), 'STARK Prove (s)', 'STARK Prove (m)', 'Tracegen (s)', 'Tracegen (m)'], tablefmt="pipe", floatfmt=".2f"), file=f)
            print(file=f)
            print(tabulate(halo2_table, headers=['Block ' + str(block_number), 'Halo2 Prove (s)', 'Halo2 Prove (m)'], tablefmt="pipe", floatfmt=".2f"), file=f)

    if args.print:
        print(tabulate(total_table, headers=['Block ' + str(block_number), 'time (s)', 'time (m)', 'partime (s)', 'partime (m)'], tablefmt="pipe", floatfmt=".2f"))
        print()        
        print(tabulate(exec_table, headers=['Block ' + str(block_number), 'Execution (s)', 'Execution (m)'], tablefmt="pipe", floatfmt=".2f"))
        print()
        print(tabulate(stark_table, headers=['Block ' + str(block_number), 'STARK Prove (s)', 'STARK Prove (m)', 'Tracegen (s)', 'Tracegen (m)'], tablefmt="pipe", floatfmt=".2f"))
        print()
        print(tabulate(halo2_table, headers=['Block ' + str(block_number), 'Halo2 Prove (s)', 'Halo2 Prove (m)'], tablefmt="pipe", floatfmt=".2f"))

    if args.print_raw:
        for y in x:
            print(y)

if __name__ == '__main__':
    main()
