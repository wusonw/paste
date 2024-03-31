<script setup>
import Scrollbar from 'smooth-scrollbar';
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';
import { Store } from "tauri-plugin-store-api";
import { clear, readText, startListening, onClipboardUpdate, hasImage, readImageObjectURL, hasText } from 'tauri-plugin-clipboard-api'
import { listen } from '@tauri-apps/api/event';

const STORED_COUNT = 30
const clipStore = new Store(".clip-store.dat");
const clipRecords = ref([])
const isLoadingClipRecords = ref(false)

const loadClipRecords = async () => {
  console.log('reload')
  try {
    isLoadingClipRecords.value = true
    const storedClipped = await clipStore.get('clipRecords') || []
    clipRecords.value = storedClipped.toSorted((a, b) => b?.time > a?.time)
    isLoadingClipRecords.value = false
  } catch {
    clipRecords.value = []
    isLoadingClipRecords.value = false
  }
}

onClipboardUpdate(async () => {
  let record

  if (await hasText()) {
    record = {
      type: 'TEXT',
      value: await readText(),
      time: new Date()
    }
  }
  else if (await hasImage()) {
    record = {
      type: 'TEXT',
      value: await readImageObjectURL(),
      time: new Date()
    }
  }

  if (record) {
    await clipStore.set('clipRecords', [record, ...clipRecords.value])
    await loadClipRecords()
  }


})

onMounted(async () => {
  Scrollbar.init(document.querySelector('.page'))
  await loadClipRecords()
  await startListening()
  listen('custom://reload-clip-records', loadClipRecords)
})
</script>
<template>
  <div class="page">
    <div v-if="isLoadingClipRecords" class="center-text">
      Loading...
    </div>
    <div v-else-if="!isLoadingClipRecords && clipRecords.length == 0" class="center-text">
      No Clipped
    </div>
    <div v-else class="container">
      <div v-for="r in clipRecords">
        <div class="clip-item">
          <div class="clip-item-title">
            <span :class="[`${r.type.toLowerCase()}-type`]">{{ r.type }}</span>
            <span class="clip-time">{{ new Date(r?.time).toLocaleString() }}</span>
          </div>
          <p v-if="r.type === 'TEXT'" class="clip-item-text">
            {{ r.value }}
          </p>
          <p v-else-if="r.type === 'IMAGE'" class="clip-item-image">
            <img :src="r.value" width="100%" />
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
