<script setup>
import { run } from "../action/code";
</script>

<template>
  <svg @click="run()" class="-run-button">
    <rect width="24" height="24" fill="#ff0000" />
  </svg>
</template>

<style>
.-run-button {
  width: 24px;
  height: 24px;
}
</style>
