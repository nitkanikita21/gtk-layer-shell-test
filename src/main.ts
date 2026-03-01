import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import { QueryClient, VueQueryPlugin } from "@tanstack/vue-query";

const app = createApp(App)
const queryClient = new QueryClient();
const pinia = createPinia();
app.use(pinia);
app.use(VueQueryPlugin, { queryClient });

app.mount("#app");
