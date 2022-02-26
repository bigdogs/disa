<script setup>
import { computed, h, onMounted, watchEffect } from "vue";
import useUIState from "../data/useUIState";
import RunButton from "./RunButton.vue";
import config from "../data/config";

let { hideBottom, toggleBottom, toggleLeft } = useUIState;

const locationText = computed(() => {
  if (config.editorCurrentRow == null || config.editorCurrentCol == null) {
    return "";
  }
  return `Line: ${config.editorCurrentRow} Col: ${config.editorCurrentCol}`;
});

watchEffect(() => {
  if (hideBottom.value) {
    const container = document.querySelector(".-tb-bottom-container");
    if (container) {
      container.style.height = "";
    }
  }
});

onMounted(() => {
  // min-height: 100, default: 120(or-last-height?)
  const dragger = document.querySelector(".-tb-line-drag");
  const container = document.querySelector(".-tb-bottom-container");

  // drag contant area is not working
  const statusBarHeight = parseInt(window.getComputedStyle(dragger).height);
  const minHeightContainer = statusBarHeight + 100 + 2;
  const defaultHeightContainer = statusBarHeight + 240;
  if (
    parseInt(window.getComputedStyle(container).height) < defaultHeightContainer
  ) {
    // on mounted, we set to default height
    container.style.height = defaultHeightContainer + "px";
  }

  let startY;
  let prevY;
  let startHeight;
  const startResize = (e) => {
    startY = e.screenY;
    prevY = e.screenY;
    startHeight = parseInt(window.getComputedStyle(container).height);
  };
  const resizeEvent = (e) => {
    const dy = startY - e.screenY;
    const newHeight = dy + startHeight;
    const up = e.screenY < prevY ? true : false;
    prevY = e.screenY;
    if (up) {
      if (hideBottom.value && newHeight > minHeightContainer - 20) {
        hideBottom.value = false;
      }
      if (!hideBottom.value) {
        container.style.height = Math.max(newHeight, minHeightContainer) + "px";
      }
    } else {
      // down
      if (newHeight < minHeightContainer - 20) {
        hideBottom.value = true;
      } else if (newHeight < minHeightContainer) {
        // [min - 20, min] is a buffer area we do nothing
        container.style.height = minHeightContainer + "px";
      } else {
        container.style.height = newHeight + "px";
      }
    }
  };
  const installEvent = () => {
    document.onselectstart = function () {
      return false;
    };
    document.body.style.cursor = "row-resize";
    document.body.addEventListener("mousemove", resizeEvent);
  };

  const removeEvent = () => {
    document.body.style.cursor = "auto";
    document.onselectstart = function () {
      return true;
    };
    document.body.removeEventListener("mousemove", resizeEvent);
  };
  dragger.addEventListener("mousedown", function (e) {
    startResize(e);
    installEvent();
    // mouseup event should be captured with windows level to make it works outside browser
    window.addEventListener("mouseup", function (ee) {
      removeEvent();
    });
  });
});
</script>

<template>
  <div class="-tb-container">
    <div class="-lb-top-container">
      <div class="-lb-top-container-bar">
        <RunButton />
      </div>
      <div class="-lb-top-container-content">
        <slot name="top"></slot>
      </div>
    </div>

    <div class="-tb-bottom-container">
      <div class="-tb-line-status">
        <svg class="-tb-line-button-left" @click="toggleLeft()">
          <rect width="12" height="12" fill="#ff0000" />
        </svg>
        <div class="-tb-line-drag">
          <div class="-tb-editor-location">
            {{ locationText }}
          </div>
        </div>
        <svg class="-tb-line-button-bottom" @click="toggleBottom()">
          <rect width="12" height="12" fill="#ff0000" />
        </svg>
      </div>
      <!-- TODO: 使用动画 -->
      <div class="-tb-bottom-content" v-if="!hideBottom">
        <slot name="bottom"></slot>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
.-tb-container {
  display: flex;
  flex-direction: column;
  height: 100%;

  .-lb-top-container {
    flex: 1;
    min-height: 120px;
    display: flex;
    flex-direction: column;
    .-lb-top-container-bar {
      height: 30px;
      width: 100%;
      border-top: 1px solid rgb(229, 229, 229);
      border-bottom: 1px solid rgb(229, 229, 229);
      display: flex;
      flex-direction: row;
      align-items: center;
      & > :first-child {
        margin-left: 10px;
      }
    }
    .-lb-top-container-content {
      margin: 2px 0;
      flex: 1;
    }
  }

  .-tb-bottom-container {
    display: flex;
    flex-direction: column;

    .-tb-line-status {
      width: 100%;
      height: 26px;
      border-top: 1px solid #eee;
      border-bottom: 1px solid #eee;
      display: flex;
      align-items: center;

      .-tb-line-button-left {
        width: 12px;
        height: 12px;
        margin-right: 5px;
        margin-left: 10px;
      }

      .-tb-line-drag {
        flex: 1;
        height: 100%;
        &:hover {
          cursor: row-resize;
        }
        display: flex;
        justify-content: end;
        align-items: center;
        .-tb-editor-location {
          position: relative;
          margin-right: 10px;
          color: rgb(170, 170, 170) !important;
          font-size: 12px;
          font-family: Menlo;
          font-weight: bold;
          &::after {
            content: "";
            position: absolute;
            top: 1px;
            bottom: 1px;
            margin-left: 7px;
            border-right: 1.5px solid rgb(230, 230, 230);
          }
        }
      }

      .-tb-line-button-bottom {
        width: 12px;
        height: 12px;
        margin-right: 10px;
        margin-left: 5px;
      }
    }

    .-tb-bottom-content {
      flex: 1;
      min-height: 100px;
    }
  }
}
</style>
