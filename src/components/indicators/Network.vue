<template>
  <AnimatePresence mode="wait">
    <motion.span
      v-if="!isBoth"
      key="both"
      @click="isBoth = !isBoth"
      class="text-green"
      :exit="{ opacity: 0 }"
    >
      ↕ {{ filesize(bothBytes) }}
    </motion.span>
    <motion.span
      v-else
      key="separate"
      @click="isBoth = !isBoth"
      class="text-green"
      :exit="{ opacity: 0 }"
    >
       {{ filesize(inBytes) }} |  {{ filesize(outBytes) }}
    </motion.span>
  </AnimatePresence>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { AnimatePresence, motion } from "motion-v";
import { useSystemStore } from "../../stores/system";
import { filesize } from "filesize";

const system = useSystemStore();
const bothBytes = computed(() => system.systemQuery.data?.net_both ?? 0);
const inBytes = computed(() => system.systemQuery.data?.net_in ?? 0);
const outBytes = computed(() => system.systemQuery.data?.net_out ?? 0);
const isBoth = ref(false);
</script>
