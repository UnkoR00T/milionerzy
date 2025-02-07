<script setup lang="ts">

import PrizeLabel from '@/components/PrizeLabel.vue'
import { ref } from 'vue'
import { useWebsockets } from '@/stores/counter.ts'
import { useQuestions } from '@/stores/question.ts'
import { useRouter } from 'vue-router'

const ws = useWebsockets();
const router = useRouter();
const questions = useQuestions();
const curQuestion = ref(questions.curQuestion);

ws.webSocketConnection.on('changeScreen', (value) => {
  questions.setQuestion(value.questionId, value.question);
  curQuestion.value = value.questionId;
  if (!questions.screenLock) {
    router.push("/question")
  }
})
ws.webSocketConnection.on('nextQuestion', (value) => {
  questions.setQuestion(value.questionId, value.question);
  curQuestion.value = value.questionId;
})
</script>

<template>
  <main>
    <PrizeLabel text="10 Zupek" :guaranteed="true" :cur="curQuestion == 10" />
    <PrizeLabel text="9 Zupek" :guaranteed="false" :cur="curQuestion == 9" />
    <PrizeLabel text="8 Zupek" :guaranteed="false" :cur="curQuestion == 8" />
    <PrizeLabel text="7 Zupek" :guaranteed="false" :cur="curQuestion == 7"/>
    <PrizeLabel text="6 Zupek" :guaranteed="true"  :cur="curQuestion == 6"/>
    <PrizeLabel text="5 Zupek" :guaranteed="false" :cur="curQuestion == 5"/>
    <PrizeLabel text="4 Zupek" :guaranteed="false" :cur="curQuestion == 4"/>
    <PrizeLabel text="3 Zupek" :guaranteed="false" :cur="curQuestion == 3"/>
    <PrizeLabel text="2 Zupek" :guaranteed="true"  :cur="curQuestion == 2"/>
    <PrizeLabel text="1 Zupek" :guaranteed="false" :cur="curQuestion == 1"/>
  </main>
</template>

<style scoped>
  main{
    height: 100dvh;
    display:flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    gap: 10px;
    backdrop-filter: blur(5px);
  }
</style>
