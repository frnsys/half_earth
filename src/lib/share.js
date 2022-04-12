import state from '/src/state';


let SHARE_SERVER = 'http://localhost:5000/';
if (process.env.NODE_ENV === 'production') {
  SHARE_SERVER = 'https://history.half.earth/';
}

function share(win, cb)  {
  state.gameState.win = win;
  post(SHARE_SERVER, state.gameState, (data) => {
    cb(data);
  });
}

function post(url, data, onSuccess, onErr) {
  return fetch(url, {
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    credentials: 'same-origin',
    method: 'POST',
    body: JSON.stringify(data)
  })
    .then(res => res.json())
    .then((data) => onSuccess && onSuccess(data))
    .catch(err => {
      if (onErr) {
        onErr(err);
      } else {
        throw err;
      }
    });
}


export default share;
