<script setup>
import { h, onMounted, watchEffect } from "vue";
import useState from "../data/useState";

let { hideBottom, toggleBottom } = useState;

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
  const defaultHeightContainer = statusBarHeight + 120;
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
      <slot name="top"></slot>
    </div>

    <div class="-tb-bottom-container">
      <div class="-tb-line-status">
        <div class="-tb-line-drag" />
        <svg class="-tb-line-button" @click="toggleBottom()">
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

      .-tb-line-drag {
        flex: 1;
        height: 100%;
        &:hover {
          cursor: row-resize;
        }
      }

      .-tb-line-button {
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
