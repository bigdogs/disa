<script setup>
import Ace from "./Ace.vue";
</script>

<template>
  <Ace
    editorId="code-edit-id"
    :showGutter="true"
    :readOnly="false"
    :activeLine="true"
  />
</template>

<style lang="scss"></style>
