from flask import Flask
from .routes import bp, socketio

def create_app(package_name=__name__, static_folder='../static', template_folder='../templates', **config_overrides):
    app = Flask(package_name,
                static_url_path='/static',
                static_folder=static_folder,
                template_folder=template_folder)
    app.register_blueprint(bp)
    socketio.init_app(app)
    return app