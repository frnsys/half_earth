from db import Database
from datetime import datetime

db = Database('logs.db')

print('Recent Sessions:')
print('-'*50)
sessions = db.sessions()
for session in sessions[:5]:
    print(session['id'])
    print(' ', datetime.fromtimestamp(float(session['timestamp'])))
    print(' ', session['useragent'])

# print('='*50)
# print('Most recent session:')
# session_id = sessions[0]['id']
# for year in db.snapshots(session_id):
#     print(year)