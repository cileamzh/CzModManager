// 通用输入项的接口
export interface FormItem {
  key: string;
  name: string;
  kind: string; // 'text' | 'number' | 'file' | ...
  options?: any;
}

// 键值对模式的接口
export interface KvGroup {
  mode: 'kv';
  name: string;
}

// 循环模式的接口
export interface LoopGroup {
  mode: 'loop';
  name: string;
  kind: string;
  options?: any;
}

// 最终的 Builder 类型，它是一个数组，每个元素可以是普通输入项、键值对组或循环组
export type DialogBuilder = Array<FormItem | FormItem[] | KvGroup | LoopGroup>;