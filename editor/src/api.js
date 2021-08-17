function load() {
  return fetch('/data', {
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    method: 'GET'
  })
    .then((res) => {
      if (!res.ok) {
        throw new Error(`Response ${res.status}`);
      }
      return res.json();
    });
}

function update(item) {
  return fetch('/data', {
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    method: 'POST',
    body: JSON.stringify(item)
  })
    .then((res) => {
      if (!res.ok) {
        throw new Error(`Response ${res.status}`);
      }
      return res.json();
    });
}

export default {load, update};
