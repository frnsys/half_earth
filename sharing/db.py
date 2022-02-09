import json
import sqlite3
from datetime import datetime, timezone

class Database:
    def __init__(self, path):
        self.path = path
        _, cur = self._con()
        cur.execute('CREATE TABLE IF NOT EXISTS sessions \
                (id text primary key,\
                timestamp text,\
                data json not null)')

    def _con(self):
        con = sqlite3.connect(self.path)
        cur = con.cursor()
        return con, cur

    def add(self, id, data):
        timestamp = datetime.utcnow().replace(tzinfo=timezone.utc).timestamp()
        con, cur = self._con()
        cur.execute(
            'INSERT INTO sessions(id, timestamp, data) VALUES (?,?,?)',
            (id, timestamp, json.dumps(data)))
        con.commit()

    def session(self, id):
        _, cur = self._con()
        res = cur.execute(
                'SELECT timestamp, data FROM sessions WHERE id == ?',
                (id,)).fetchone()
        if res:
            return {
                'timestamp': res[0],
                'data': json.loads(res[1])
            }
