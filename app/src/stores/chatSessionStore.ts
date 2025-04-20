import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import * as api from '../services/api';
import { ChatSession } from '../services/typings';

export const useChatSessionStore = defineStore('chatSession', () => {
  const sessions = ref<ChatSession[]>([]);
  const loading = ref(false);
  const initialized = ref(false);

  /**
   * 初始化加载所有会话数据
   */
  async function fetchAllSessions() {
    if (initialized.value) return;
    
    loading.value = true;
    try {
      sessions.value = await api.getAllSessions();

      // 重新排序
      sortSessions();
      
      initialized.value = true;
    } catch (error) {
      console.error('Failed to initialize chat sessions:', error);
    } finally {
      loading.value = false;
    }
  }

  /**
   * 根据ID获取会话
   */
  function getSessionById(id: number): ChatSession | undefined {
    return sessions.value.find(session => session.id === id);
  }

  /**
   * 根据智能体ID获取相关会话
   */
  function getSessionsByAgentId(agentId: number): ChatSession[] {
    return sessions.value.filter(session => session.agentId === agentId);
  }

  /**
   * 创建新会话
   */
  async function createNewSession(agentId: number, topic: string): Promise<ChatSession> {
    loading.value = true;
    try {
      const newSession = await api.addSession({
        agentId,
        topic
      });
      
      // 将新会话添加到本地状态
      sessions.value.push(newSession);
      
      // 按更新时间排序
      sortSessions();
      
      return newSession;
    } catch (error) {
      console.error('Failed to create chat session:', error);
      throw error;
    } finally {
      loading.value = false;
    }
  }

  /**
   * 更新会话
   */
  async function updateSession(sessionId: number, updateData: Partial<ChatSession>): Promise<boolean> {
    loading.value = true;
    try {
      const sessionIndex = sessions.value.findIndex(s => s.id === sessionId);
      if (sessionIndex === -1) {
        throw new Error(`Session with id ${sessionId} not found`);
      }
      
      const existingSession = sessions.value[sessionIndex];
      const updatedSession: ChatSession = {
        ...existingSession,
        ...updateData,
        updatedAt: Date.now()
      };
      
      const success = await api.updateSession(updatedSession);
      
      if (success) {
        // 更新本地状态
        sessions.value[sessionIndex] = updatedSession;
        
        // 重新排序
        sortSessions();
      }
      
      return success;
    } catch (error) {
      console.error(`Failed to update chat session ${sessionId}:`, error);
      return false;
    } finally {
      loading.value = false;
    }
  }

  /**
   * 删除会话
   */
  async function deleteSession(sessionId: number): Promise<boolean> {
    loading.value = true;
    try {
      const success = await api.deleteSession(sessionId);
      
      if (success) {
        // 从本地状态中删除
        sessions.value = sessions.value.filter(session => session.id !== sessionId);
      }
      
      return success;
    } catch (error) {
      console.error(`Failed to delete chat session ${sessionId}:`, error);
      return false;
    } finally {
      loading.value = false;
    }
  }

  /**
   * 根据最近更新时间对会话进行排序
   */
  function sortSessions() {
    sessions.value.sort((a, b) => {
      const timeA = a.updatedAt || a.createdAt;
      const timeB = b.updatedAt || b.createdAt;
      return timeB - timeA; // 降序排列，最新的在前面
    });
  }

  /**
   * 计算最近的会话
   */
  const recentSessions = computed(() => {
    return [...sessions.value].sort((a, b) => {
      const timeA = a.updatedAt || a.createdAt;
      const timeB = b.updatedAt || b.createdAt;
      return timeB - timeA;
    }).slice(0, 5); // 获取最近的5个会话
  });

  /**
   * 更新会话主题
   * @param sessionId 会话ID
   * @param newTopic 新的主题
   * @returns 更新是否成功
   */
  async function updateSessionTopic(sessionId: number, newTopic: string): Promise<boolean> {
    if (!newTopic.trim()) {
      console.error('会话主题不能为空');
      return false;
    }
    
    try {
      // 使用已有的updateSession函数，只更新topic字段
      const success = await updateSession(sessionId, { topic: newTopic.trim() });

      if (success) {
        // 更新本地状态
        const sessionIndex = sessions.value.findIndex(s => s.id === sessionId);
        if (sessionIndex !== -1) {
          sessions.value[sessionIndex].topic = newTopic.trim();
        }

        // 重新排序
        sortSessions();
      }
      return success;
    } catch (error) {
      console.error(`更新会话主题失败 (ID: ${sessionId}):`, error);
      return false;
    }
  }

  return {
    sessions,
    loading,
    initialized,
    recentSessions,
    fetchAllSessions,
    getSessionById,
    getSessionsByAgentId,
    createNewSession,
    updateSession,
    deleteSession,
    updateSessionTopic  // 导出新函数
  };
});
