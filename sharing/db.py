import json
from flask_sqlalchemy import SQLAlchemy
from datetime import datetime, timezone

db = SQLAlchemy()

class Session(db.Model):
    __tablename__ = 'sessions'
    id = db.Column(db.String(255), primary_key=True)
    timestamp = db.Column(db.String(255), nullable=False)
    data = db.Column(db.JSON, nullable=False)

def save_session(id, summary):
    timestamp = datetime.utcnow().replace(tzinfo=timezone.utc).timestamp()
    session = Session(id=id, timestamp=timestamp, data=json.dumps(summary))
    db.session.add(session)
    db.session.commit()

def get_session(id):
    return Session.query.get_or_404(id)

def find_sessions(id):
    return Session.query.filter_by(id=id).all()
