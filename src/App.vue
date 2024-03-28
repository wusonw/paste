<script setup>
import Scrollbar from 'smooth-scrollbar';
import { ref, onMounted } from 'vue';
import { writeText, readText } from '@tauri-apps/api/clipboard';
import { Store } from "tauri-plugin-store-api";

const STORED_COUNT = 30
const store = new Store(".clip-store.dat");
const clipped = ref([])

const loadClippedFromStore = async () => {
  const storedClipped = await store.get('clipped')
  clipped.value = (storedClipped || [])
    .toSorted((a, b) => b?.time > a?.time)
}

const watchClipboard = () => {
  setInterval(async () => {
    const time = new Date()
    const currentClipped = await readText()
    const clickedClipped = await store.get('clicked')
    if (currentClipped === clickedClipped) return
    const storedClipped = (await store.get('clipped') || []).toSorted((a, b) => b?.time > a?.time)
    const storeCurrentClipped = async () => {
      let clippedToStore = storedClipped.slice(0, STORED_COUNT)
      await store.set('clipped',
        [...clippedToStore, {
          type: 'TXT',
          value: currentClipped,
          time
        }])
    }
    if (currentClipped === storedClipped?.[0]?.value) return
    await storeCurrentClipped()
    await loadClippedFromStore()
  }, 1000)
}

const writeTextToClipboard = async (value) => {
  await store.set('clicked', value)
  await writeText(value)
}

onMounted(async () => {
  Scrollbar.init(document.querySelector('.page'))
  await loadClippedFromStore()
  watchClipboard()
})
</script>
<template>
  <div class="page">
    <div class="container">
      <div v-for="c in clipped">
        <div class="clip-item" @dblclick="writeTextToClipboard(c.value)">
          <div class="clip-item-title">
            <span class="string-type">{{ c.type }}</span>
            <span class="clip-time">{{ new Date(c?.time).toLocaleString() }}</span>
          </div>
          <p class="clip-item-string">
            {{ c.value }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
