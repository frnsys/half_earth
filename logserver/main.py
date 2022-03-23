import config
from db import Database
from flask_cors import CORS
from flask import Flask, request, jsonify
import sentry_sdk
from sentry_sdk.integrations.flask import FlaskIntegration

sentry_sdk.init(
    dsn=config.SENTRY_DSN,
    integrations=[FlaskIntegration()],

    # Set traces_sample_rate to 1.0 to capture 100%
    # of transactions for performance monitoring.
    # We recommend adjusting this value in production.
    traces_sample_rate=1.0
)

db = Database('logs.db')

app = Flask(__name__)
CORS(app)

@app.route('/session', methods=['POST'])
def session():
    if request.method == 'POST':
        data = request.get_json()
        ua = request.headers.get('User-Agent')
        db.add_session(data['session_id'], data['version'], ua)
        return jsonify(success=True)
    return jsonify(success=False)

@app.route('/snapshot', methods=['POST'])
def snapshot():
    if request.method == 'POST':
        data = request.get_json()
        db.add_snapshot(data['session_id'], data['snapshot'])
        return jsonify(success=True)
    return jsonify(success=False)


if __name__ == '__main__':
    app.run()