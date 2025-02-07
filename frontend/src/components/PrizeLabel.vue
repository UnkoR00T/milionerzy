<script setup lang="ts">

const props = defineProps({
  text: String,
  guaranteed: Boolean,
  cur: Boolean,
})

</script>

<template>
  <div :class="{cur: props.cur}">
    <h2 :class="{guaranteed: props.guaranteed, cur: props.cur}">{{ props.text }}</h2>
  </div>
</template>

<style scoped>
div{
  height: 55px;
  width: 700px;
  background-color: #2C59BA;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: calc(55px / 2);
  border-width: 5px;
  border-color: #808080;
  border-style: solid;
}
h2{
  font-size: 40px;
}
.guaranteed{
  color: #FF9800;
}
.cur{
  background-color: #FF9800;
  color: white !important;
}
</style>
