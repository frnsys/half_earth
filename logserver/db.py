import json
import sqlite3
from datetime import datetime, timezone

class Database:
    def __init__(self, path):
        self.path = path
        _, cur = self._con()
        cur.execute('CREATE TABLE IF NOT EXISTS sessions \
                (session text primary key,\
                version text,\
                timestamp text,\
                useragent text)')
        cur.execute('CREATE TABLE IF NOT EXISTS snapshots \
                (timestamp text primary key,\
                session text,\
                snapshot json not null)')

    def _con(self):
        con = sqlite3.connect(self.path)
        cur = con.cursor()
        return con, cur

    def add_session(self, session_id, version, user_agent):
        timestamp = datetime.utcnow().replace(tzinfo=timezone.utc).timestamp()
        con, cur = self._con()
        cur.execute(
            'INSERT INTO sessions(session, version, timestamp, useragent) VALUES (?,?,?,?)',
            (session_id, version, timestamp, user_agent))
        con.commit()

    def add_snapshot(self, session_id, snapshot):
        timestamp = datetime.utcnow().replace(tzinfo=timezone.utc).timestamp()
        con, cur = self._con()
        cur.execute(
            'INSERT INTO snapshots(timestamp, session, snapshot) VALUES (?,?,?)',
            (timestamp, session_id, json.dumps(snapshot)))
        con.commit()

    def snapshots(self, session_id):
        _, cur = self._con()
        rows = cur.execute(
                'SELECT timestamp, session, snapshot FROM snapshots WHERE session == ?',
                (session_id,)).fetchall()
        return [{
            'session_id': session,
            'timestamp': timestamp,
            'snapshot': json.loads(snapshot),
        } for timestamp, session, snapshot in rows]

    def sessions(self):
        _, cur = self._con()
        rows = cur.execute(
                'SELECT timestamp, session, version, useragent FROM sessions').fetchall()
        return [{
            'id': session,
            'timestamp': timestamp,
            'version': version,
            'useragent': useragent,
        } for timestamp, session, version, useragent in rows]
