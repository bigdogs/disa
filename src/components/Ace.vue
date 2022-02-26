<script setup>
import { onMounted, ref } from "vue";
import "../ace-min/ace";
import "../ace-min/theme-xcode";

const props = defineProps({
  showGutter: {
    type: Boolean,
  },
  readOnly: {
    type: Boolean,
  },
  editorId: {
    type: String,
  },
  activeLine: {
    type: Boolean,
  },
  editor: {
    type: Object,
  },
});

const emit = defineEmits(["setEditor"]);

onMounted(() => {
  // https://ace.c9.io/#nav=howto
  const editor = ace.edit(props.editorId);
  editor.setTheme("ace/theme/xcode");
  editor.setShowPrintMargin(false);
  editor.renderer.setShowGutter(props.showGutter);
  editor.setReadOnly(props.readOnly);
  editor.setHighlightActiveLine(props.activeLine);
  emit("setEditor", editor);
});
</script>

<template>
  <div class="code-editor" :id="props.editorId"></div>
</template>

<style lang="scss">
.code-editor {
  width: 100%;
  height: 100%;
  resize: none;
  font-family: "SF Mono", SFMono-Regular, ui-monospace, "DejaVu Sans Mono",
    Menlo, Consolas, monospace;
  font-size: 14px;
}

.ace_active-line {
  background: rgba(245, 246, 247, 0.7) !important;
}

.ace_gutter-active-line {
  background: rgba(245, 246, 247, 0.7) !important;
}

.ace_gutter-cell {
  left: auto;
}

.ace_gutter {
  background: white !important;
  color: rgb(194, 194, 194) !important;
  font-size: 12px;
  font-family: "DejaVu Sans Mono", Menlo, Consolas, monospace;
  line-height: 17px;
}

.ace_cursor {
  border-left: 1.3px solid;
}
</style>
