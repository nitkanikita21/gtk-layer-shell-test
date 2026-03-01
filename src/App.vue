<template>
  <div class="bg-transparent w-screen h-screen flex flex-row gap-2 px-4 p-1 pb-0">
    <!-- Ваш вміст -->
    <Chip>
      <Cpu class="text-green" />
      <Ram class="text-peach" />
      <Disk class="text-blue" />
    </Chip>
    <MediaPlayer />
    <Chip>
      <Network class="text-green" />
    </Chip>
  </div>
</template>

<script setup lang="ts">
import Chip from './components/Chip.vue';
import Cpu from './components/indicators/Cpu.vue';
import Disk from './components/indicators/Disk.vue';
import Network from './components/indicators/Network.vue';
import Ram from './components/indicators/Ram.vue';
import MediaPlayer from './components/MediaPlayer.vue';


</script>

<style>
html,
body {
  margin: 0;
  background: transparent;
}
</style>