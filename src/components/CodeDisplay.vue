<script setup>
import config from "../data/config";
import Ace from "./Ace.vue";

const setEditor = (e) => {
  config.resultSetter = (data) => {
    return e.setValue(data, -1);
  };
};
</script>

<template>
  <Ace
    editorId="code-display-id"
    :showGutter="false"
    :readOnly="true"
    :activeLine="false"
    @setEditor="setEditor"
  />
</template>

<style lang="scss"></style>
