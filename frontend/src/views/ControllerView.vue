<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref } from 'vue'
import { useWebsockets } from '@/stores/counter.ts'

const code = prompt("Enter game code for a controller:");
const router = useRouter();
const ws = useWebsockets();
const usedResques = ref([false, false, false]);
const setUsedResque = (index: number) => {
  usedResques.value[index] = true;
}

const btn = ref(0);
const correct = ref(0);

const clickSelect = (btnNew: number) => {
  btn.value = btnNew;
}

const clickSendSelect = () => {
  if (btn.value != 0){
    ws.webSocketConnection.emitWithAck("answerQuestion", {
      gameid: code,
      selected: btn.value
    }).then((res) => {
      if (!res) {
        alert("Error while selecting answer!")
      }
    });
  }
}
ws.webSocketConnection.on("answerQuestionSelect", (value) => {
  btn.value = value;
  alert(`Selected ${value}`);
})
ws.webSocketConnection.on("answerQuestionFinal", (value) => {
  console.log(value);
  correct.value = value.correct;
})
ws.webSocketConnection.on('nextQuestion', (value) => {
  btn.value = 0;
  correct.value = 0;
})
ws.webSocketConnection.on('changeScreen', (value) => {
  btn.value = 0;
  correct.value = 0;
})
ws.webSocketConnection.on("startGame", (value) => {
  alert("Game started");
})
const useResque = (index: number) => {
  if(confirm("Are you sure you want to use that?")){
    ws.webSocketConnection.emitWithAck("useResque", {gameId: code, index: index}).then((res) => {
      if (res) {
        setUsedResque(index);
        alert("Rescue used!");
      }
    })
  }
}
</script>

<template>
  <main>
    <div class="answerSelector">
      <div class="answerSelectorRow" style="margin-bottom: 25px">
        <button class="answerSelectSingle" @click="ws.webSocketConnection.emit('startGame', code)">
          Start game
        </button>
      </div>
      <div class="answerSelectorRow">
        <button class="answerSelect" @click="clickSelect(1)" :class="{selected: btn == 1, correct: correct == 1}">A</button>
        <button class="answerSelect" @click="clickSelect(2)" :class="{selected: btn == 2, correct: correct == 2}">B</button>
      </div>
      <div class="answerSelectorRow">
        <button class="answerSelect" @click="clickSelect(3)" :class="{selected: btn == 3, correct: correct == 3}">C</button>
        <button class="answerSelect" @click="clickSelect(4)" :class="{selected: btn == 4, correct: correct == 4}">D</button>
      </div>

      <div class="answerSelectorRow">
        <button class="answerSelectSingle" @click="clickSendSelect">
          Select
        </button>
      </div>
      <div class="answerSelectorRow" style="margin-top: 25px">
        <button class="answerSelectSingle" @click="ws.webSocketConnection.emit('changeScreen', code)">
          Change screen
        </button>
      </div>
      <div class="answerSelectorRow">
        <button class="answerSelectSingle" @click="ws.webSocketConnection.emit('nextQuestion', code)">
          Next question
        </button>
      </div>
      <div class="answerSelectorRow">
        <button class="answerSelectSingle" @click="ws.webSocketConnection.emit('testConnection', code)">
          Test
        </button>
      </div>
      <div class="answerSelectorRow">
        <button class="answerSelect answerSelectTripple" @click="useResque(0)">
          P
        </button>
        <button class="answerSelect answerSelectTripple" @click="useResque(1)">
          50
        </button>
        <button class="answerSelect answerSelectTripple" @click="useResque(2)">
          T
        </button>
      </div>
    </div>
  </main>
</template>

<style scoped>
.answerSelect{
  height: 80px;
  width: 80px;
  margin: 0;
  padding: 0;
}
.answerSelectTripple{
  width: calc(160px / 3) !important;
  height: 65px;
}
.correct{
  background-color: #00D126 !important;
}
.answerSelectSingle {
  height: 80px;
  width: 160px;
}
.selected {
  background-color: #FF9800;
}
main {
  display: flex;
  justify-content: center;
}

</style>
