<script setup>
import { onMounted, onUpdated } from "vue";
import useUIState from "../data/useUIState";

let { hideLeft } = useUIState;

const installEvent = () => {
  let resizeBox = document.querySelector(".-lr-l-resize");
  let leftContainer = document.querySelector(".-lr-left-container");
  if (!resizeBox || !leftContainer) {
    return;
  }

  let startX;
  let startWidth;
  const startResize = function (e) {
    startX = e.screenX;
    startWidth = parseInt(window.getComputedStyle(leftContainer).width);
  };

  let resizeEvent = function (e) {
    let newWidth = e.screenX - startX + startWidth;
    if (newWidth < 60) {
      hideLeft.value = true;
      removeEvent();
    } else if (newWidth < 200) {
      leftContainer.style.width = "200px";
    } else {
      leftContainer.style.width = newWidth + "px";
    }
  };

  let installEvent = function () {
    document.onselectstart = function () {
      return false;
    };
    document.body.style.cursor = "col-resize";
    document.body.addEventListener("mousemove", resizeEvent);
  };

  let removeEvent = function () {
    document.body.style.cursor = "auto";
    document.onselectstart = function () {
      return true;
    };
    document.body.removeEventListener("mousemove", resizeEvent);
  };

  resizeBox.addEventListener("mousedown", function (e) {
    console.log("mouse down");
    startResize(e);
    installEvent();
    document.body.addEventListener("mouseup", function () {
      removeEvent();
    });
  });
};

// as left part can be hidden or show dynamicly depends on current state
// need install event on both  updated and mounted
onUpdated(() => {
  installEvent();
});

onMounted(() => {
  installEvent();
});
</script>

<template>
  <div class="-lr-container">
    <div class="-lr-left-container" v-if="!hideLeft">
      <div class="-lr-l-content">
        <slot name="left"></slot>
      </div>
      <div class="-lr-l-resize"></div>
    </div>

    <div class="-lr-right-container">
      <slot name="right"></slot>
    </div>
  </div>
</template>

<style lang="scss">
.-lr-container {
  display: flex;
  height: 100%;

  .-lr-left-container {
    display: flex;
    width: 240px;
    background: rgba(225, 222, 229, 0.4);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);

    .-lr-l-content {
      flex: 1;
    }

    .-lr-l-resize {
      width: 2px;
      // background: red;
      &:hover {
        cursor: col-resize;
      }
    }
  }

  .-lr-right-container {
    flex: 1;
    min-width: 180px;
  }
}
</style>
