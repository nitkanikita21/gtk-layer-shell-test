<template>
    <AnimatePresence mode="wait">
        <motion.span v-if="!isFull" :key="'simple'" @click="isFull = !isFull" class="text-green cursor-pointer"
            :initial="{ opacity: 0 }" :animate="{ opacity: 1 }" :exit="{ opacity: 0 }" :transition="{ duration: 0.2 }">
             {{ shortDate }}
        </motion.span>
        <motion.span v-else :key="'full'" @click="isFull = !isFull" class="text-green cursor-pointer whitespace-nowrap"
            :initial="{ opacity: 0 }" :animate="{ opacity: 1 }" :exit="{ opacity: 0 }" :transition="{ duration: 0.2 }">
             {{ fullDate }}
        </motion.span>
    </AnimatePresence>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { AnimatePresence, motion } from "motion-v";
import { useIntervalFn } from "@vueuse/core";

const isFull = ref(false);
const now = ref(new Date());

// Оновлення часу кожну секунду
useIntervalFn(() => {
    now.value = new Date();
}, 1000);

// Отримання номера тижня (алгоритм ISO 8601)
function getWeekNumber(date: Date): number {
    const d = new Date(Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()));
    const dayNum = d.getUTCDay() || 7;
    d.setUTCDate(d.getUTCDate() + 4 - dayNum);
    const yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1));
    return Math.ceil(((d.getTime() - yearStart.getTime()) / 86400000 + 1) / 7);
}

// Форматери для різних частин
const weekdayFormatter = new Intl.DateTimeFormat('en-US', { weekday: 'long' });
const monthFormatter = new Intl.DateTimeFormat('en-US', { month: 'long' });
const shortWeekdayFormatter = new Intl.DateTimeFormat('en-US', { weekday: 'short' });
const shortMonthFormatter = new Intl.DateTimeFormat('en-US', { month: 'short' });
const time12Formatter = new Intl.DateTimeFormat('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: true
});
const time24Formatter = new Intl.DateTimeFormat('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
});

// Повний формат: "Monday March 02 2026 (10) | 08:31:45 PM"
const fullDate = computed(() => {
    const d = now.value;
    const weekday = weekdayFormatter.format(d);
    const month = monthFormatter.format(d);
    const day = d.getDate().toString().padStart(2, '0');
    const year = d.getFullYear();
    const week = getWeekNumber(d);
    const time = time12Formatter.format(d);

    return `${weekday} ${month} ${day} ${year} (${week}) | ${time}`;
});

// Короткий формат: "02 Mon 20:32"
const shortDate = computed(() => {
    const d = now.value;
    const day = d.getDate().toString().padStart(2, '0');
    const month = shortMonthFormatter.format(d);
    const time = time24Formatter.format(d);

    return `${day} ${month} ${time}`;
});
</script>