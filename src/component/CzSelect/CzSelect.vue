<template>
    <div class="cz-select">
        <div class="cz-select-title" @click="toggle">
            <div class="icon" v-if="selected?.icon"><img :src="selected?.icon" alt=""></div> {{ selected?.name }}
        </div>
        <div v-show="visible" class="cz-select-body" :class="hidden ? 'wrapSelect' : 'unwrapSelect'"
            @animationend="onAnimationEnd">
            <div v-for="(item, index) in list" class="cz-select-item" @click="clickItem(item, index)">
                <div class="icon" v-if="item.icon"><img :src="item.icon" alt=""></div>{{ item.name }}
            </div>
        </div>

    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ChangeEventValue, SelectItem } from './CzSelect';

interface Props {
    list: SelectItem[] | [],
    selected?: SelectItem
}

defineProps<Props>()

const model = defineModel<SelectItem>()

const emit = defineEmits<{
    change: [ChangeEventValue]
}>()

// 控制是否正在显示元素
const visible = ref(false)
// 控制当前是否正在隐藏中
const hidden = ref(true)


const toggle = () => {
    if (hidden.value) {
        visible.value = true
        hidden.value = false // 播放展开动画
    } else {
        hidden.value = true // 播放收起动画
    }
}

const onAnimationEnd = () => {
    if (hidden.value) {
        visible.value = false // 收起动画结束后彻底隐藏
    }
}

const clickItem = (item: SelectItem, index: number) => {
    hidden.value = true
    model.value = item;
    let data: ChangeEventValue = {
        index,
        data: item
    }
    emit('change', data);
}   
</script>

<style scoped>
.cz-select {
    display: flex;
    flex-direction: column;
    gap: 5px;
    max-width: 100px;
    width: 80px;
    position: relative;
    font-family: 'Franklin Gothic Medium', 'Arial Narrow', Arial, sans-serif;
    font-size: 14px;
    color: rgb(88, 88, 88);
}

.cz-select-title {
    padding: 2px;
    border-radius: 15px;
    display: flex;
    align-items: center;
    justify-content: space-around;
    max-width: 100px;
    height: 30px;
    background-color: rgba(255, 255, 255, 0.543);
    cursor: pointer;
    gap: 5px;
    border: 1px rgba(129, 129, 129, 0.244) solid;
}

.icon {
    height: 100%;
    overflow: hidden;
    object-fit: contain;
    padding: 1.5px;
}

.icon img {
    height: 100%;
    width: 100%;
    border-radius: 20px;
}

.cz-select-body {
    border-radius: 15px;
    position: absolute;
    max-width: 100px;
    overflow-y: auto;
    background-color: rgba(150, 150, 150, 0.548);
    height: 100px;
    padding: 2px;
    border: 1px solid rgba(207, 207, 207, 0.527);
    width: 80px;
    top: 35px;
    z-index: 99;
}

::-webkit-scrollbar {
    width: 0px;
}

.cz-select-item {
    border-radius: 15px;
    background-color: rgba(255, 255, 255, 0.715);
    height: 30px;
    display: flex;
    margin-bottom: 2px;
    align-items: center;
    justify-content: space-around;
    padding: 2px;
    gap: 4px;
}

.wrapSelect {
    animation: wrap 0.5s ease forwards;
}

.unwrapSelect {
    animation: unwrap 0.5s ease forwards;
}

@keyframes wrap {
    from {
        opacity: 1;
        height: 100px;
    }

    to {
        opacity: 0;
        height: 0px;
    }
}

@keyframes unwrap {
    from {
        opacity: 0;
        height: 0px;
    }

    to {
        opacity: 1;
        height: 100px;
    }
}
</style>
