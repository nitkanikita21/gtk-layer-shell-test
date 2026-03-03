<script setup lang="ts">
import { computed, ref, watch, watchEffect } from 'vue';
import { useQuery } from '@tanstack/vue-query';
import { invoke } from '@tauri-apps/api/core';
import Chip from '../Chip.vue';

const mediaInfo = useQuery({
  queryFn: async () => await invoke<any>("get_media_info"),
  queryKey: ["get_media_info"],
  refetchInterval: 200,
})

async function next() {
  await invoke<string>("media_next")
}

async function previous() {
  await invoke<string>("media_previous")
}

async function toggle() {
  await invoke<string>("media_toggle_play_pause")
}


const icon = computed(() => {
  if (!mediaInfo.data.value) return ""
  switch (mediaInfo.data.value.status as 'Playing' | 'Paused' | 'Stopped') {
    case 'Paused':
      return ""
    case 'Stopped':
      return "ﭥ "
    default:
      return ""
  }
})
</script>

<template>
  <Chip v-if="mediaInfo.data.value" @click="toggle"
    :class="{ 'gradient-cycle-wall': mediaInfo.data.value.status == 'Playing' }" class="max-w-xs ">
    <div :class="{ 'text-white text-shadow-xs': mediaInfo.data.value.status == 'Playing' }" class="text-mauve truncate">
      {{ icon }} {{ mediaInfo.data.value.artist }} - {{ mediaInfo.data.value.title }}
    </div>
  </Chip>
</template>