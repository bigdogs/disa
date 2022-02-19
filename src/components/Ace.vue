<script setup>
import { onMounted } from "vue";
import "../ace-min/ace";
import "../ace-min/theme-xcode";

const prop = defineProps({
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
});

onMounted(() => {
  const editor = ace.edit(prop.editorId);
  editor.setTheme("ace/theme/xcode");
  editor.setShowPrintMargin(false);
  editor.renderer.setShowGutter(prop.showGutter);
  editor.setReadOnly(prop.readOnly);
  editor.setHighlightActiveLine(prop.activeLine);
});
</script>

<template>
  <div class="code-editor" :id="prop.editorId"></div>
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
  background: rgba(236, 245, 255, 0.7) !important;
}

.ace_gutter-active-line {
  background: rgba(236, 245, 255, 0.7) !important;
}

.ace_gutter {
  background: white !important;
  color: rgb(194, 194, 194) !important;
  font-size: 12px;
  font-family: "Apple Color Emoji", SFMono-Regular, "SF Mono", ui-monospace,
    "DejaVu Sans Mono", Menlo, Consolas, monospace;
  line-height: 17px;
}
</style>
