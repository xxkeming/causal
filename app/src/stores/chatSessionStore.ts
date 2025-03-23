import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { ChatSession, ChatMessage } from '../services/typings';

export const useChatSessionStore = defineStore('chatSession', () => {
  // 状态
  const sessions = ref<ChatSession[]>([]);
  const messages = ref<ChatMessage[]>([]);
  const currentSessionId = ref<number | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  // 计算属性
  const currentSession = computed(() => {
    if (currentSessionId.value === null) return null;
    return sessions.value.find(s => s.id === currentSessionId.value) || null;
  });
  
  const currentMessages = computed(() => {
    if (currentSessionId.value === null) return [];
    return messages.value.filter(m => m.sessionId === currentSessionId.value);
  });

  const getSessionsByAgentId = computed(() => (agentId: string) => {
    return sessions.value.filter(s => s.agentId === agentId);
  });
  
  const getMessagesBySessionId = computed(() => (sessionId: number) => {
    return messages.value.filter(m => m.sessionId === sessionId);
  });
  
  // 生成新的会话ID (简单实现，实际应从后端获取)
  function generateSessionId(): number {
    const existingIds = sessions.value.map(s => s.id);
    if (existingIds.length === 0) return 1;
    return Math.max(...existingIds) + 1;
  }
  
  // 加载会话和消息数据
  function loadSessionsAndMessages() {
    try {
      // 加载会话
      const storedSessions = localStorage.getItem('chat-sessions');
      if (storedSessions) {
        sessions.value = JSON.parse(storedSessions);
      }
      
      // 加载消息
      const storedMessages = localStorage.getItem('chat-messages');
      if (storedMessages) {
        const parsedMessages = JSON.parse(storedMessages);
        messages.value = parsedMessages.map((msg: any) => ({
          ...msg,
          timestamp: msg.timestamp ? new Date(msg.timestamp) : undefined
        }));
      }
      
      // 恢复当前会话
    //   const storedCurrentSessionId = localStorage.getItem('current-session-id');
    //   if (storedCurrentSessionId) {
    //     currentSessionId.value = Number(storedCurrentSessionId);
    //   }
    } catch (err) {
      console.error('加载会话记录失败:', err);
      error.value = '加载会话记录失败';
    }
  }
  
  // 保存会话数据到本地存储
  function saveSessions() {
    try {
      localStorage.setItem('chat-sessions', JSON.stringify(sessions.value));
      localStorage.setItem('current-session-id', currentSessionId.value?.toString() || '');
    } catch (err) {
      console.error('保存会话记录失败:', err);
      error.value = '保存会话记录失败';
    }
  }
  
  // 保存消息数据到本地存储
  function saveMessages() {
    try {
      const messagesToSave = messages.value.map(msg => ({
        ...msg,
        timestamp: msg.timestamp ? msg.timestamp.toISOString() : undefined
      }));
      localStorage.setItem('chat-messages', JSON.stringify(messagesToSave));
    } catch (err) {
      console.error('保存消息记录失败:', err);
      error.value = '保存消息记录失败';
    }
  }
  
  // 切换到指定智能体
  async function switchToAgent(agentId: string) {
    loading.value = true;
    
    try {
      // 筛选与当前智能体相关的会话
      const agentSessions = sessions.value.filter(s => s.agentId === agentId);
      
      // 如果有相关会话，选择最近的一个
      if (agentSessions.length > 0) {
        // 按更新时间排序，选择最新的会话
        const sortedSessions = [...agentSessions].sort((a, b) => {
          const timeA = new Date(a.updatedAt || a.createdAt).getTime();
          const timeB = new Date(b.updatedAt || b.createdAt).getTime();
          return timeB - timeA; // 降序排列
        });
        
        currentSessionId.value = sortedSessions[0].id;
      }
      
      saveSessions();
      return true;
    } catch (err) {
      console.error('切换智能体失败:', err);
      error.value = '切换智能体失败';
      return false;
    } finally {
      loading.value = false;
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
    currentSessionId.value = newSession.id;
    saveSessions();
    
    return newSession;
  }
  
  // 切换会话
  function switchSession(sessionId: number) {
    const session = sessions.value.find(s => s.id === sessionId);
    if (session) {
      currentSessionId.value = sessionId;
      saveSessions();
      return session;
    }
    return null;
  }
  
  // 删除会话及其消息
  function deleteSession(sessionId: number) {
    const sessionIndex = sessions.value.findIndex(s => s.id === sessionId);
    if (sessionIndex >= 0) {
      // 删除会话
      sessions.value.splice(sessionIndex, 1);
      
      // 删除关联的所有消息
      messages.value = messages.value.filter(m => m.sessionId !== sessionId);
      
      // 如果删除的是当前会话，需要处理当前会话ID
      if (currentSessionId.value === sessionId) {
        if (sessions.value.length > 0) {
          currentSessionId.value = sessions.value[0].id;
        } else {
          currentSessionId.value = null;
        }
      }
      
      // 保存更新
      saveSessions();
      saveMessages();
      return true;
    }
    return false;
  }
  
  // 添加消息
  function addMessage(message: Omit<ChatMessage, 'sessionId'>) {
    if (currentSessionId.value === null) return false;
    
    const newMessage: ChatMessage = {
      ...message,
      sessionId: currentSessionId.value
    };
    
    messages.value.push(newMessage);
    
    // 更新会话的最后更新时间
    const sessionIndex = sessions.value.findIndex(s => s.id === currentSessionId.value);
    if (sessionIndex >= 0) {
      sessions.value[sessionIndex].updatedAt = new Date().toISOString();
    }
    
    saveMessages();
    saveSessions();
    return true;
  }
  
  // 删除单条消息
  function removeMessage(messageIndex: number) {
    if (currentSessionId.value === null) return false;
    
    // 找出当前会话的消息
    const sessionMessages = messages.value.filter(m => m.sessionId === currentSessionId.value);
    
    if (messageIndex < 0 || messageIndex >= sessionMessages.length) {
      return false;
    }
    
    // 获取要删除的消息
    const targetMessage = sessionMessages[messageIndex];
    
    // 从整体消息列表中找到和删除
    const globalIndex = messages.value.findIndex(m => 
      m.sessionId === targetMessage.sessionId && 
      m.content === targetMessage.content && 
      m.role === targetMessage.role && 
      m.timestamp === targetMessage.timestamp
    );
    
    if (globalIndex >= 0) {
      messages.value.splice(globalIndex, 1);
      saveMessages();
      return true;
    }
    
    return false;
  }
  
  // 清空当前会话的所有消息
  function clearCurrentSessionMessages() {
    if (currentSessionId.value === null) return false;
    
    // 删除当前会话的所有消息
    messages.value = messages.value.filter(m => m.sessionId !== currentSessionId.value);
    
    // 更新会话的最后更新时间
    const sessionIndex = sessions.value.findIndex(s => s.id === currentSessionId.value);
    if (sessionIndex >= 0) {
      sessions.value[sessionIndex].updatedAt = new Date().toISOString();
    }
    
    saveMessages();
    saveSessions();
    return true;
  }
  
  // 更新会话主题
  function updateSessionTopic(sessionId: number, topic: string) {
    const sessionIndex = sessions.value.findIndex(s => s.id === sessionId);
    if (sessionIndex >= 0) {
      sessions.value[sessionIndex].topic = topic;
      sessions.value[sessionIndex].updatedAt = new Date().toISOString();
      saveSessions();
      return true;
    }
    return false;
  }
  
  // 清空当前会话id
  function clearCurrentSessionId() {
      currentSessionId.value = null;
      saveSessions();
  }
    
  return {
    // 状态
    sessions,
    messages,
    currentSessionId,
    loading,
    error,
    
    // 计算属性
    clearCurrentSessionId,
    currentSession,
    currentMessages,
    getSessionsByAgentId,
    getMessagesBySessionId,
    
    // 方法
    loadSessionsAndMessages,
    saveSessions,
    saveMessages,
    switchToAgent,
    createNewSession,
    switchSession,
    deleteSession,
    addMessage,
    removeMessage,
    clearCurrentSessionMessages,
    updateSessionTopic
  };
});
