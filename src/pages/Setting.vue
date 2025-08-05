<template>
    <div class="setting">
        <h2>{{ getText("ui_setting") }}</h2>
        <div class="setting-item">
            {{ getText('language') }}:
            <CzSelect :selected="cfg.current_lang" @change="(item) => {
                cfg.current_lang = item.data as Lang; // 确保获取的是值
            }" :list="cfg.lang">
            </CzSelect>
        </div>

        <div class="setting-item">
            {{ getText('icon_fragment') }}: <CzSwitch v-model="cfg.icon_fragment" :height="20" theme="modern">
            </CzSwitch>
        </div>

        <div class="setting-item">
            {{ getText('list_view') }}: <CzSwitch v-model="cfg.list_view" :height="20" theme="modern">
            </CzSwitch>
        </div>

        <div class="setting-item">
            {{ getText('style') }}: <CzSelect
                :list="(cfg.style_list || []).map((e: any) => { return { name: get_name(e.name), value: e.value } })"
                @change="(v) => { cfg.current_style = v.data }" :selected="cfg.current_style">
            </CzSelect>
        </div>

        <h2>{{ getText('migoto_path') }}</h2>
        <div class="setting-item" v-for="game in games" :key="game.value">
            <div> {{ get_name(game.name) }}:</div>
            <span> {{ game.migoto_path || getText('path_not_set') }}</span>
            <div class="center" style="margin-left: auto;gap: 5px;">
                <div class="game_edit" @click="delete_migoto_path(game)">✖</div>
                <div class="game_edit" @click="edit_migoto_path(game)">✏️</div>
            </div>
        </div>

        <h2>{{ getText('game_path') }}</h2>
        <div class="setting-item" v-for="game in games" :key="game.value">
            <div>{{ get_name(game.name) }}</div>
            <span> {{ game.game_path || getText('path_not_set') }}</span>
            <div class="center" style="margin-left: auto;gap: 5px;">
                <div class="game_edit" @click="delete_game_path(game)">✖</div>
                <div class="game_edit" @click="edit_game_path(game)">✏️</div>
            </div>
        </div>

    </div>
</template>

<script setup lang="ts">
import { ask, open } from '@tauri-apps/plugin-dialog';
import { store, Lang, Game, get_name, cfg } from '../var';
import { invoke } from '@tauri-apps/api/core';
import CzSelect from '../component/CzSelect/CzSelect.vue';
import CzSwitch from '../component/CzSwitch/CzSwitch.vue';


const games = defineModel<Game[]>()

const { lang, current_lang } = defineProps<{ lang: any, current_lang: any }>()

/**
 * 辅助函数：根据 key 获取当前语言的文本
 */
const getText = (key: string): string => {
    return lang[current_lang]?.[key] || key;
};

// 编辑Migoto路径
async function edit_migoto_path(game: Game) {
    try {
        const pathResult = await open({ directory: true, title: getText('select_migoto_folder') }) as string || "";
        if (!pathResult) return;

        const checkResult = await invoke("check_mods_folder", { path: pathResult });
        if (checkResult === "ok") {
            game.migoto_path = pathResult
            await store?.set(game.value, pathResult); // 存储该游戏的路径
            alert(getText('path_set_success'));
        } else {
            alert(`${getText('invalid_path')}${checkResult}`);
        }
    } catch (error) {
        console.error("Error editing migoto path:", error);
        alert(`${getText('path_edit_error')}${error}`);
    }
}

// 编辑游戏启动路径
async function edit_game_path(game: Game) {
    try {
        const pathResult = await open({ title: getText('select_game_executable'), filters: [{ name: 'Game', extensions: ['exe'] }] }) as string || "";
        if (!pathResult) return;
        game.game_path = pathResult
    } catch (error) {
        console.error("Error editing game path:", error);
        alert(`${getText('path_edit_error')}${error}`);
    }
}

// 删除Migoto路径
async function delete_migoto_path(game: Game) {
    try {
        const confirmed = await ask(getText("submit_delete_path"), { title: getText('confirm_delete') });
        if (confirmed) {
            game.migoto_path = "";
            alert(getText('path_delete_success'));
        }
    } catch (e) {
        console.log(e);
    }
}

// 删除游戏启动路径
async function delete_game_path(game: Game) {
    try {
        const confirmed = await ask(getText("submit_delete_path"), { title: getText('confirm_delete') });
        if (confirmed) {
            game.game_path = "";
            alert(getText('path_delete_success'));
        }
    } catch (e) {
        console.log(e);
    }
}
</script>

<style scoped>
/* 按钮容器 */
.center {
    display: flex;
    align-items: center;
    gap: 8px;
    /* 按钮间距 */
}
</style>