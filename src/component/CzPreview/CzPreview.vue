<template>
    <div class="image-viewer-container">
        <div v-if="list && list.length > 0" class="image-viewer-wrapper">
            <img :src="currentImageSrc" :alt="`预览图片 ${currentIndex + 1}`" class="preview-image"
                @error="handleImageError" />
        </div>
        <div v-else class="no-images">
            没有可预览的图片。
        </div>

        <div v-if="list && list.length > 0" class="controls">
            <div class="nav">
                <button class="nav-button" @click="prevImage" :disabled="currentIndex === 0">
                    &#9664;
                </button>
                <div class="image-index">
                    {{ currentIndex + 1 }} / {{ list.length }}
                </div>
                <button class="nav-button" @click="nextImage" :disabled="currentIndex === list.length - 1">
                    &#9654;
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface Props {
    list: string[]; // 图片 src 字符串数组
    initialIndex?: number; // 初始显示的图片索引，默认为 0
}

const props = withDefaults(defineProps<Props>(), {
    list: () => [], // 默认值为空数组
    initialIndex: 0,
});

const currentIndex = ref(props.initialIndex);

// 计算当前显示图片的 src
const currentImageSrc = computed(() => {
    if (props.list && props.list.length > 0) {
        return props.list[currentIndex.value];
    }
    return '';
});

// 监听 list 属性的变化，如果 list 发生变化，重置 currentIndex
watch(() => props.list, (newList) => {
    if (newList && newList.length > 0) {
        // 确保当前索引不会超出新列表的范围
        if (currentIndex.value >= newList.length) {
            currentIndex.value = newList.length - 1;
        }
    } else {
        currentIndex.value = 0; // 如果列表为空，重置索引
    }
}, { immediate: true }); // 立即执行一次，处理初始值

// 切换到上一张图片
const prevImage = () => {
    if (currentIndex.value > 0) {
        currentIndex.value--;
    }
};

// 切换到下一张图片
const nextImage = () => {
    if (currentIndex.value < props.list.length - 1) {
        currentIndex.value++;
    }
};

// 处理图片加载失败的情况
const handleImageError = (event: Event) => {
    const imgElement = event.target as HTMLImageElement;
    imgElement.src = 'https://via.placeholder.com/150?text=Image+Error'; // 显示一个占位符图片
    console.error('图片加载失败:', imgElement.src);
};
</script>
