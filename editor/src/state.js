import api from './api';
import {reactive} from 'vue';
import {io} from "socket.io-client";

const state = reactive({
  items: {},
});

// Load data from backend
api.load().then(({items}) => {
  state.items = items;
});

// Sync updates across peers
const socket = io('http://' + document.domain + ':' + location.port);
socket.on('update', ({item}) => {
  state.items[item.id] = item;
});

export default state;
