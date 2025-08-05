<template>
    <div class="viewer" @click="open_flowwin('')" @contextmenu.prevent>
        <div class="fragment" :class="{ icon_fragment: cfg.icon_fragment }">

            <div class="fragment-item" @click="emit('home')"><img :src="svr_path + '/icon/icon.png'">
                <span v-if="!cfg.icon_fragment">CzManager</span>
            </div>

            <div v-for="(class_item, index) in game?.classes" class="fragment-item" @click="current_class = class_item"
                @contextmenu.prevent="context_card($event, class_item, 'class', index)"
                :class="{ selected_class: class_item.value == current_class?.value }">
                <img :src="svr_path + class_item.icon">
                <span v-if="!cfg.icon_fragment">{{ get_name(class_item.name) }}</span>
            </div>

            <div class="fragment-item" @click="add_class">
                <img :src="svr_path + '/icon/add.png'">
                <span v-if="!cfg.icon_fragment">{{ getText('add_class') }}</span>
            </div>
        </div>

        <div class="detail">
            <div class="filter" data-tauri-drag-region>
                <input type="text" v-model="search" class="search-filter">

                <div class="filter-logo center" @click.stop="open_flowwin('show_filter')">
                    <img :src="svr_path + '/icon/filter.png'">
                    <div class="filter-content" :class="{ 'show-filter': status.show_filter }" @click.stop>
                        <div v-for="filter in current_class?.filters" class="filter-item">
                            <div class="center filter-icon" v-for="item in filter.list" :title="get_name(item.name)"
                                @click="toggle_to_filter(item, filter.key)"
                                :class="{ selected: filter_map.get(filter.key)?.has(item.value) }">
                                <span v-if="!item.icon">{{ item.name }} </span>
                                <img v-else :src="svr_path + item.icon">
                            </div>
                        </div>
                    </div>
                </div>

                <div class="filter-logo center" @click.stop="add_filter" v-if="current_class != null">
                    <img :src="svr_path + '/icon/add.png'">
                </div>

                <div class="filter-logo center" @click.stop="open_flowwin('show_sort')">
                    <img :src="svr_path + '/icon/sort.png'">
                    <div class="filter-content" :class="{ 'show-filter': status.show_sort }" @click.stop>
                        <div class="filter-item">
                            <div class="center filter-icon" @click="toggle_sort('name')"
                                :class="{ 'selected': sort.key === 'name' }">
                                <span>{{ getText('name') }}</span>
                                <span v-if="sort.key === 'name'">{{ sort.order === 'asc' ? '↑' : '↓' }}</span>
                            </div>
                            <div class="center filter-icon" @click="toggle_sort('mod_count')"
                                :class="{ 'selected': sort.key === 'mod_count' }">
                                <span>{{ getText('mod_count') }}</span>
                                <span v-if="sort.key === 'mod_count'">{{ sort.order === 'asc' ? '↑' : '↓' }}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="content" :class="{ list_view: cfg.list_view }">
                <div class="relc">
                    <div class="card" :key="item.label" v-for="(item, index) in items_filter" @click="item_click(item)"
                        @contextmenu.prevent="context_card($event, item, 'card', index)"
                        :class="{ selected: item.label == current_item?.label }">
                        <div class="card-border" :class="{ top_card: item.toped }">
                            <img v-if="item.icon !== 'default.png' && item.icon"
                                :src="img_path + '/' + game?.value + '/' + current_class?.value + '/' + item.icon"
                                :alt="item.label">
                            {{ get_name(item.name) }}
                            <span v-if="mods_map">
                                {{ (mods_map.get(item.label)?.used ?? 0) + '/' + (mods_map.get(item.label)?.total ?? 0)
                                }}
                            </span>
                        </div>
                    </div>

                    <div class="card" style="font-size: 60px;color: gray;" @click="add_card"
                        v-if="current_class != null">
                        <div class="card-border">
                            <img :src="svr_path + '/icon/add.png'">
                        </div>
                    </div>
                </div>

                <div class="divide-line"></div>
                <div class="relm">
                    <div class="modlist">
                        <div v-for="(mod, index) in mods" class="mod-item" :key="mod.name"
                            :class="{ selected: mod.name == current_mod?.name }"
                            @contextmenu="context_card($event, mod, 'mod', index)">
                            <div class="mod-border" @click="current_mod = mod">
                                <span> {{ mod.name }}</span>
                                <div class="round" :class="{ used: mod.used }" @click.stop="toggle_mod_status(mod)">
                                </div>
                            </div>
                        </div>
                        <span v-if="mods.length < 1">
                            {{ getText('no_mod') }}
                        </span>
                    </div>

                    <div class="divide-line"></div>
                    <div class="preview">
                        <CzPreview :list="previews"></CzPreview>
                    </div>

                </div>
            </div>
        </div>
    </div>

    <CzDialog ref="dialog"></CzDialog>

    <div class="context-menu card-menu" :class="{ hidden: !card_menu.show }" @contextmenu.stop
        @click="card_menu.show = false" :style="{ left: card_menu.left, top: card_menu.top }">
        <div @click="remove_item((card_menu.data as any).index)" v-if="card_menu.type == 'card'">
            {{ getText('delete') }}
        </div>
        <div @click="add_mod(card_menu.data as Item)" v-if="card_menu.type == 'card'">
            {{ getText('add_mod') }}
        </div>
        <div @click="top_item(card_menu.data as Item)" v-if="card_menu.type == 'card'">
            {{ getText('top') }}
        </div>
        <div @click="delete_mod(card_menu.data as ModItem)" v-if="card_menu.type == 'mod'">
            {{ getText('delete') }}
        </div>

        <div @click="edit_class(card_menu.data as Class)" v-if="card_menu.type == 'class'">
            {{ getText('edit') }}
        </div>
        <div @click="delete_class(card_menu.data as Class, (card_menu.data as any).index)"
            v-if="card_menu.type == 'class'">
            {{ getText('delete') }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, Ref, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
    img_path, Item, lang,
    search,
    ModItem,
    Game,
    svr_path,
    GameOption,
    get_name,
    Class,
    cfg,
    Filter
} from '../var';
import CzDialog from '../component/CzDialog/CzDialog.vue';
import CzPreview from '../component/CzPreview/CzPreview.vue';
import { ask, open } from '@tauri-apps/plugin-dialog';

interface Prop {
    current_lang: string,
    game: Game
}

const emit = defineEmits(['home', 'update-game'])

const { current_lang, game } = defineProps<Prop>()
// 绑定game 自动变更
// 自定义弹窗
const dialog: Ref<any> = ref(null)

/**
 * 辅助函数：根据 key 获取当前语言的文本
 * @param key
 * @returns
 */
const getText = (key: string): string => {
    return lang[current_lang]?.[key] || key;
};

onMounted(() => {
    if (game.classes && game.classes.length > 0) {
        current_class.value = game.classes[0];
    }
})

// 状态控件
const status: any = reactive({
    // filter是否展示
    show_filter: false,
    // sort 是否展示
    show_sort: false
})

const current_class: Ref<Class | null> = ref(null)

const current_item: Ref<Item | null> = ref(null)

const mods: Ref<ModItem[]> = ref([])
const mods_map: Ref<Map<string, { used: number, total: number }>> = ref(new Map());
const current_mod: Ref<ModItem | null> = ref(null)
const previews: Ref<string[]> = ref([]);

// 刷新mod列表和mods_map的函数
const refreshModsAndMap = async (itemLabel: string | undefined = undefined) => {
    if (!game || !current_class.value) return;

    if (itemLabel) {
        // 更新特定卡片的 mods
        const modsList: ModItem[] = await invoke('get_mods', {
            migotoPath: game.migoto_path,
            class: current_class.value.value,
            label: itemLabel
        }) || [];
        mods_map.value.set(itemLabel, {
            used: modsList.filter(m => m.used).length,
            total: modsList.length
        });
    } else {
        // 刷新所有 mods_map
        if (current_class.value.list) {
            current_class.value.list.forEach(async ele => {
                const modsList: ModItem[] = await invoke("get_mods", {
                    migotoPath: game.migoto_path,
                    class: current_class.value?.value,
                    label: ele.label
                }) || [];
                mods_map.value.set(ele.label, {
                    used: modsList.filter(m => m.used).length,
                    total: modsList.length
                });
            });
        }
    }
};

const refreshModsListAndPreview = async () => {
    if (!current_item.value) {
        mods.value = [];
        previews.value = [];
        return;
    }
    mods.value = await invoke('get_mods', {
        migotoPath: game.migoto_path,
        class: current_class.value?.value,
        label: current_item.value?.label
    }) || [];
    current_mod.value = mods.value[0] ?? null;
};

// 监听当前分类的变化，刷新 mods_map
watch(current_class, async (newClass) => {
    filter_map.value = new Map();
    mods_map.value = new Map();
    current_item.value = newClass?.list[0] ?? null;
    await refreshModsAndMap();
}, { deep: true });

// 监听当前卡片的变化，刷新 mods 列表和预览
watch(current_item, async () => {
    await refreshModsListAndPreview();
});

// 监听当前 Mod 的变化，刷新预览
watch(current_mod, async (newMod) => {
    if (newMod && current_item.value) {
        previews.value = (await invoke("get_previews", {
            migotoPath: game.migoto_path,
            class: current_class.value?.value,
            label: current_item.value.label,
            modName: newMod.name
        }) as string[]).map(p => svr_path + "/preview/" + p)
    } else {
        previews.value = []
    }
});

// 右键菜单属性
const card_menu = reactive({
    show: false,
    left: '0px',
    top: '0px',
    data: {},
    type: ''
});

// 右键菜单控件
const context_card = (e: MouseEvent, data: any, type: string, index: number) => {
    data.index = index;
    card_menu.show = false;
    card_menu.left = e.x + 'px';
    card_menu.top = e.y + 'px';
    card_menu.show = true;
    card_menu.data = data;
    card_menu.type = type;
}

const filter_map: Ref<Map<string, Set<string>>> = ref(new Map());
// 排序
const sort = reactive({
    key: 'mod_count', // 默认按名称排序
    order: 'desc', // 默认降序
});

// 切换排序方式
const toggle_sort = (key: string) => {
    if (sort.key === key) {
        sort.order = sort.order === 'asc' ? 'desc' : 'asc';
    } else {
        sort.key = key;
        sort.order = 'asc';
    }
};

// 添加/删去 一个
const toggle_to_filter = (option: GameOption, key: string) => {
    const set = filter_map.value.get(key) ?? new Set<string>();
    if (set.has(option.value)) {
        set.delete(option.value);
    } else {
        set.add(option.value);
    }
    filter_map.value.set(key, set);
}

// 计算属性过滤物品列表
const items_filter = computed(() => {
    // 确保 current_class.value.list 存在，并且进行深拷贝以防意外修改
    if (!current_class.value?.list) return [];
    let items: Item[] = JSON.parse(JSON.stringify(current_class.value.list));

    // 过滤逻辑 (这部分保持不变)
    current_class.value?.filters?.forEach((filter) => {
        const set = filter_map.value.get(filter.key) || new Set();
        if (set.size > 0) {
            items = items?.filter((item) => set.has((item as any)[filter.key] ?? 'NoSuchKey'));
        }
    });

    if (search.value) {
        const lowerCaseSearch = search.value.toLowerCase();
        items = items?.filter((item: Item) =>
            get_name(item.name).toLowerCase().includes(lowerCaseSearch)
        );
    }

    // 排序逻辑 (这是主要修改部分)
    if (items) {
        // 1. 将列表分离为置顶和非置顶两部分
        const pinned_items = items.filter(item => item.toped);
        const regular_items = items.filter(item => !item.toped);

        // 2. 只对非置顶部分进行排序
        regular_items.sort((a, b) => {
            let comparison = 0;
            if (sort.key === 'name') {
                const nameA = get_name(a.name).toLowerCase();
                const nameB = get_name(b.name).toLowerCase();
                if (nameA < nameB) comparison = -1;
                if (nameA > nameB) comparison = 1;
            } else if (sort.key === 'mod_count') {
                const countA = mods_map.value.get(a.label)?.total ?? 0;
                const countB = mods_map.value.get(b.label)?.total ?? 0;
                comparison = countA - countB;
            }
            return sort.order === 'asc' ? comparison : -comparison;
        });

        // 3. 将置顶的项放回列表顶部
        items = [...pinned_items, ...regular_items];
    }

    return items;
});
// 点击物品卡片
const item_click = (item: Item) => {
    current_item.value = item;
};

// 打开弹窗
const open_flowwin = (key: string) => {
    card_menu.show = false;
    Object.entries(status).forEach(ele => {
        if (ele[0] !== key) {
            status[ele[0]] = false;
        } else {
            status[ele[0]] = !ele[1];
        }
    });
};

// 置顶/取消置顶 项
const top_item = (item: Item) => {
    if (!current_class.value?.list) {
        return;
    }
    // 直接在原始数据列表上找到对应的项
    const target_item = current_class.value.list.find(ele => ele.label === item.label);
    if (target_item) {
        // 切换 toped 属性，如果不存在则设为 true
        target_item.toped = !target_item.toped;
        // TODO: 这里需要触发父组件的更新，以保存新的列表状态
        // emit('update-game', game);
        // console.log(`Item ${target_item.label} toped status: ${target_item.toped}`);
    }
};

// 添加mod
const add_mod = async (item: Item) => {
    const modpath = await open({ filters: [{ name: "mod archive", extensions: ['zip', '7z'] }, { name: "all", extensions: [""] }] });
    if (!modpath) return;

    const r = await dialog.value?.open({ title: getText('add_mod'), builder: [{ name: getText('mod_name'), kind: 'text', key: 'mod_name' }] });
    if (!r?.mod_name) return;

    try {
        await invoke("add_mod", {
            filePath: modpath,
            label: item.label,
            class: current_class.value?.value,
            migotoPath: game.migoto_path,
            modName: r['mod_name']
        });
        await refreshModsListAndPreview(); // 刷新 mod 列表
        await refreshModsAndMap(item.label); // 刷新 mod 数量
        alert(getText('mod_add_success'));
    } catch (error) {
        console.error("添加mod失败:", error);
        alert(`${getText('mod_add_fail')}: ${error}`);
    }
};

// 切换mod启用/禁用状态
const toggle_mod_status = async (mod: ModItem) => {
    if (!current_item.value || !current_class.value || !game) return;
    try {
        if (mod.used) {
            await invoke("disable_mod", {
                migotoPath: game.migoto_path,
                modName: mod.name,
                label: current_item.value.label,
                class: current_class.value.value
            });
        } else {
            await invoke("enable_mod", {
                migotoPath: game.migoto_path,
                modName: mod.name,
                label: current_item.value.label,
                class: current_class.value.value
            });
        }
        await refreshModsListAndPreview(); // 刷新 mod 列表
        await refreshModsAndMap(current_item.value.label); // 刷新 mod 数量
    } catch (error) {
        console.error("切换mod状态失败:", error);
        alert(`${getText('mod_status_fail')}: ${error}`);
    }
};

// 删除mod
const delete_mod = async (mod: ModItem) => {
    const confirmed = await ask(getText('submit_delete_mod'), { title: getText('confirm_delete') });
    if (!confirmed || !current_item.value) return;
    try {
        const modToDeleteName = mod.used ? mod.name : `DISABLED_${mod.name}`;
        await invoke("delete_mod", {
            migotoPath: game.migoto_path,
            modName: modToDeleteName,
            label: current_item.value.label,
            class: current_class.value?.value
        });
        await refreshModsListAndPreview(); // 刷新 mod 列表
        await refreshModsAndMap(current_item.value.label); // 刷新 mod 数量
        alert(getText('mod_delete_success'));
    } catch (error) {
        console.error("删除mod失败:", error);
        alert(`${getText('mod_delete_fail')}: ${error}`);
    }
};

const add_card = async () => {
    const r = await dialog.value?.open({
        title: getText('new_card'),
        builder: [
            { name: getText('card_icon'), kind: 'file', key: 'icon' },
            { name: getText('card_name'), kind: 'text', key: 'name' },
            { name: getText('card_label'), kind: 'text', key: 'label' },
            { mode: 'kv', name: getText('card_attr'), key: 'attr' }
        ]
    });
    if (!r?.label || !current_class.value) return;

    try {
        let icon = "default.png";
        if (r.icon) {
            const data: any = await invoke("add_card_icon", { game: game.value, class: current_class.value.value, icon: r.icon });
            if (data.msg === "err") {
                console.log(data);
                return;
            }
            icon = data.file_name;
        }
        const card: any = {};
        card.name = [[current_lang, r.name]];
        card.label = r.label;
        card.icon = icon;
        if (r.attr) {
            r.attr.forEach((a: any) => {
                card[a[0]] = a[1];
            });
        }
        current_class.value.list.push(card);
        await refreshModsAndMap(card.label);
        alert(getText('card_add_success'));
        // TODO: 这里需要触发父组件的更新，以保存新的卡片数据
        // emit('update-game', game);
    } catch (error) {
        console.error("添加卡片失败:", error);
        alert(`${getText('card_add_fail')}: ${error}`);
    }
}

// 删除项
const remove_item = async (index: number) => {
    const confirmed = await ask(getText('submit_delete_current_item'), { title: getText('confirm_delete') });
    if (!confirmed || !current_class.value) return;
    try {
        current_class.value.list.splice(index, 1);
        await refreshModsAndMap();
        alert(getText('item_delete_success'));
        // TODO: 这里需要触发父组件的更新，以保存删除后的列表数据
        // emit('update-game', game);
    } catch (error) {
        console.error("删除物品失败:", error);
        alert(`${getText('item_delete_fail')}: ${error}`);
    }
};

// 新增 Class 相关功能
/**
 * 添加新的类别
 */
const add_class = async () => {
    const r = await dialog.value?.open({
        title: getText('new_class'),
        builder: [
            { name: getText('class_name'), kind: 'text', key: 'name' },
            { name: getText('class_value'), kind: 'text', key: 'value' },
            { name: getText('class_icon'), kind: 'file', key: 'icon' }
        ]
    });
    if (!r?.name || !r?.value || !game.classes) return;

    try {
        let icon = "default.png";
        if (r.icon) {
            const data: any = await invoke("add_class_icon", { game: game.value, class: r.value, icon: r.icon });
            if (data.msg === "err") {
                console.log(data);
                return;
            }
            icon = data.file_name;
        }
        const newClass: Class = {
            name: [[current_lang, r.name]],
            value: r.value,
            icon: icon,
            list: []
        };
        game.classes.push(newClass);
        emit('update-game', game);
        alert(getText('class_add_success'));
    } catch (error) {
        console.error("添加类别失败:", error);
        alert(`${getText('class_add_fail')}: ${error}`);
    }
};

/**
 * 编辑类别信息
 * @param classItem 要编辑的类别对象
 */
const edit_class = async (classItem: Class) => {
    const r = await dialog.value?.open({
        title: getText('edit_class'),
        builder: [
            { name: getText('class_name'), kind: 'text', key: 'name', default: get_name(classItem.name) },
            { name: getText('class_value'), kind: 'text', key: 'value', default: classItem.value, disabled: true },
            { name: getText('class_icon'), kind: 'file', key: 'icon' }
        ]
    });
    if (!r?.name) return;

    try {
        if (r.icon) {
            const data: any = await invoke("add_class_icon", { game: game.value, class: classItem.value, icon: r.icon });
            if (data.msg === "err") {
                console.log(data);
                return;
            }
            classItem.icon = data.file_name;
        }
        classItem.name = [[current_lang, r.name]];
        emit('update-game', game);
        alert(getText('class_edit_success'));
    } catch (error) {
        console.error("编辑类别失败:", error);
        alert(`${getText('class_edit_fail')}: ${error}`);
    }
};

/**
 * 删除类别
 * @param classItem 要删除的类别对象
 * @param index 类别在数组中的索引
 */
const delete_class = async (classItem: Class, index: number) => {
    const confirmed = await ask(getText('submit_delete_class'), { title: getText('confirm_delete') });
    if (!confirmed || !game.classes) return;

    try {
        game.classes.splice(index, 1);
        if (current_class.value?.value === classItem.value) {
            current_class.value = game.classes[0] || null;
        }
        emit('update-game', game);
        alert(getText('class_delete_success'));
    } catch (error) {
        console.error("删除类别失败:", error);
        alert(`${getText('class_delete_fail')}: ${error}`);
    }
};

// 新增 Filter 相关功能
/**
 * 添加新的过滤器
 */
const add_filter = async () => {
    if (!current_class.value) return;

    const r = await dialog.value?.open({
        title: getText('new_filter'),
        builder: [
            { name: getText('filter_key'), kind: 'text', key: 'key' },
            { name: getText('filter_name'), kind: 'text', key: 'name' },
            { mode: 'kv', name: getText('filter_options'), key: 'options' }
        ]
    });

    if (!r?.key || !r?.name || !r?.options) return;

    try {
        const newFilter: Filter = {
            key: r.key,
            name: [[current_lang, r.name]],
            list: r.options.map((opt: [string, string]) => ({
                name: [[current_lang, opt[1]]],
                value: opt[0],
                icon: '' // 默认没有图标
            }))
        };
        if (!current_class.value.filters) {
            current_class.value.filters = [];
        }
        current_class.value.filters.push(newFilter);
        emit('update-game', game);
        alert(getText('filter_add_success'));
    } catch (error) {
        console.error("添加过滤器失败:", error);
        alert(`${getText('filter_add_fail')}: ${error}`);
    }
};
</script>