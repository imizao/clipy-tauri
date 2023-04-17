<script setup lang="ts">
    import {reactive} from "vue"
    import {writeText, readText} from '@tauri-apps/api/clipboard';
    import Greet from "./components/Greet.vue";

    const state = reactive({
        textList: []
    });

    async function clipboardTextFn() {
        const clipboardText = await readText();
        console.log(clipboardText);
    }

    async function writeTextFn() {
        await writeText('Tauri is awesome!');
        const clipboardText = await readText();
        console.log(clipboardText);
    }

</script>

<template>
    <div class="container">
        <h1>Welcome to Tauri!</h1>

        <div class="row">
            <a href="https://vitejs.dev" target="_blank">
                <img src="/vite.svg" class="logo vite" alt="Vite logo" />
            </a>
            <a href="https://tauri.app" target="_blank">
                <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
            </a>
            <a href="https://vuejs.org/" target="_blank">
                <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
            </a>
        </div>

        <Greet />
        <div class="card">
            <button type="button" @click="clipboardTextFn()">clipboardTextFn</button>
            <button type="button" @click="writeTextFn()">writeTextFn</button>
        </div>
    </div>
</template>

<style scoped>
    .logo.vite:hover {
        filter: drop-shadow(0 0 2em #747bff);
    }

    .logo.vue:hover {
        filter: drop-shadow(0 0 2em #249b73);
    }

</style>
