<template>
  <div class="bg-transparent w-screen h-screen flex flex-row px-4 p-1 pb-0 justify-between">
    <!-- Ваш вміст -->
    <div class="flex flex-row gap-2">
      <MediaPlayer />
    </div>
    <div class="flex flex-row gap-2">
      <Workspaces />
    </div>
    <div class="flex flex-row gap-2">
      <Chip>
        <Network class="text-green" />
      </Chip>
      <Chip>
        <Cpu class="text-green" />
        <Ram class="text-peach" />
        <Disk class="text-blue" />
      </Chip>
      <Chip>
        <Date class="text-rosewater" />
      </Chip>
    </div>




  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import Chip from './components/Chip.vue';
import Cpu from './components/indicators/Cpu.vue';
import Disk from './components/indicators/Disk.vue';
import Network from './components/indicators/Network.vue';
import Ram from './components/indicators/Ram.vue';
import Workspaces from './components/widgets/Workspaces.vue';
import MediaPlayer from './components/widgets/MediaPlayer.vue';
import Date from './components/indicators/Date.vue';
</script>

<style>
html,
body {
  margin: 0;
  background: transparent;
}
</style>