import { defineStore } from 'pinia';
import { markRaw } from 'vue';
import { 
  DocumentTextOutline, // 默认文档
  ImageOutline, // 图片
  MusicalNoteOutline, // 音频
  FilmOutline, // 视频
  CodeOutline, // 代码
  DocumentOutline, // 普通文档
  ArchiveOutline, // 压缩文件
  LogoJavascript, // JavaScript
  LogoPython, // Python
  LogoHtml5, // HTML
  LogoCss3, // CSS
  LogoGithub, // Git相关
  DocumentLockOutline, // 加密文档
  DocumentAttachOutline, // 附件通用
  PrismOutline, // 设计文件
  SpeedometerOutline, // 可执行文件
  ListOutline, // 列表/数据文件
  ServerOutline // 数据库文件
} from '@vicons/ionicons5';

// 定义文件图标选项接口
export interface FileIconOption {
  icon: any;
  color: string;
}

// 文件扩展名到图标的映射
const extensionIconMap: Record<string, { icon: any; color: string }> = {
  // 图片文件
  'jpg': { icon: markRaw(ImageOutline), color: '#f06292' },
  'jpeg': { icon: markRaw(ImageOutline), color: '#f06292' },
  'png': { icon: markRaw(ImageOutline), color: '#f06292' },
  'gif': { icon: markRaw(ImageOutline), color: '#f06292' },
  'svg': { icon: markRaw(ImageOutline), color: '#f06292' },
  'webp': { icon: markRaw(ImageOutline), color: '#f06292' },
  'bmp': { icon: markRaw(ImageOutline), color: '#f06292' },
  
  // 文档文件
  'pdf': { icon: markRaw(DocumentOutline), color: '#f44336' },
  'doc': { icon: markRaw(DocumentOutline), color: '#2196f3' },
  'docx': { icon: markRaw(DocumentOutline), color: '#2196f3' },
  'txt': { icon: markRaw(DocumentTextOutline), color: '#607d8b' },
  'rtf': { icon: markRaw(DocumentTextOutline), color: '#607d8b' },
  'odt': { icon: markRaw(DocumentOutline), color: '#2196f3' },
  
  // 电子表格
  'xls': { icon: markRaw(ListOutline), color: '#4caf50' },
  'xlsx': { icon: markRaw(ListOutline), color: '#4caf50' },
  'csv': { icon: markRaw(ListOutline), color: '#4caf50' },
  
  // 演示文稿
  'ppt': { icon: markRaw(PrismOutline), color: '#ff9800' },
  'pptx': { icon: markRaw(PrismOutline), color: '#ff9800' },
  
  // 音频文件
  'mp3': { icon: markRaw(MusicalNoteOutline), color: '#9c27b0' },
  'wav': { icon: markRaw(MusicalNoteOutline), color: '#9c27b0' },
  'ogg': { icon: markRaw(MusicalNoteOutline), color: '#9c27b0' },
  'flac': { icon: markRaw(MusicalNoteOutline), color: '#9c27b0' },
  'aac': { icon: markRaw(MusicalNoteOutline), color: '#9c27b0' },
  
  // 视频文件
  'mp4': { icon: markRaw(FilmOutline), color: '#e91e63' },
  'avi': { icon: markRaw(FilmOutline), color: '#e91e63' },
  'mov': { icon: markRaw(FilmOutline), color: '#e91e63' },
  'wmv': { icon: markRaw(FilmOutline), color: '#e91e63' },
  'mkv': { icon: markRaw(FilmOutline), color: '#e91e63' },
  'webm': { icon: markRaw(FilmOutline), color: '#e91e63' },
  
  // 压缩文件
  'zip': { icon: markRaw(ArchiveOutline), color: '#795548' },
  'rar': { icon: markRaw(ArchiveOutline), color: '#795548' },
  '7z': { icon: markRaw(ArchiveOutline), color: '#795548' },
  'tar': { icon: markRaw(ArchiveOutline), color: '#795548' },
  'gz': { icon: markRaw(ArchiveOutline), color: '#795548' },
  
  // 代码文件
  'js': { icon: markRaw(LogoJavascript), color: '#fdd835' },
  'ts': { icon: markRaw(LogoJavascript), color: '#1976d2' },
  'py': { icon: markRaw(LogoPython), color: '#3572A5' },
  'html': { icon: markRaw(LogoHtml5), color: '#e34c26' },
  'css': { icon: markRaw(LogoCss3), color: '#563d7c' },
  'java': { icon: markRaw(CodeOutline), color: '#b07219' },
  'c': { icon: markRaw(CodeOutline), color: '#555555' },
  'cpp': { icon: markRaw(CodeOutline), color: '#f34b7d' },
  'cs': { icon: markRaw(CodeOutline), color: '#178600' },
  'php': { icon: markRaw(CodeOutline), color: '#4F5D95' },
  'go': { icon: markRaw(CodeOutline), color: '#00ADD8' },
  'rb': { icon: markRaw(CodeOutline), color: '#701516' },
  'swift': { icon: markRaw(CodeOutline), color: '#ffac45' },
  'kt': { icon: markRaw(CodeOutline), color: '#A97BFF' },
  
  // 版本控制
  'git': { icon: markRaw(LogoGithub), color: '#F34F29' },
  'gitignore': { icon: markRaw(LogoGithub), color: '#F34F29' },
  
  // 数据库文件
  'db': { icon: markRaw(ServerOutline), color: '#00838f' },
  'sql': { icon: markRaw(ServerOutline), color: '#00838f' },
  'sqlite': { icon: markRaw(ServerOutline), color: '#00838f' },
  
  // 可执行文件
  'exe': { icon: markRaw(SpeedometerOutline), color: '#212121' },
  'msi': { icon: markRaw(SpeedometerOutline), color: '#212121' },
  'dmg': { icon: markRaw(SpeedometerOutline), color: '#212121' },
  'apk': { icon: markRaw(SpeedometerOutline), color: '#a4c639' },
  
  // 加密文件
  'key': { icon: markRaw(DocumentLockOutline), color: '#90a4ae' },
  'cert': { icon: markRaw(DocumentLockOutline), color: '#90a4ae' },
};

// 默认文件图标
const defaultIcon = { icon: markRaw(DocumentAttachOutline), color: '#2080f0' };

export const useFileIconStore = defineStore('fileIcon', () => {
  // 根据文件名或扩展名获取图标
  function getIconByFilename(filename: string): FileIconOption {
    // 如果没有文件名，返回默认图标
    if (!filename) return defaultIcon;
    
    // 获取文件扩展名（不带点）
    const extension = filename.split('.').pop()?.toLowerCase() || '';
    
    // 根据扩展名返回对应图标，如果没有对应的图标，返回默认图标
    return extensionIconMap[extension] || defaultIcon;
  }
  
  // 根据MIME类型获取图标
  function getIconByMimeType(mimeType: string): FileIconOption {
    // 常见MIME类型映射
    if (!mimeType) return defaultIcon;
    
    if (mimeType.startsWith('image/')) {
      return extensionIconMap['png']; // 使用图片图标
    } else if (mimeType.startsWith('audio/')) {
      return extensionIconMap['mp3']; // 使用音频图标
    } else if (mimeType.startsWith('video/')) {
      return extensionIconMap['mp4']; // 使用视频图标
    } else if (mimeType.startsWith('text/')) {
      return extensionIconMap['txt']; // 使用文本图标
    } else if (mimeType === 'application/pdf') {
      return extensionIconMap['pdf'];
    } else if (mimeType.includes('spreadsheet') || mimeType.includes('excel')) {
      return extensionIconMap['xlsx'];
    } else if (mimeType.includes('presentation') || mimeType.includes('powerpoint')) {
      return extensionIconMap['pptx'];
    } else if (mimeType.includes('document') || mimeType.includes('word')) {
      return extensionIconMap['docx'];
    } else if (mimeType.includes('compressed') || mimeType.includes('zip') || mimeType.includes('archive')) {
      return extensionIconMap['zip'];
    }
    
    return defaultIcon;
  }
  
  // 获取所有可用的图标类型
  function getAllIcons(): Record<string, FileIconOption> {
    return extensionIconMap;
  }
  
  return {
    getIconByFilename,
    getIconByMimeType,
    getAllIcons,
    defaultIcon
  };
});