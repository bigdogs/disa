<script setup>
import { ref } from "vue";
const props = defineProps({
  isActive: {
    type: Boolean,
  },
  fontSize: {
    type: Number,
  },
  light: {
    type: Boolean,
    default: false,
  },
});
let hover = ref(false);
</script>

<template>
  <div
    :class="[
      {
        'xcode-button-active': isActive && !light,
        'xcode-button-active-light': isActive && light,
        'xcode-button-hover': hover && !isActive,
      },
      'xcode-button',
    ]"
    @mouseenter="hover = true"
    @mouseleave="hover = false"
    :style="{ 'font-size': fontSize + 'px' }"
  >
    <slot />
  </div>
</template>

<style>
.xcode-button {
  text-align: center;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
    Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  display: inline;
  padding: 2px 6px 3px 6px;
  border-radius: 4px;
  color: rgb(63, 60, 65);
}

.xcode-button-active {
  background: rgb(0, 122, 255);
  color: white;
}

.xcode-button-active-light {
  background: rgb(72, 161, 255);
  color: #fff;
}

.xcode-button-hover {
  background: rgb(214, 211, 216);
}
</style>
