<template>
  <Chip>
    <div v-for="ws in workspaces" :key="ws.id" @click="() => switchToWorkspace(ws.id)"
      class="flex flex-row items-center justify-center text-xs text-center text-white text-shadow-xs bg-surface3 h-4 rounded-full transition-all duration-500 ease-in-out hover:bg-surface2 hover:cursor-pointer"
      :class="{
        'w-12 gradient-cycle-wall': ws.is_active,
        'w-4': !ws.is_active,
      }">
      <div v-if="ws.is_active" class="self-center">{{ ws.id }}</div>
    </div>
  </Chip>
</template>

<script setup lang="ts">
import { useQuery } from '@tanstack/vue-query'
import Chip from '../Chip.vue'
import { invoke } from '@tauri-apps/api/core'
import { WorkspaceInfo } from '../../types/WorkspaceInfo'

const { data: workspaces } = useQuery({
  queryFn: async () => await invoke<WorkspaceInfo[]>('get_workspaces'),
  queryKey: ['get_workspaces'],
  refetchInterval: 50,
})

async function switchToWorkspace(id: number) {
  await invoke('switch_to_workspace', { id })
}
</script>
