<script setup>
import Card from 'primevue/card';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
async function openimg() {
  msg.value = "请选择图片"
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'Image',
      extensions: ['png', 'jpeg']
    }]
  });

  if (selected === null) {
  } else {
    console.log(selected);
    msg.value = "Processing 正在处理中..."
    let b64 = await invoke("remove_bg_from_select", { path: selected });
    imgBase64.value = 'data:image/png;base64,' + b64;
    msg.value = "Right click image to save 请右键另存图片"
  }
}

const imgBase64 = ref("")
const msg = ref("")
</script>

<template>
  <Card @click="openimg">
    <template #title>Click here select image 点击此处选择文件</template>
    <template #content>
      {{ msg }}
    </template>
  </Card>
  <img :src="imgBase64">
</template>

<style scoped>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}



a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
