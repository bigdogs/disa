<script setup>
import Ace from "./Ace.vue";
import config from "../data/config";
const setEditor = (e) => {
  config.codeGetter = () => {
    return e.getValue();
  };

  e.session.selection.on("changeCursor", () => {
    const p = e.getCursorPosition();
    config.editorCurrentRow = p.row;
    config.editorCurrentCol = p.column;
  });
};
</script>

<template>
  <Ace
    editorId="code-edit-id"
    :showGutter="true"
    :readOnly="false"
    :activeLine="true"
    @setEditor="setEditor"
  />
</template>

<style lang="scss"></style>
