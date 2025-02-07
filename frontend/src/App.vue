<script setup lang="ts">

import { useWebsockets } from '@/stores/counter.ts'

const ws = useWebsockets();
ws.webSocketConnection.on('testConnection', ()=>{
  alert("Connection tested");
})
</script>

<template>
  <div class="backgroundBoxImg">
    <div class="backgroundBoxOverlay">
    </div>
  </div>
  <div class="gameView">
    <RouterView />
  </div>
</template>

<style scoped>
*{
  color: white;
}
.backgroundBoxImg {
  height: 100dvh;
  width: 100dvw;
  position: absolute;
  background-size: cover;
  background-position: center;
  background-image: url('/milionerzybg.jpg');
}
.backgroundBoxOverlay {
  background-color: rgba(0, 0, 0, 0.25);
  height: 100dvh;
  width: 100dvw;
  -webkit-box-shadow: inset 0px 0px 80px 4px rgba(0, 0, 0, 1);
  -moz-box-shadow: inset 0px 0px 80px 4px rgba(0, 0, 0, 1);
  box-shadow: inset 0px 0px 80px 4px rgba(0, 0, 0, 1);
}
.gameView{
  position:absolute;
  width: 100dvw;
  height: 100dvh;
}
</style>
