import os
from datetime import datetime, timezone
from werkzeug.utils import secure_filename

UPLOAD_PATH = 'uploads'
ALLOWED_EXTENSIONS = ['png', 'jpg', 'JPG', 'jpeg']

def allowed_file(fname):
    return '.' in fname and fname.rsplit('.', 1)[1] in ALLOWED_EXTENSIONS

def make_filename(filename):
    filename = secure_filename(filename)
    ext = filename.split('.')[-1]
    timestamp = datetime.utcnow().replace(tzinfo=timezone.utc).timestamp()
    filename = str(timestamp) + '.' + ext
    return filename

def save_image(file):
    if allowed_file(file.filename):
        filename = make_filename(file.filename)
        file.save(os.path.join(UPLOAD_PATH, filename))
        return filename

def image_path(fname):
    return os.path.join('..', UPLOAD_PATH), fname
