import { Ref, ref } from "vue";

export const msgs: Ref<Array<Msg>> = ref([]);

// 添加消息函数
export const addMsg = (msg: string, option?: MsgOption) => {
  msgs.value.push({ msg, kind: option?.kind || "common" });
  setTimeout(() => {
    msgs.value.shift();
  }, 2500);
};

export const popMsg = () => {
  msgs.value.pop();
  console.log(1);
};

interface Msg {
  msg: string;
  kind: "common" | "warn" | "danger";
}

export interface MsgOption {
  kind: "common" | "warn" | "danger";
}
