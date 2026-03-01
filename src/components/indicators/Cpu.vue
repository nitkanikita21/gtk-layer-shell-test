<template>
    <div v-if="cpuUsage"> {{ cpuUsage.toFixed(0) }}%</div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue';
import { useSystemStore } from '../../stores/system';

const system = useSystemStore()
const cpuUsage = computed<number>(() => system.systemQuery.data?.cpu_used)

watch(cpuUsage, console.log)
</script>