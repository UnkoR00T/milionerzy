<script setup lang="ts">

import AnswerLabel from '@/components/AnswerLabel.vue'
import { ref } from 'vue'
import { useQuestions } from '@/stores/question.ts'
import { useRouter } from 'vue-router'
import { useWebsockets } from '@/stores/counter.ts'

const questions = useQuestions();
const router = useRouter();
const ws = useWebsockets();

const correct = ref(0);
const markdown = ref(0);
const question = ref(questions.textQuestion);
const answers = ref([questions.answerA,questions.answerB,questions.answerC,questions.answerD])
const removed = ref([false, false, false, false])

ws.webSocketConnection.on('changeScreen', (value) => {
  questions.setQuestion(value.questionId, value.question);
  if (!questions.screenLock) {
    router.push("/prize")
  }else{
    correct.value = 0;
    markdown.value = 0;
    question.value = value.question.question;
    answers.value = [value.question.answerA, value.question.answerB, value.question.answerC, value.question.answerD];
  }
})

ws.webSocketConnection.on("answerQuestionSelect", (value) => {
  markdown.value = value;
})
ws.webSocketConnection.on("answerQuestionFinal", (value) => {
  correct.value = value.correct;
})
ws.webSocketConnection.on('nextQuestion', (value) => {
  questions.setQuestion(value.questionId, value.question);
  correct.value = 0;
  markdown.value = 0;
  question.value = value.question.question;
  answers.value = [value.question.answerA, value.question.answerB, value.question.answerC, value.question.answerD]
})
ws.webSocketConnection.on("useResque", (index) => {
  questions.setUsedResque(index);
  if(index == 1){
    let removedCount = 0;
    for(let i = 0; i < 4; i++){
      if (removedCount === 2){
        break;
      }
      if (questions.correctAnswer != i + 1){
        removed.value[i] = true;
        removedCount++;
      }
    }
  }
})
</script>

<template>
  <main>
    <div class="resque">
      <img src="/people.png" alt="People" class="resque-icon" :class="{used: questions.usedResques[0]}">
      <img src="/half.png" alt="50 50" class="resque-icon" :class="{used: questions.usedResques[1]}">
      <img src="/telephone.png" alt="Telephone" class="resque-icon" :class="{used: questions.usedResques[2]}">
    </div>
    <div class="questionLabel">
      <div class="questionBorderSide"></div>
      <h1>{{question}}</h1>
      <div class="questionBorderSide"></div>
    </div>
    <div class="answersHolder">
      <div class="answersHorizontal">
        <AnswerLabel code="A" :markdown="markdown == 1" :correct="correct == 1" :text="answers[0]" :removed="removed[0]"/>
        <AnswerLabel code="B" :markdown="markdown == 2" :correct="correct == 2" :text="answers[1]" :removed="removed[1]"/>
      </div>
      <div class="answersHorizontal">
        <AnswerLabel code="C" :markdown="markdown == 3" :correct="correct == 3" :text="answers[2]" :removed="removed[2]"/>
        <AnswerLabel code="D" :markdown="markdown == 4" :correct="correct == 4" :text="answers[3]" :removed="removed[3]"/>
      </div>
    </div>
  </main>
</template>

<style scoped>
img.resque-icon{
  width: 50px;
  height: 50px;
  padding: 10px;
  background-color: #2C59BA;
  margin: 5px;
  border-radius: 50%;
  border-color: #808080;
  border-width: 10px;
  border-style: solid;
}
.used{
  background-color: #ba2c2c !important;
}
.resque{
  align-self: end;
  margin-right: 8dvw;
  margin-bottom: 10px;
}
h1{
  margin: 0;
  padding: 0;
}
main {
  height: 100dvh;
  width: 100dvw;
  display:flex;
  flex-direction: column;
  justify-content: flex-end;
  align-items: center;
  transform: translateY(-10px);
}
.questionLabel{
  z-index: 10;
  width: 80dvw;
  height: 100px;
  background-color: #2C59BA;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  border-top-width: 10px;
  border-top-color: #808080;
  border-top-style: solid;

  border-bottom-width: 10px;
  border-bottom-color: #808080;
  border-bottom-style: solid;
  margin-bottom: 10px;
}
.questionBorderSide{
  z-index: -1;
  height: 73px;
  width: 73px;
  background-color: #2C59BA;
  transform: rotate(45deg);
  border-width: 10px;
  border-color: #808080;
  border-style: solid;
  overflow: hidden;
}
.questionBorderSide:first-child{
  position: absolute;
  left: -42px;
  border-right: none;
  border-top: none;
}
.questionBorderSide:last-child{
  position: absolute;
  right: -42px;
  border-left: none;
  border-bottom: none;
}
.answersHolder{
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.answersHorizontal{
  display: flex;
  gap: 100px;
}
</style>
