const LOG_HOST = 'https://logs.half.earth';

/**
 * Generates a GUID string.
 * @returns {string} The generated GUID.
 * @example af8a8416-6e18-a307-bd9c-f2c947bbb3aa
 * @author Slavik Meltser.
 * @link http://slavik.meltser.info/?p=142
 */
function guid() {
    function _p8(s) {
        var p = (Math.random().toString(16)+"000000000").substr(2,8);
        return s ? "-" + p.substr(0,4) + "-" + p.substr(4,4) : p ;
    }
    return _p8() + _p8(true) + _p8(true) + _p8();
}

const sessionId = guid();

function post(endpoint, data) {
  if (VERSION !== 'dev') {
    return fetch(`${LOG_HOST}${endpoint}`, {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      method: 'POST',
      body: JSON.stringify(data)
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`Response ${res.status}`);
        }
        return res.json();
      });
  }
}

function startSession() {
  post('/session', {
    session_id: sessionId,
    version: VERSION
  });
}

function sendSnapshot(snapshot) {
  post('/snapshot', {
    session_id: sessionId,
    snapshot
  });
}

export {sessionId, startSession, sendSnapshot};