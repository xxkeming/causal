import { invoke, Channel } from "@tauri-apps/api/core";

interface Response {
  status: string;
  data?: Object;
  error?: string;
}

/**
 * 调用Tauri后端的fetch_local函数
 * @param name 要问候的名称
 * @param data 附加数据
 * @returns 返回问候语的Promise
 */
export async function fetch_local(name: string, data: Object | null): Promise<Object> {
  try {
    console.log('fetch_local:', name, data);

    // data 转json字符串
    let result = await invoke('fetch', { name, data: data === null ? "" : JSON.stringify(data)}) as Response;
    console.log('fetch_local result:', result);

    if (result.status === "error") {
      throw 'error:' + result.error;
    }
    return result.data === undefined ? true : result.data as Object;
  } catch (error) {
    console.error('Failed to call fetch_local function:', error);
    throw error;
  }
}

export type  MessageEvent =
| {
    event: 'started';
  }
| {
    event: 'progress';
    data: {
      content: string;
    };
  }
| {
    event: 'finished';
};
    
/**
 * 调用Tauri后端的event_local函数
 * @param agnetId 智能体ID
 * @param content 发送的消息
 * @returns 返回问候语的Promise
 */
export async function event_local(agentId: number, sessionId: number, messageId: number, stream: boolean, onData: (event: MessageEvent) => void): Promise<Object> {
  try {
    console.log('event_local:', agentId, sessionId, messageId);

    const onEvent = new Channel<MessageEvent>();
    onEvent.onmessage = (message) => {
      console.log(`got event ${message.event}`);
      onData(message);
    };
    
    // data 转json字符串
    let result = await invoke('event', { agent: agentId, session: sessionId, message: messageId, stream, onEvent }) as Response;
    console.log('event_local result:', result);

    if (result.status === "error") {
      throw 'error:' + result.error;
    }
    return result.data === undefined ? true : result.data as Object;
  } catch (error) {
    console.error('Failed to call event_local function:', error);
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
