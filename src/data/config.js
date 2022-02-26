import { reactive } from "vue";

let state = reactive({
  activeLang: "armv7",
  direction: "assembly",
  editorCurrentRow: null,
  editorCurrentCol: null,
  codeGetter: null,
  resultSetter: null,
});

export default state;
