/*
 * A simple RPC class wrapper for web workers.
 *
 * Usage:
 *  - In your worker file, call RPC.prepare(YourClass);
 *  - In your main file, call RPC.initialize(yourWorker);
 */

const TYPE = Object.freeze({
  READY: 1,
  NEW:   2,
  GET:   3,
  SET:   4,
  CALL:  5,
});


// Make a request to the worker and
// wait for the matching response.
let nextReqId = 0;
function requestResponse(worker, data) {
  // Send the request to the worker,
  // with a request id to match
  // to the correct response.
  let thisReqId = nextReqId;
  worker.postMessage({
    reqId: thisReqId, ...data
  });
  nextReqId++;

  return new Promise((resolve, reject) => {
    // Detach the listener once we get
    // the matching response.
    let handler = ({data}) => {
      let {reqId, resp} = data;
      if (reqId == thisReqId) {
        resolve(resp);
        worker.removeEventListener('message', handler);
      }
    };
    worker.addEventListener('message', handler);
  });
}


// Create a proxy for the class instance
// that the main thread can use.
function createProxy(worker, id, methods) {
  return new Proxy({}, {
    get(_target, key, _receiver) {
      // The way promises work is that after
      // you return them, they try to lookup `next` on
      // whatever they resolve to (e.g. this proxy),
      // so just ignore that.
      if (key == 'then') return;

      // If this key matches a known method,
      // return a function to call.
      if (methods.has(key)) {
        return function() {
          return requestResponse(worker, {
            id, key, args: [...arguments],
            type: TYPE.CALL
          });
        };

      // Otherwise get the value.
      } else {
        return requestResponse(worker, {
          id, key,
          type: TYPE.GET
        });
      }
    },

    // Setting is weird, you can't use `then` afaik.
    // But you can do `await (proxy.foo = 'bar')`
    set(_target, key, val) {
      return requestResponse(worker, {
        id, val, key,
        type: TYPE.SET,
      });
    }
  });
}


// Waits for the worker to be ready.
// This assumes that once it's ready
// its ready state doesn't change.
function waitReady(worker) {
  return new Promise((resolve, reject) => {
    if (worker.ready) {
      resolve();
    } else {
      const handler = ({data}) => {
        if (data.ready) {
          worker.ready = true;
          worker.removeEventListener('message', handler);
          clearInterval(poll);
          resolve();
        }
      }
      worker.addEventListener('message', handler);
      let poll = setInterval(() => {
        worker.postMessage({
          type: TYPE.READY
        });
      }, 10);
    }
  });
}


// Initialize the worker RPC
// interface on the main thread.
function initialize(worker) {
  return new Proxy(() => {}, {
    construct(_target, args) {
      return waitReady(worker).then(() => {
        return requestResponse(worker, {
          type: TYPE.NEW,
          args: args
        });
      }).then(({id, methods}) => {
        return createProxy(worker, id, methods);
      });
    }
  });
}


// Prepare the wrapped class in the worker.
function prepare(cls) {
  let nextId = 0;
  const instances = {};

  // Let the RPC proxy know what keys are methods
  // Probably a better way to do this?
  let methods = new Set(Object.getOwnPropertyNames(cls.prototype));
  addEventListener('message', (msg) => {
    const {reqId, type, ...data} = msg.data;

    switch (type) {
      // Check that this worker is ready.
      case TYPE.READY: {
        postMessage({ready: true});
      } break;

      // Create a new instance of the wrapped class.
      case TYPE.NEW: {
        let {args} = data;
        instances[nextId] = new cls(...args);
        postMessage({reqId, resp: {id: nextId, methods}});
        nextId++;
      } break;

      // Get a property from the wrapped class instance.
      case TYPE.GET: {
        const {id, key} = data;
        let val = instances[id][key];
        postMessage({reqId, resp: val});
      } break;

      // Set a property on the wrapped class instance.
      case TYPE.SET: {
        const {id, key, val} = data;
        instances[id][key] = val;
        postMessage({reqId, resp: val});
      } break;

      // Call a method on the wrapped class instance.
      case TYPE.CALL: {
        const {id, key, args} = data;
        let ret = instances[id][key](...args);
        postMessage({reqId, resp: ret});
      } break;
    }
  });
}


export default {initialize, prepare};
