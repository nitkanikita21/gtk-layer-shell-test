import { defineStore } from "pinia";
import { useQuery } from "@tanstack/vue-query";
import { invoke } from "@tauri-apps/api/core";
import { watch } from "vue";

export const useSystemStore = defineStore("system", () => {
  const systemQuery = useQuery<any>({
    queryKey: ["systemInfo"],
    queryFn: async () => await invoke<any>("get_system_info"),
    refetchInterval: 1000,
  })

  return { systemQuery }
});
