import json
import sqlite3
from http.server import BaseHTTPRequestHandler, HTTPServer

class Database:
    def __init__(self, path):
        self.con = sqlite3.connect(path)
        self.cur = self.con.cursor()
        self.cur.execute('CREATE TABLE IF NOT EXISTS configs \
                         (name text primary key,\
                         config text not null default "")')

    def save(self, name, config):
        self.cur.execute('INSERT OR REPLACE INTO configs VALUES (?, ?)', (name, json.dumps(config)))
        self.con.commit()

    def list(self):
        return [e[0] for e in self.cur.execute('SELECT name FROM configs').fetchall()]

    def load(self, name):
        result = self.cur.execute('SELECT config FROM configs WHERE name == ?', (name,)).fetchone()
        return json.loads(result[0])


class JSONRequestHandler(BaseHTTPRequestHandler):
    def _set_headers(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.send_header('Content-Type', 'application/json')
        self.end_headers()

    def do_HEAD(self):
        self._set_headers()

    def do_OPTIONS(self):
        self._set_headers()

    def do_POST(self):
        if self.path == '/save':
            data = self.rfile.read(int(self.headers['Content-Length']))
            data = json.loads(data)
            try:
                db.save(data['name'], data['config'])
                self._set_headers()
                self.wfile.write(json.dumps({'success': True}).encode('utf8'))
            except KeyError:
                self.send_response(400)
                self.end_headers()
                self.wfile.write(b'bad request')

        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b'not found')

    def do_GET(self):
        if self.path == '/list':
            self._set_headers()
            self.wfile.write(json.dumps({'configs': db.list()}).encode('utf8'))
        elif self.path.startswith('/load/'):
            name = self.path.split('/')[2]
            self._set_headers()
            print(db.load(name))
            self.wfile.write(json.dumps({'config': db.load(name)}).encode('utf8'))
        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b'not found')


db = Database('editor.db')

if __name__ == '__main__':
    port = 8000
    print('Running on port', port)
    server = HTTPServer(('0.0.0.0', port), JSONRequestHandler)
    server.serve_forever()