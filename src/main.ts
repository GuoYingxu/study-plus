import { createApp } from "vue";
import App from "./App.vue";

import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
async function checkVersion() {
  const update = await check();
  if (update) {
    await update.downloadAndInstall();
    await relaunch();
  } else {
    createApp(App).mount("#app");
  }
}
checkVersion();
