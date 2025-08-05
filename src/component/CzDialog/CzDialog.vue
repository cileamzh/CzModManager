<template>
  <div class="cz-dialog" :class="[{ leave: !model, dsnone }]">
    <h2>{{ title }}</h2>
    <div class="dialog-list">
      <div v-for="(builderItem, idx) in exposedBuilder" :key="idx" class="dialog-section">
        <div v-if="'kind' in builderItem" class="dialog-item">
          <label>{{ builderItem.name }}</label>
          <div v-if="builderItem.kind === 'file'" class="file-picker"
            @click="openTauriFile(builderItem.key, builderItem.options)">
            <span class="file-name">{{ values[builderItem.key] || '点击选择文件' }}</span>
          </div>
          <input v-else :type="builderItem.kind" v-model="values[builderItem.key]" />
        </div>
        <div v-else-if="Array.isArray(builderItem)" class="dialog-list-row">
          <div v-for="(mitem, mindex) in builderItem" :key="mindex" class="dialog-item">
            <label>{{ mitem.name }}</label>
            <div v-if="mitem.kind === 'file'" class="file-picker" @click="openTauriFile(mitem.key, mitem.options)">
              <span class="file-name">{{ values[mitem.key] || '点击选择文件' }}</span>
            </div>
            <input v-else :type="mitem.kind" v-model="values[mitem.key]" />
          </div>
        </div>
        <div v-else-if="'mode' in builderItem && builderItem.mode === 'kv'" class="dialog-list-kv">
          <label class="kv-group-label">{{ builderItem.name }}</label>
          <div v-for="(kvItem, kvIndex) in kvItems[idx]" :key="kvIndex" class="kv-item-row">
            <div class="dialog-item dialog-kv-input">
              <label>键</label>
              <input type="text" v-model="kvItem.key" />
            </div>
            <div class="dialog-item dialog-kv-input">
              <label>值</label>
              <input type="text" v-model="kvItem.value" />
            </div>
            <button v-if="kvIndex === kvItems[idx].length - 1" class="add-kv-btn" @click="addKvItem(idx)">
              +
            </button>
            <button v-else-if="kvItems[idx].length > 1" class="remove-kv-btn" @click="removeKvItem(idx, kvIndex)">
              -
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="cz-dialog-control">
      <button class="confirm" @click="onConfirm">确认</button>
      <button class="cancel" @click="onCancel">取消</button>
    </div>
  </div>

  <div class="modal" :class="{ hidden: !(model && exposedModal), dsnone }" @click="onCancel"></div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { open as openDialog } from '@tauri-apps/plugin-dialog';

export interface FormItem {
  key: string;
  name: string;
  kind: string; // 'text' | 'number' | 'file' | ...
  options?: any;
}

// 修复：KvGroup 添加了 key 属性
export interface KvGroup {
  mode: 'kv';
  name: string;
  key: string; 
}

export type DialogBuilder = Array<FormItem | FormItem[] | KvGroup>;

export interface DialogOptions {
  title: string,
  builder: DialogBuilder;
  modalDialog?: boolean;
}

interface KvPair {
  key: string;
  value: any;
}

const title = ref('')

const model = ref(false);
const dsnone = ref(true);

const exposedBuilder = ref<DialogBuilder>([]);
const exposedModal = ref(true);

const values = ref<Record<string, any>>({});
const kvItems = ref<KvPair[][]>([]);

watch(model, (n) => {
  if (!n) {
    setTimeout(() => {
      dsnone.value = true
    }, 500)
  } else {
    dsnone.value = false
  }
})

let _resolve: ((val: Record<string, any> | null) => void) | null = null

function open(options: DialogOptions): Promise<Record<string, any> | null> {
  title.value = options.title
  values.value = {};
  kvItems.value = [];

  exposedBuilder.value = options.builder;
  exposedModal.value = options.modalDialog ?? true;
  model.value = true;
  dsnone.value = false;

  exposedBuilder.value.forEach((item, index) => {
    if (typeof item === 'object' && 'mode' in item) {
      if (item.mode === 'kv') {
        kvItems.value[index] = [{ key: '', value: '' }];
      }
    }
  });

  return new Promise(resolve => {
    _resolve = resolve
  })
}

async function openTauriFile(key: string, options?: any) {
  try {
    const selectedPath = await openDialog(options);
    if (selectedPath) {
      values.value[key] = selectedPath;
    }
  } catch (error) {
    console.error('打开文件对话框失败:', error);
  }
}

function addKvItem(builderIndex: number) {
  const currentKvGroup = kvItems.value[builderIndex];
  const lastItem = currentKvGroup[currentKvGroup.length - 1];
  if (lastItem && lastItem.key.trim() !== '') {
    currentKvGroup.push({ key: '', value: '' });
  }
}

function removeKvItem(builderIndex: number, kvIndex: number) {
  kvItems.value[builderIndex].splice(kvIndex, 1);
}

// 修复后的 onConfirm 方法
function onConfirm() {
  const finalResult: Record<string, any> = {};

  Object.assign(finalResult, values.value);

  exposedBuilder.value.forEach((item, index) => {
    if (typeof item === 'object' && 'mode' in item && item.mode === 'kv') {
      const filteredKv = kvItems.value[index]
        .filter((kv: KvPair) => kv.key.trim() !== '')
        .map((kv: KvPair) => [kv.key, kv.value]);
      
      // 修复：使用 item.key 作为键，将键值对数组赋值给它
      finalResult[item.key] = filteredKv;
    }
  });

  _resolve?.(finalResult);
  _resolve = null;
  closeDialog();
}

function onCancel() {
  _resolve?.(null)
  _resolve = null
  closeDialog()
}

function closeDialog() {
  if (model.value) model.value = false
}

defineExpose({ open })
</script>

<style scoped lang="scss">
/* 样式部分保持不变 */
.cz-dialog {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  min-width: 300px;
  padding: 15px;
  border-radius: 10px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  z-index: 1001;
  display: flex;
  flex-direction: column;
  gap: 15px;
  animation: fadeInUp 0.3s ease;
}

.dialog-list {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.dialog-list-row {
  display: flex;
  gap: 5px;
}

.dialog-section {
  display: contents;
}

.dialog-item {
  display: flex;
  gap: 5px;
  height: 50px;
  align-items: center;
  border-radius: 5px;
  padding: 5px;

  &+& {
    margin-top: -5px;
  }

  input {
    margin-left: auto;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 5px;
    outline: none;
    transition: 0.2s;
    width: 200px;

    &:focus {
      background-color: rgb(243, 243, 243);
    }
  }

  input[type='checkbox'] {
    transform: scale(1.3);
    margin-left: 5px;
  }

  label {
    font-size: 14px;
    color: #555;
  }

  outline: none;
}

/* 动态键值对样式 */
.dialog-list-kv {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.kv-group-label {
  font-weight: bold;
  font-size: 14px;
  margin-bottom: 5px;
}

.kv-item-row {
  display: flex;
  gap: 5px;
  align-items: center;
}

.dialog-kv-input {
  width: 50%;

  label {
    min-width: 20px;
  }

  input {
    width: 100%;
  }
}

.add-kv-btn,
.remove-kv-btn {
  width: 30px;
  height: 30px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  font-size: 18px;
  line-height: 1;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;

  &:hover {
    background-color: #0056b3;
  }
}

.remove-kv-btn {
  background-color: #dc3545;

  &:hover {
    background-color: #c82333;
  }
}

.file-picker {
  margin-left: auto;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 5px;
  background-color: rgb(228, 228, 228);
  cursor: pointer;
  width: 200px;
  height: 30px;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  text-align: left;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;

  .file-name {
    font-size: 12px;
    color: rgb(126, 126, 126);
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
}

.cz-dialog-control {
  display: flex;
  justify-content: flex-end;
  gap: 10px;

  button {
    padding: 6px 12px;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .confirm {
    background: #007bff;
    color: white;

    &:hover {
      background: #0056b3;
    }
  }

  .cancel {
    background: #f0f0f0;
    color: #333;

    &:hover {
      background: #e0e0e0;
    }
  }
}

.modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.3);
  z-index: 1000;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translate(-50%, -60%);
  }

  to {
    opacity: 1;
    transform: translate(-50%, -50%);
  }
}

@keyframes disappear {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    display: none;
  }
}

@keyframes leave {
  from {
    opacity: 1;
    transform: translate(-50%, -50%);
  }

  to {
    transform: translate(-50%, -60%);
    opacity: 0;
    display: none;
  }
}

.leave {
  animation: leave 0.5s ease forwards;
}

.hidden {
  animation: disappear 0.5s ease forwards;
}

.dsnone {
  display: none;
}
</style>