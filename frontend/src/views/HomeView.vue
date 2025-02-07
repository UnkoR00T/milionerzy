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

  </main>
</template>

<style scoped>

</style>
