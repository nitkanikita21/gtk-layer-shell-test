<template>
  <div v-if="isBoth">↕ {{ filesize(bothBytes, { base: 2 }) }}</div>
  <div v-else-if="!isBoth"> {{ filesize(inBytes, { base: 2 }) }} |  {{ filesize(outBytes, { base: 2 }) }}</div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useSystemStore } from '../../stores/system';
import { filesize } from 'filesize';

const system = useSystemStore()
const bothBytes = computed<number>(() => system.systemQuery.data?.net_both ?? 0)
const inBytes = computed<number>(() => system.systemQuery.data?.net_in ?? 0)
const outBytes = computed<number>(() => system.systemQuery.data?.net_out ?? 0)

watch(() => bothBytes.value, console.warn)

const isBoth = ref(false)
</script>