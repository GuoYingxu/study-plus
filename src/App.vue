<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { download } from '@tauri-apps/plugin-upload';
import { BaseDirectory, exists, mkdir, remove } from '@tauri-apps/plugin-fs';
import { appLocalDataDir, desktopDir, join } from '@tauri-apps/api/path';
import { getPythonEnvDir, setPythonEnvDir } from './config/userConfig';
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from '@tauri-apps/api/core';
import { Command } from '@tauri-apps/plugin-shell';

const pythonEnvPath = ref();
const isDownloading = ref(false);
const downloadProgress = ref(0);
const JupyterEnvZipUrl = ref('https://uplus-sql.oss-cn-hangzhou.aliyuncs.com/jupyter_server.zip');
const expermentId = "firstJupyterLabProject";
const canExecute = ref(false);
onMounted(async () => {
  const envPath = await getPythonEnvDir();
  if (envPath) {
    pythonEnvPath.value = envPath;
  } else {
    await setPythonEnvDir(await appLocalDataDir());
    pythonEnvPath.value = await appLocalDataDir();
  }
});

async function open_jupyterlab() {
  if (!pythonEnvPath.value) return;
  const folderPath = await join(pythonEnvPath.value, expermentId);
  console.log('准备打开的文件夹路径...', folderPath);
  // 这里可以使用 Tauri 的命令行工具来打开 JupyterLab
  const exePath = await join(folderPath, 'jupyter_server', 'jupyter_server.exe');
  const jupyterfile = "D:\\introduction_to_ml_with_python-main"
  Command.create(exePath, [jupyterfile])
    .spawn()
    .then(() => {
      console.log('JupyterLab 已启动');
    })
    .catch((error) => {
      console.error('启动 JupyterLab 失败:', error);
      alert('启动 JupyterLab 失败，请检查安装目录和环境变量设置。');
    });
  // await invoke('open_jupyterlab', { path: folderPath });
}

async function selectDirectory() {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await desktopDir(),
  });
  if (selected) {
    pythonEnvPath.value = selected as string;

    console.log('选择的安装目录:', pythonEnvPath.value);
  }
}

async function downloadAndExtract() {
  console.log('开始下载和解压...', pythonEnvPath.value);
  if (!pythonEnvPath.value) return;
  const folderPath = await join(pythonEnvPath.value, expermentId);
  console.log('准备创建的文件夹路径...', folderPath);

  try {
    isDownloading.value = true;
    downloadProgress.value = 0;
    if (await exists(folderPath)) {
      console.log('文件夹已存在，准备删除旧文件夹...');
      await remove(folderPath, {
        recursive: true
      });
    }
    console.log('准备创建新文件夹...');
    // 创建目录
    await mkdir(folderPath, { recursive: true })
    console.log('创建文件夹成功，准备下载文件...');
    // 下载文件
    const filePath = await join(folderPath, 'jupyterlab.zip');
    await download(JupyterEnvZipUrl.value, filePath, (progress) => {
      downloadProgress.value = Math.round(progress.progressTotal / progress.total * 100);
      // console.log('下载进度:', progress.progress, progress.progressTotal, progress.total, progress.transferSpeed);
    });

    // 解压文件
    const extractPath = folderPath;// await join(folderPath, 'jupyterlab');
    await invoke('extract_zip', { filePath, extractTo: extractPath });

    // 清理临时文件
    await remove(filePath, { recursive: true });
    alert('JupyterLab环境已成功安装！');
    canExecute.value = true;
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
        <div class="selected-path" v-if="pythonEnvPath">
          已选择: {{ pythonEnvPath }}
        </div>
      </div>

      <div class="download-section" v-if="pythonEnvPath">
        <button @click="downloadAndExtract" :disabled="isDownloading" class="download-button">
          {{ isDownloading ? '正在安装...' : '下载并安装JupyterLab' }}
        </button>

        <div class="progress-bar" v-if="isDownloading">
          <div class="progress" :style="{ width: `${downloadProgress}%` }"></div>
          <span class="progress-text">{{ downloadProgress }}%</span>
        </div>
      </div>
      <div class="execute-section">
        <button @click="open_jupyterlab" class="download-button">
          打开JupyterLab
        </button>
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
