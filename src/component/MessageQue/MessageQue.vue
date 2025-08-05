<script setup lang="ts">
import { msgs, popMsg } from "./messageQue"
</script>
<template>
    <div class="msg-que">
        <div v-for="msg in msgs" class="msg shadow" :class="[msg.kind]">
            {{ msg.msg }} <div @click="popMsg" class="msg-close"><img src="./close.svg"></div>
        </div>
    </div>
</template>
<style scoped lang="scss">
.msg-que {
    width: 100vw;
    height: 100vh;
    position: fixed;
    z-index: 999;
    display: flex;
    pointer-events: none;
    flex-direction: column;
    padding: 10px;
    gap: 10px;
    top: 0px;
    left: 0px;

    .msg {
        background-color: rgb(255, 255, 255);
        display: flex;
        align-items: center;
        border-radius: 8px;
        justify-content: center;
        padding: 5px;
        color: ghostwhite;
        animation: 0.3s fadeIn;
        word-break: break-all;

        .msg-close {
            pointer-events: all;
            margin-left: auto;
            height: 25px;
            padding: 5px;

            img {
                height: 100%;
                object-fit: contain;
            }
        }
    }

    .common {
        background-color: rgb(49, 162, 255);
    }

    .warn {
        background-color: rgb(255, 125, 49);
    }
}
</style>