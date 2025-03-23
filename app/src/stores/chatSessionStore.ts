import { defineStore } from 'pinia';
import { ref } from 'vue';
import { ChatSession } from '../services/typings';

export const useChatSessionStore = defineStore('chatSession', () => {
  // 状态 - 只保留会话列表
  const sessions = ref<ChatSession[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const needRefresh = ref(false);
  
  // 计算属性
  const getSessionsByAgentId = (agentId: string) => {
    return sessions.value.filter(s => s.agentId === agentId);
  };
  
  // 生成新的会话ID (简单实现，实际应从后端获取)
  function generateSessionId(): number {
    const existingIds = sessions.value.map(s => s.id);
    if (existingIds.length === 0) return 1;
    return Math.max(...existingIds) + 1;
  }
  
  // 加载会话列表
  function loadSessions() {
    try {
      // 加载会话
      const storedSessions = localStorage.getItem('chat-sessions');
      if (storedSessions) {
        sessions.value = JSON.parse(storedSessions);
      }
    } catch (err) {
      console.error('加载会话记录失败:', err);
      error.value = '加载会话记录失败';
    }
  }
  
  // 保存会话数据到本地存储
  function saveSessions() {
    try {
      localStorage.setItem('chat-sessions', JSON.stringify(sessions.value));
    } catch (err) {
      console.error('保存会话记录失败:', err);
      error.value = '保存会话记录失败';
    }
  }
  
  // 创建新的会话
  async function createNewSession(agentId: string, topic: string): Promise<ChatSession> {
    const now = new Date().toISOString();
    const newSession: ChatSession = {
      id: generateSessionId(),
      agentId,
      topic: topic || "新的对话", // 确保总是有一个默认值
      createdAt: now,
      updatedAt: now
    };
    
    sessions.value.push(newSession);
    saveSessions();
    needRefresh.value = false;
    
    return newSession;
  }
  
  // 获取会话信息 - 添加新方法获取单个会话
  function getSessionById(sessionId: number): ChatSession | null {
    return sessions.value.find(s => s.id === sessionId) || null;
  }
  
  // 删除会话
  function deleteSession(sessionId: number) {
    const sessionIndex = sessions.value.findIndex(s => s.id === sessionId);
    if (sessionIndex >= 0) {
      // 删除会话
      sessions.value.splice(sessionIndex, 1);
      
      // 保存更新
      saveSessions();
      return true;
    }
    return false;
  }
  
  // 更新会话 - 可用于更新最后活动时间等
  function updateSession(sessionId: number, updateData: Partial<ChatSession>) {
    const sessionIndex = sessions.value.findIndex(s => s.id === sessionId);
    if (sessionIndex >= 0) {
      // 更新会话数据，保留原有字段
      sessions.value[sessionIndex] = {
        ...sessions.value[sessionIndex],
        ...updateData,
        updatedAt: new Date().toISOString() // 自动更新时间戳
      };
      
      saveSessions();
      return true;
    }
    return false;
  }

  // 更新会话主题
  function updateSessionTopic(sessionId: number, topic: string) {
    return updateSession(sessionId, { topic });
  }
  
  // 设置需要刷新标志
  function setNeedRefresh() {
    needRefresh.value = true;
  }
  
  // 初始化 store - 添加初始化方法
  function initialize() {
    loadSessions();
    needRefresh.value = false;
  }
  
  return {
    // 状态
    sessions,
    loading,
    error,
    needRefresh,
    
    // 方法
    initialize,
    getSessionById,
    getSessionsByAgentId,
    loadSessions,
    saveSessions,
    createNewSession,
    deleteSession,
    updateSession,
    updateSessionTopic,
    setNeedRefresh
  };
});
