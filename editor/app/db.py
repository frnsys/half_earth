import os
import json
import shutil
from datetime import datetime

class Database:
    def __init__(self, path):
        self.path = path
        self.load()

    def load(self):
        try:
            with open(self.path, 'r') as f:
                self.data = json.load(f)
        except FileNotFoundError:
            self.data = {}

    def save(self):
        if os.path.exists(self.path):
            backup_path = '/tmp/{}.json'.format(datetime.utcnow())
            shutil.copyfile(self.path, backup_path)
        try:
            with open(self.path, 'w') as f:
                json.dump(self.data, f)
        except:
            if os.path.exists(backup_path):
                shutil.copyfile(backup_path, self.path)

    def __getitem__(self, key):
        return self.data[key]

    def __setitem__(self, key, val):
        self.data[key] = val
