<template>
    <div v-if="ramUsage"> {{ filesize(ramUsage.toFixed(0), { base: 2 }) }}</div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue';
import { useSystemStore } from '../../stores/system';
import { filesize } from 'filesize';

const system = useSystemStore()
const ramUsage = computed<number>(() => system.systemQuery.data?.ram_used)

watch(ramUsage, console.log)
</script>