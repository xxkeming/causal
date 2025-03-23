import { invoke } from "@tauri-apps/api/core";

/**
 * 调用Tauri后端的greet函数
 * @param name 要问候的名称
 * @returns 返回问候语的Promise
 */
export async function greet(name: string): Promise<string> {
  try {
    // 调用Rust端定义的greet命令
    return await invoke('greet', { name });
  } catch (error) {
    console.error('Failed to call Tauri greet function:', error);
    throw error;
  }
}

/**
 * 检查Tauri是否可用
 * @returns 如果Tauri可用则返回true，否则返回false
 */
export function isTauriAvailable(): boolean {
  return typeof window !== 'undefined' && window.__TAURI__ !== undefined;
}

/**
 * 安全地调用Tauri函数，如果Tauri不可用则返回fallback
 * @param tauriFunction 要调用的Tauri函数
 * @param fallback 如果Tauri不可用时返回的值
 * @param args 传递给Tauri函数的参数
 * @returns 返回Tauri函数的结果或fallback
 */
export async function safeTauriCall<T>(
  tauriFunction: (...args: any[]) => Promise<T>, 
  fallback: T, 
  ...args: any[]
): Promise<T> {
  if (isTauriAvailable()) {
    try {
      return await tauriFunction(...args);
    } catch (error) {
      console.error('Tauri function call failed:', error);
      return fallback;
    }
  }
  return fallback;
}
