import { Store } from "@tauri-apps/plugin-store";
import { reactive, ref, Ref, watch } from "vue";
import lan from "./lang.json";
import { Path } from "typescript";
import { invoke } from "@tauri-apps/api/core";
export let store: Store | null;

export const lang: any = lan;

export const img_path = "http://localhost:12981/img";
export const svr_path = "http://localhost:12981";

export const cfg: Ref<any> = ref({
  current_lang: { name: "中文", value: "zh" },
});

export const filters: {
  elements: Set<string>;
  kinds: Set<string>;
} = reactive({
  elements: new Set(),
  kinds: new Set(),
});

export const data: Ref<any> = ref(null);
export const search = ref("");
export const mods: Ref<any> = ref(null);
export const adding_item: Ref<any> = ref(null);
export const previews: Ref<string[]> = ref([]);

export const add_item = () => {
  adding_item.value = {};
};

export async function init() {
  cfg.value = await invoke("get_json_file", { name: "config.json" });

  watch(
    cfg,
    async () => {
      console.log(cfg.value);

      await invoke("set_json_file", {
        name: "config.json",
        data: cfg.value,
      });
    },
    { deep: true }
  );
}

export const get_name = (item: Name) => {
  return (
    item.filter((ele) => ele[0] == cfg.value.current_lang.value).length > 0
      ? item.filter((ele) => ele[0] == cfg.value.current_lang.value)
      : item
  )[0][1];
};

type Name = string[][];

export interface Item {
  name: Name;
  area?: string;
  star?: number;
  element?: string;
  weapon?: string;
  icon?: string;
  label: string;
  kind?: string;
  toped?: boolean;
}

export interface DropFile {
  name: string;
  path: Path;
}

export interface Lang {
  name: string;
  value: string;
}

export interface GameItem {
  name: string;
  icon: string;
  value: string;
  migoto_path: string;
  bg: string;
  game_path: string;
}

export interface GameOption {
  name: Name;
  value: string;
  icon?: string;
}

// 类型定义
export interface ModItem {
  name: string;
  used: boolean;
}

export interface Class {
  icon: string | undefined;
  name: Name;
  value: string;
  filters?: Filter[];
  list: Item[];
}

export interface Filter {
  name: Name;
  key: string;
  list: GameOption[];
}

export interface Game {
  name: Name;
  value: string;
  migoto_path: string;
  game_path: string;
  icon: string;
  classes: Class[];
  bg: string;
}
