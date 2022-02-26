// decode if left or button panel is displaying

import { ref } from "vue";

let hideLeft = ref(false);
let hideBottom = ref(false);

export default {
  hideLeft,
  hideBottom,
  toggleBottom() {
    hideBottom.value = !hideBottom.value;
  },
  toggleLeft() {
    hideLeft.value = !hideLeft.value;
  },
};
