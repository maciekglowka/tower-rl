import argparse
import csv
import os
import yaml

STATIC_COLUMNS = ['min_level', 'spawn_chance']

def default_ctor(loader, tag_suffix, node):
    return tag_suffix + ' ' + node.value

parser = argparse.ArgumentParser()
parser.add_argument('-d', '--source_dir', default='.', nargs='?')
parser.add_argument('-o', '--output', default='entities.csv', nargs='?')
args = parser.parse_args()

entities = {}

yaml.add_multi_constructor('', default_ctor)

for file_path in os.listdir(args.source_dir):
    if os.path.splitext(file_path)[1] not in ['.yaml', '.yml']:
        continue
    with open(file_path, 'r') as f:
        content = yaml.unsafe_load(f)
    for k, v in content.items():
        entities[k] = v

component_names = set([
    c for e in entities.values()
    for c in e.get('components', {})
])
rows = []

header = STATIC_COLUMNS + list(component_names)

for k, v in entities.items():
    row = [k]
    for c in STATIC_COLUMNS:
        row.append(v.get(c, '-'))

    for c in component_names:
        row.append(v.get('components', {}).get(c, '-'))
    

    rows.append(row)

with open(args.output, 'w') as f:
    writer = csv.writer(f)
    writer.writerow(header)
    writer.writerows(rows)