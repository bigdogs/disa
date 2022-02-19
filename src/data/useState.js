import { ref } from "vue";

let hideLeft = ref(false);
let hideBottom = ref(false);

export default {
  hideLeft,
  hideBottom,
  toggleBottom() {
    hideBottom.value = !hideBottom.value;
  },
};
