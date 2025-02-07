import { defineStore } from 'pinia'
import io from 'socket.io-client'
import { useRouter } from 'vue-router'

export const useWebsockets = defineStore('ws', () => {
  const webSocketConnection = io("wss://joltamp.pl/", {transports: ["websocket", "polling"], withCredentials: true});
  webSocketConnection.on("connect", () => {
    const cos = prompt("Enter game code:");
    webSocketConnection.emitWithAck("connectConsumer", cos).then((response) => {
      if (response) {
        alert("Connected to a websocket")
      }else{
        alert("Cannot connect to a websocket")
      }
    });
  })

  webSocketConnection.on('reconnect', () => {
    alert("Reconnecting");
  })

  return {webSocketConnection}

})
