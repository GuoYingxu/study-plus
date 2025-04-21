<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/api/dialog';
import { downloadFile } from '@tauri-apps/api/http';
import { extract } from '@tauri-apps/api/archive';
import { createDir } from '@tauri-apps/api/fs';

const selectedPath = ref('');
const isDownloading = ref(false);
const downloadProgress = ref(0);

async function selectDirectory() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  if (selected) {
    selectedPath.value = selected as string;
  }
}

async function downloadAndExtract() {
  if (!selectedPath.value) return;

  try {
    isDownloading.value = true;
    downloadProgress.value = 0;

    // 创建目录
    await createDir(selectedPath.value, { recursive: true });

    // 下载文件
    const filePath = `${selectedPath.value}/jupyterlab.zip`;
    await downloadFile(
      'https://your-jupyterlab-download-url.com/jupyterlab.zip',
      filePath,
      (progress) => {
        downloadProgress.value = Math.round(progress.progress * 100);
      }
    );

    // 解压文件
    await extract(filePath, selectedPath.value);

    alert('JupyterLab环境已成功安装！');
  } catch (error) {
    console.error('下载或解压过程出错：', error);
    alert('安装过程出错，请重试');
  } finally {
    isDownloading.value = false;
    downloadProgress.value = 0;
  }
}
</script>

<template>
  <main class="container">
    <h1>JupyterLab环境配置</h1>

    <div class="setup-container">
      <div class="directory-selector">
        <button @click="selectDirectory" :disabled="isDownloading">
          选择安装目录
        </button>
        <div class="selected-path" v-if="selectedPath">
          已选择: {{ selectedPath }}
        </div>
      </div>

      <div class="download-section" v-if="selectedPath">
        <button @click="downloadAndExtract" :disabled="isDownloading" class="download-button">
          {{ isDownloading ? '正在安装...' : '下载并安装JupyterLab' }}
        </button>

        <div class="progress-bar" v-if="isDownloading">
          <div class="progress" :style="{ width: `${downloadProgress}%` }"></div>
          <span class="progress-text">{{ downloadProgress }}%</span>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.setup-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-top: 20px;
  align-items: center;
}

.directory-selector {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.selected-path {
  font-size: 0.9em;
  color: #666;
  word-break: break-all;
  max-width: 400px;
}

.download-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
}

.download-button {
  background-color: #4CAF50;
  color: white;
}

.download-button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.progress-bar {
  width: 300px;
  height: 20px;
  background-color: #f0f0f0;
  border-radius: 10px;
  overflow: hidden;
  position: relative;
}

.progress {
  height: 100%;
  background-color: #4CAF50;
  transition: width 0.3s ease;
}

.progress-text {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  color: #000;
  font-size: 12px;
}

.version {
  margin-top: 20px;
  font-size: 0.9em;
  color: #646cff;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>