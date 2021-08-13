import json
from flask import Blueprint, render_template, request, jsonify
from flask_socketio import SocketIO, emit
from .db import Database

bp = Blueprint('main', __name__)
db = Database('data.json')
socketio = SocketIO()

@bp.route('/')
def index():
    return render_template('index.html')

@bp.route('/data', methods=['GET', 'POST'])
def data():
    if request.method == 'POST':
        data = request.get_json()
        db[data['id']] = data
        db.save()
        emit('update', {'item': data}, namespace='/', broadcast=True)
        return jsonify(success=True)
    else:
        return jsonify(items=db.data)
