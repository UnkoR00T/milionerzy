<script setup lang="ts">

import { useWebsockets } from '@/stores/counter.ts'
import {type Ref, ref} from 'vue'

const ws = useWebsockets()

const question = ref("");
const answerA = ref("");
const answerB = ref("");
const answerC = ref("");
const answerD = ref("");
const correctAnswer = ref(0);

const promptGameId = () => {
  const g = prompt('Enter game id:');
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-expect-error
  localStorage.setItem('gameId', g);
  return g;
}
const questions: Ref<{
  id: number,
  question: string,
  answerA: string,
  answerB: string,
  answerC: string,
  answerD: string,
  correctAnswer: number
}[]> = ref([]);
const gameId = localStorage.getItem('gameId') ? localStorage.getItem('gameId') : promptGameId();

const fetchQuestions = () => {
  ws.webSocketConnection.emitWithAck('getQuestions', gameId).then((res) => {
    questions.value = res;
  })
}

const addQuestion = () => {
  const data = {
    gameId: gameId,
    question: question.value,
    answerA: answerA.value,
    answerB: answerB.value,
    answerC: answerC.value,
    answerD: answerD.value,
    correctAnswer: correctAnswer.value
  }
  ws.webSocketConnection.emit('addQuestion', data);
  setTimeout(() => fetchQuestions(), 100);
  question.value = "";
  answerA.value = "";
  answerB.value = "";
  answerC.value = "";
  answerD.value = "";
  correctAnswer.value = 0;
}

const getCharfromInt = (i: number) => {
  switch (i) {
    case 1: return 'A';
    case 2: return 'B';
    case 3: return 'C';
    case 4: return 'D';
    default: return '';
  }
}

const deleteQuestion = (id: number) => {
  ws.webSocketConnection.emit('deleteQuestion', {
    id: id,
    gameId: gameId
  });
  setTimeout(() => fetchQuestions(), 100);
}

fetchQuestions();

</script>

<template>
  <main>
    <div class="questions">
      <h4>{{gameId}} <span @click="promptGameId()">Change gameid</span></h4>
      <div class="question" v-for="question in questions">
        <p>
          <button @click="deleteQuestion(question.id)">Del</button>
          {{question.id}}. {{ question.question }} A: {{question.answerA}} B: {{question.answerB}} C: {{question.answerC}} D: {{question.answerD}}
          <b>Poprawna {{getCharfromInt(question.correctAnswer)}}</b>
        </p>
      </div>
    </div>

    <div class="form">
      <input type="text" placeholder="Question" v-model="question">
      <input type="text" placeholder="Answer A" v-model="answerA">
      <input type="text" placeholder="Answer B" v-model="answerB">
      <input type="text" placeholder="Answer C" v-model="answerC">
      <input type="text" placeholder="Answer D" v-model="answerD">
      <input type="number" name="Correct" id="" placeholder="Correct answer 1-4" v-model="correctAnswer">
      <button @click="addQuestion">Add</button>
    </div>
  </main>
</template>

<style scoped>
main{
  display: flex;
  flex-direction: column;
  align-items: center;
  height: 100vh;
}
.form{
  display: flex;
  flex-direction: column;
  width: 400px;
}
</style>
