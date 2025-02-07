import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useQuestions = defineStore('questions', () => {
  const curQuestion = ref(1);
  const textQuestion = ref("Loading");
  const answerA = ref("Loading");
  const answerB = ref("Loading");
  const answerC = ref("Loading");
  const answerD = ref("Loading");
  const correctAnswer = ref(0);
  const usedResques = ref([false, false, false]);

  const screenLock = ref(false);

  const setQuestion = (id: number, value: {question: string, answerA: string, answerB: string, answerC: string, answerD: string, correctAnswer: number}) => {
    curQuestion.value = id + 1;
    textQuestion.value = value.question;
    answerA.value = value.answerA;
    answerB.value = value.answerB;
    answerC.value = value.answerC;
    answerD.value = value.answerD;
    correctAnswer.value = value.correctAnswer;
  }

  const flapScreenLock = () => {
    screenLock.value = !screenLock.value;
  }

  const setUsedResque = (index: number) => {
    usedResques.value[index] = true;
  }

  return {curQuestion, textQuestion, answerA, answerB, answerC, answerD, correctAnswer, setQuestion, screenLock, flapScreenLock, usedResques, setUsedResque};

})
