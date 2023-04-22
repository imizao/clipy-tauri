<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from '@tauri-apps/api/event'

const greetMsg = ref("");
const name = ref("");

function greet() {
  emit("greet", { name: name.value });
}

// async function listen_to_greet() {
//   const unlisten = await listen('hello', (event: any) => {
//     // event.payload 才是实际的结构体
//     greetMsg.value = event.payload;
//     console.log(greetMsg.value);
//   });
// }

// onMounted(() => {
//   listen_to_greet();
// })
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>