from .db import Database
from .image import save_image, image_path
from flask_socketio import SocketIO, emit
from flask import Blueprint, render_template, request, jsonify, abort, send_from_directory

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

@bp.route('/image', methods=['POST'])
def upload_image():
    if request.method == 'POST':
        # If an image was submitted, save it
        if request.files.get('image'):
            filename = save_image(request.files['image'])
            if filename is None:
                abort(400)
            return jsonify(filename=filename)
    else:
        abort(400)

@bp.route('/image/<fname>')
def image(fname):
    return send_from_directory(*image_path(fname))
