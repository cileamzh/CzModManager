<template>
    <div class="cz-switch" :class="[themeClass, { 'is-on': is_on }]"
        :style="{ height: height + 'px', width: height * 2 + 'px' }" @click="value = !value">
        <div class="cz-switch-bg"></div>
        <div class="cz-switch-round"></div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const value = defineModel<boolean>();

const props = withDefaults(defineProps<{
    height?: number,
    theme?: 'default' | 'modern' | '3d'
}>(), {
    height: 25,
    theme: 'default'
});

const themeClass = computed(() => `cz-switch-${props.theme}`);
const is_on = computed(() => value.value);
</script>
<style scoped lang="scss">
* {
    box-sizing: border-box;
}

.cz-switch {
    position: relative;
    border-radius: 99px;
    cursor: pointer;
    overflow: hidden;
    transition: box-shadow 0.3s ease;
}

.cz-switch-bg {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: 99px;
    transition: background-color 0.4s ease;
}

.cz-switch-round {
    position: absolute;
    border-radius: 99px;
    top: 0;
    left: 0;
    height: 100%;
    aspect-ratio: 1 / 1;
    transition: transform 0.4s ease;
}

/* ==================== 默认主题 (Default) ==================== */
.cz-switch-default {
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);

    .cz-switch-bg {
        background-color: #eee;
    }

    .cz-switch-round {
        background-color: #fff;
    }

    &.is-on {
        .cz-switch-bg {
            background-color: rgb(40, 140, 255);
        }

        .cz-switch-round {
            transform: translateX(100%);
        }
    }
}

/* ==================== 现代扁平主题 (Modern) ==================== */
.cz-switch-modern {
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);

    .cz-switch-bg {
        background-color: #e0e0e0;
        transition: all 0.3s cubic-bezier(0.2, 0.85, 0.4, 1.275);
    }

    .cz-switch-round {
        background-color: #ffffff;
        transition: all 0.3s cubic-bezier(0.2, 0.85, 0.4, 1.275);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        transform: scale(0.9);
    }

    &.is-on {
        .cz-switch-bg {
            background-color: #42a5f5;
        }

        .cz-switch-round {
            transform: translateX(100%) scale(0.9);
        }
    }
}

/* ==================== 简约立体主题 (3D) ==================== */
.cz-switch-3d {
    box-shadow: inset 0 3px 5px rgba(0, 0, 0, 0.2), inset 0 -3px 5px rgba(255, 255, 255, 0.2);

    .cz-switch-bg {
        background-color: #e0e0e0;
        transition: all 0.4s ease;
    }

    .cz-switch-round {
        background-color: #ffffff;
        transition: all 0.4s ease, transform 0.2s ease-in-out;
        box-shadow: 0 3px 5px rgba(0, 0, 0, 0.3), 0 0 1px rgba(0, 0, 0, 0.1);
    }

    .cz-switch-round:active {
        transform: scale(0.95);
    }

    &.is-on {
        .cz-switch-bg {
            background-color: #32c146;
        }

        .cz-switch-round {
            transform: translateX(100%);
        }
    }
}
</style>