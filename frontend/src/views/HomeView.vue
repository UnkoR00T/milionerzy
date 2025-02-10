<script setup lang="ts">
import { useWebsockets } from '@/stores/counter.ts'
import { useRouter } from 'vue-router'
import { useQuestions } from '@/stores/question.ts'

const ws = useWebsockets();
const router = useRouter();
const questions = useQuestions();
ws.webSocketConnection.on('startGame', (value) => {
  questions.setQuestion(value.questionId, value.question);
  router.push("/prize")
})
</script>

<template>
  <main>
    <img src="/joltamp.png" alt="Joltamp.pl logo">
    <div class="text">
      <p>Powered by</p>
      <h1>Joltamp.pl</h1>
    </div>
  </main>
</template>

<style scoped>
  img{
    height:128px;
    width:128px;
  }
  .text{
    height: 128px;
  }
  p{
    font-size: 24px;
    color: #686868;
    margin: 0;
    padding: 0;
    position: relative;
    top: 15px;
  }
  h1{
    font-size: 64px;
    color: #686868;
    margin: 0;
    padding: 0;
  }
  main{
    position:absolute;
    left: 50px;
    bottom: 50px;
    display: flex;
    gap: 15px;
  }
</style>
