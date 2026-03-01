<template>
    <div v-if="diskUsage"> {{ filesize(diskUsage.toFixed(0), { base: 2 }) }}</div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue';
import { useSystemStore } from '../../stores/system';
import { filesize } from 'filesize';

const system = useSystemStore()
const diskUsage = computed<number>(() => system.systemQuery.data?.disk_used)

watch(diskUsage, console.log)
</script>