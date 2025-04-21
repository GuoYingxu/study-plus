import { Store } from "@tauri-apps/plugin-store";
import { appDataDir } from "@tauri-apps/api/path";
export interface UserConfig {
  //python隔离环境文件夹
  pythonEnvDir?: string;
  //jupyter 实验文件路径
  jupyterLabProjectPath?: string;
}
// console.log(USER_CONFIG_PATH);
export async function getUserConfig(): Promise<Store> {
  const USER_CONFIG_PATH = (await appDataDir()) + "userConfig.dat";
  const store = await Store.load(USER_CONFIG_PATH);
  return store;
}

export async function setUserConfig(key: keyof UserConfig, value: string) {
  const config = await getUserConfig();
  config.set(key, value);
  config.save();
}

export async function setPythonEnvDir(value: string) {
  await setUserConfig("pythonEnvDir", value);
}

export async function setJupyterLabProjectPath(value: string) {
  await setUserConfig("jupyterLabProjectPath", value);
}

export async function getUserConfigValue(key: keyof UserConfig) {
  const config = await getUserConfig();
  return config.get(key);
}
export async function getPythonEnvDir(): Promise<string | undefined> {
  return (await getUserConfigValue("pythonEnvDir")) as string;
}

export async function getJupyterLabProjectPath() {
  return await getUserConfigValue("jupyterLabProjectPath");
}
