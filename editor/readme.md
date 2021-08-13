- Compile frontend: `npm start`
- Run server: `gunicorn server:app --worker-class geventwebsocket.gunicorn.workers.GeventWebSocketWorker --bind 127.0.0.1:8000`

