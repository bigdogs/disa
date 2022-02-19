<script setup>
import Ace from "./Ace.vue";
</script>

<template>
  <Ace
    editorId="code-display-id"
    :showGutter="false"
    :readOnly="true"
    :activeLine="false"
  />
</template>

<style lang="scss"></style>
