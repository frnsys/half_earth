import json
from glob import glob

seen = set()
all_images = []
for f in glob('data/*.json'):
    data = json.load(open(f))
    for item in data.values():
        if 'image' in item:
            imdata = item['image']
            if isinstance(imdata, dict):
                if imdata['image'] in seen:
                    continue
                if 'name' not in item:
                    continue
                image = {
                    'name': item['name'],
                    'image': imdata['image'],
                    'original': imdata['original'],
                    'attribution': imdata.get('attribution', ''),
                }
                seen.add(imdata['image'])
                all_images.append(image)

with open('all_images.json', 'w') as f:
    json.dump(all_images, f)