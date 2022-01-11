import api from './api';
import {reactive} from 'vue';
import {io} from "socket.io-client";

const state = reactive({
  items: {},
  itemsByType: {},
});

// Load data from backend
api.load().then((items) => {
  Object.keys(items).forEach((id) => {
    if (items[id].deleted) delete items[id];
  });

  state.items = items;

  let itemsByType = {};
  Object.values(state.items).forEach((item) => {
    if (!(item._type in itemsByType)) {
      itemsByType[item._type] = {};
    }
    itemsByType[item._type][item.id] = item;
  });
  state.itemsByType = itemsByType;
});

// Sync updates across peers
const socket = io('http://' + document.domain + ':' + location.port);
socket.on('update', ({item}) => {
  if (item.deleted) {
    delete state.items[item.id];
    delete state.itemsByType[item._type][item.id];
  } else {
    state.items[item.id] = item;
    state.itemsByType[item._type][item.id] = item;
  }
});

export default state;
