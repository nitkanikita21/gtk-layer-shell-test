<template>
  <motion.div layout :transition="{ duration: 0.3, ease: 'easeInOut' }"
    class="select-none bg-surface0 px-4 rounded-2xl gap-4 flex flex-row items-center text-xs text-mauve">
    <slot />
  </motion.div>
</template>

<script setup lang="ts">
import { motion } from 'motion-v';
</script>