// 组件类型扩展
import { DefineComponent } from 'vue';

declare module 'vue' {
  export interface GlobalComponents {
    // 为特定组件扩展插槽
    'n-card': DefineComponent<{}, {}, any, any, any, any, any, {
      'action': () => any;
      'header': () => any;
      'icon': () => any;
      'default': () => any;
      // 添加其他您需要的插槽
    }>;
    
    // 为 Naive UI 扩展通用插槽定义
    'n-thing': DefineComponent<{}, {}, any, any, any, any, any, {
      'action': () => any;
      'header': () => any;
      'avatar': () => any;
      'default': () => any;
      'description': () => any;
      'footer': () => any;
    }>;
    
    'n-list-item': DefineComponent<{}, {}, any, any, any, any, any, {
      'prefix': () => any;
      'suffix': () => any;
      'default': () => any;
      'action': () => any;
    }>;
    
    // 为 Element Plus 扩展
    'el-card': DefineComponent<{}, {}, any, any, any, any, any, {
      'header': () => any;
      'default': () => any;
    }>;
    
    // 为 Ant Design Vue 扩展
    'a-card': DefineComponent<{}, {}, any, any, any, any, any, {
      'actions': () => any;
      'cover': () => any;
      'title': () => any;
      'extra': () => any;
      'default': () => any;
    }>;
    
    // 添加 n-alert 组件的插槽定义
    'n-alert': DefineComponent<{}, {}, any, any, any, any, any, {
      'action': () => any;
      'icon': () => any;
      'default': () => any;
    }>;
  }
  
  // 如果仍然有问题，可以尝试扩展所有具有默认插槽的组件
  export interface AllowedComponentProps {
    // 为所有组件添加 action 插槽支持
    // 注意：这是一种比较激进的方法，可能导致其他类型问题
    // 如果上面的方法无效，可以尝试取消注释这段代码
    // $slots?: {
    //   action?: () => any;
    //   [key: string]: any;
    // }
  }
}
