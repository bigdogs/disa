<script setup>
import LeftRight from "./components/LeftRight.vue";
import TopBottom from "./components/TopBottom.vue";
import CodeEditor from "./components/CodeEditor.vue";
import CodeDisplay from "./components/CodeDisplay.vue";
import Config from "./components/Config.vue";
// 属性选择器
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup

//https://stackoverflow.com/questions/35702705/is-there-a-way-to-change-the-css-resize-corners-position
</script>

<template>
  <LeftRight>
    <template #left>
      <Config />
    </template>
    <template #right>
      <TopBottom>
        <template #top>
          <CodeEditor />
        </template>
        <template #bottom>
          <CodeDisplay />
        </template>
      </TopBottom>
    </template>
  </LeftRight>
</template>

<style>
* {
  margin: 0;
  padding: 0;
}

#app {
  width: 100vw;
  min-width: 600px;
  height: 100vh;
  min-height: 200px;
}
</style>
