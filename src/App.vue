<template>
  <div class="container">
    <n-layout position="absolute" has-sider>
      <n-layout content-style="padding: 24px;" :native-scrollbar="false">
        <n-list hoverable clickable>
          <!-- <Greet /> -->
          <n-list-item v-for="item in state.textList" @click="writeTextFn(item)" :key="item">
            {{ item }}
          </n-list-item>
        </n-list>
        <!-- <div class="card">
          <button type="button" @click="clipboardTextFn()">clipboardTextFn</button>
          <button type="button" @click="writeTextFn()">writeTextFn</button>
        </div> -->
      </n-layout>
    </n-layout>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from "vue";
// import Greet from "./components/Greet.vue";
import { writeText, readText } from "@tauri-apps/api/clipboard";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

const state = reactive({
  textList: [] as any,
});

async function listen_to_clipboard() {
  await listen("clipboard", (event: any) => {
    // event.payload 才是实际的结构体
    let timer;
    clearTimeout(timer);
    timer = setTimeout(() => {
      clipboardTextFn();
    },100)
  });
}

onMounted(() => {
  listen_to_clipboard();
});

async function clipboardTextFn() {
  const clipboardText = await readText();
  state.textList.push(clipboardText);
  // console.log(clipboardText);
}

async function writeTextFn(item: string) {
  await writeText(item);
  // const clipboardText = await readText();
  // console.log(clipboardText);
}
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
