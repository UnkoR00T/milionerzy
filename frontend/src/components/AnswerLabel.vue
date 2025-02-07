<script setup lang="ts">
const props = defineProps({
  code: String,
  text: String,
  markdown: Boolean,
  correct: Boolean,
  removed: Boolean,
})
</script>

<template>
  <div class="answerLabel" :class="{markdown: props.markdown, correct: props.correct, removed: props.removed}">
    <div class="answerBorderSide"></div>
    <h2>
      {{props.code}}:
    </h2>
    <h3>
      {{props.text}}
    </h3>
    <div class="answerBorderSide"></div>
  </div>
</template>

<style scoped>
  .removed{
    color: #2C59BA !important;
  }
  .answerLabel{
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #2C59BA;
    width: 37dvw;
    position: relative;
    border-top: 10px solid #808080;
    border-bottom: 10px solid #808080;
  }
  .correct{
    background-color: #00D126 !important;
  }
  .correct .answerBorderSide{
    background-color: #00D126 !important;
  }
  h2{
    position: absolute;
    left: 0;
  }
  .markdown{
    background-color: #FF9800;
  }
  .answerBorderSide{
    height: 47px;
    width: 47px;
    background-color: #2C59BA;
    transform: rotate(45deg);
    border-width: 10px;
    border-color: #808080;
    border-style: solid;
    overflow: hidden;
    position: absolute;

  }
  .markdown .answerBorderSide{
    background-color: #FF9800;
  }
  .answerBorderSide:first-child{
    left: -29px;
    border-right: none;
    border-top: none;
  }
  .answerBorderSide:last-child{
    right: -29px;
    border-left: none;
    border-bottom: none;
  }
</style>
