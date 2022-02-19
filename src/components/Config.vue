<script setup>
import XButton from "./XButton.vue";
import config from "../data/config";
</script>

<template>
  <div class="config">
    <div class="config-direction">
      <XButton
        :fontSize="12"
        :isActive="config.direction === 'assembly'"
        @click="config.direction = 'assembly'"
      >
        Assembly
      </XButton>
      <XButton
        :fontSize="12"
        :isActive="config.direction === 'disassembly'"
        @click="config.direction = 'disassembly'"
      >
        Disassembly
      </XButton>
    </div>
    <div class="config-hr" />
    <div class="config-lang">
      <XButton
        :fontSize="15"
        class="config-lang-button"
        :isActive="config.activeLang === 'armv7'"
        @click="config.activeLang = 'armv7'"
        :light="true"
      >
        armv7
      </XButton>
      <XButton
        :fontSize="15"
        class="config-lang-button"
        :isActive="config.activeLang === 'thumb'"
        @click="config.activeLang = 'thumb'"
        :light="true"
      >
        thumb
      </XButton>
      <XButton
        :fontSize="15"
        class="config-lang-button"
        :isActive="config.activeLang === 'aarch64'"
        @click="config.activeLang = 'aarch64'"
        :light="true"
      >
        aarch64
      </XButton>
      <XButton
        :fontSize="15"
        class="config-lang-button"
        :isActive="config.activeLang === 'x86'"
        @click="config.activeLang = 'x86'"
        :light="true"
      >
        x86
      </XButton>
      <XButton
        :fontSize="15"
        class="config-lang-button"
        :isActive="config.activeLang === 'x86_64'"
        @click="config.activeLang = 'x86_64'"
        :light="true"
      >
        x86_64
      </XButton>
    </div>
  </div>
</template>

<style lang="scss">
.config {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  .config-hr {
    box-sizing: border-box;
    width: 100%;
    height: 1px;
    border-top: 1px solid rgb(202, 200, 204);
  }

  .config-direction {
    display: flex;
    margin: 5px 0;
    & > * {
      margin-left: 4px;
    }
  }
  .config-lang {
    margin-top: 6px;
    //补偿border
    margin-left: 2.5px;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    .config-lang-button {
      width: 80%;
      margin-top: 2px;
    }
  }
}
</style>
