<script setup lang="ts">
import { onMounted, reactive, Ref, ref, watch, } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { init, Game, svr_path, get_name, cfg, lang } from "./var";
import Setting from "./pages/Setting.vue";
import { invoke } from "@tauri-apps/api/core";
import GameMods from "./pages/GameMods.vue";
import { listen } from "@tauri-apps/api/event";

/**
 * 辅助函数：根据 key 获取当前语言的文本
 * @param key
 * @returns
 */
const getText = (key: string): string => {
  // 首先尝试从当前语言配置中获取文本
  const text = lang[cfg.value.current_lang.value]?.[key];
  // 如果找不到，则直接返回 key 作为备用文本
  return text || key;
};

onMounted(async () => {
  console.log("starting-load");

  await invoke("start_server");
  listen("log", (data) => {
    console.log(data.event + ":", data.payload);
  })
  await init();


  games.value = await invoke("get_games");
  if (games.value.length > 0) {
    game.value = games.value[0];
  }
  console.log(games.value, game.value);

  if (!game || !games) {
    return
  } else {
    loaded.value = true
  }

  watch(games, async () => {
    await invoke("save_games", { games: games.value })
    console.log("已自动保存game数据");
  }, { deep: true })
});

const status = reactive({
  setting_dialog: false,
  viewer: false
})

const loaded = ref(false)

const games: Ref<Game[]> = ref([])

const game: Ref<Game | null> = ref(null);


const open_mod_page = () => {
  if (!game.value?.migoto_path) {
    // 使用 getText 构造提示信息
    return alert(getText('set_migoto_path_alert') + getText('current_game_alert') + get_name(game.value?.name || [[]]))
  }
  status.viewer = !status.viewer;
}

const run_game = async () => {
  if (!game.value?.migoto_path) {
    // 使用 getText
    return alert(getText('set_migoto_path_simple_alert'))
  }
  await invoke('run_game', { migotoPath: game.value.migoto_path, gamePath: game.value.game_path || null })
}

// window control
const currentWindow = getCurrentWindow();
const closeWindow = () => {
  currentWindow.close();
};
const minimizeWindow = () => {
  currentWindow.minimize();
};

const close_settting = () => {
  status.setting_dialog = false
}
</script>

<template>
  <div data-tauri-drag-region class="title-bar center">
    <div class="title-item" @click="status.setting_dialog = true">
      <img :src="svr_path + '/icon/setting.png'" :alt="getText('settings')">
    </div>
    <div class="title-item minimize" @click="minimizeWindow">
      <img :src="svr_path + '/icon/line.png'" :alt="getText('minimize')">
    </div>
    <div class="title-item close" @click="closeWindow">
      <img :src="svr_path + '/icon/close.png'" :alt="getText('close')">
    </div>
  </div>
  <main class="main" v-if="loaded && games.length > 0">
    <link rel="stylesheet" :href="svr_path + '/style.css'">
    <link rel="stylesheet" :href="svr_path + cfg.current_style.value">

    <div data-tauri-drag-region style="height: 20px;width: 100%;;"></div>



    <div class="games">
      <div class="game-content">
        <div v-for="(gameItem, index) in games" :key="index" class="game center"
          :class="{ selected: gameItem.value == game?.value }" @click="game = gameItem">
          <img :src="svr_path + gameItem.icon" :alt="gameItem.value">
        </div>
      </div>
    </div>

    <div class="control">
      <div class="mod-manage center" @click="open_mod_page">
        {{ status.viewer ? getText('home') : getText('mod_manage_btn') }}
      </div>
      <div class="start-game center" @click="run_game">
        <div class="center round">▶</div>{{ getText('run_game') }}
      </div>
    </div>

    <div class="bg">
      <transition name="bg-slide">
        <img :key="game?.bg" :src="svr_path + game?.bg" :alt="getText('game_background')">
      </transition>
    </div>

    <transition name="dialog">
      <div v-if="status.setting_dialog" class="modal" @click="status.setting_dialog = false">
        <div class="modal-close" @click="close_settting">✖</div>
        <Setting @click.stop v-model="games" :lang="lang" :current_lang="cfg.current_lang.value"></Setting>
      </div>
    </transition>
    <transition name="dialog" v-if="game != null">
      <div v-if="status.viewer && game?.migoto_path" class="flow-win">
        <GameMods :game="game" :current_lang="cfg.current_lang.value" @home="status.viewer = false">
        </GameMods>
      </div>
    </transition>
  </main>
  <div v-else id="loading">
    <span v-if="!loaded">{{ getText('loading') }}</span>
    <span v-else>{{ getText('no_game') }}</span>
  </div>


</template>

<style scoped lang="scss">
#loading {
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: gray;

  span {
    font-size: 34px;
  }
}

main {
  height: 100vh;
  width: 100vw;
  background-size: cover;
  background-color: rgba(0, 0, 0, 0.235);
  position: relative;
}

.center {
  display: flex;
  align-items: center;
  justify-content: center;
}

/* --- 标题栏样式 --- */
.title-bar {
  height: 30px;
  justify-content: end;
  padding: 5px;
  gap: 10px;
  position: fixed;
  z-index: 88;
  right: 0px;
  top: 0px;

  .title-item {
    height: 20px;
    width: 20px;
    padding: 2px;
    border-radius: 10px;
    transition: 0.3s;
    user-select: none;
    cursor: pointer;

    img {
      height: 100%;
      width: 100%;
    }

    &:hover {
      background-color: rgba(178, 178, 178, 0.808);
    }

    &.close:hover {
      background-color: rgba(255, 97, 97, 0.856);
    }
  }
}

/* --- 游戏列表样式 --- */
.games {
  position: absolute;
  bottom: 0px;
  left: 0px;
  user-select: none;
  padding: 10px;


  .game-content {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 240px;
    height: 50px;
    position: relative;
    overflow: hidden;
  }

  .game {
    width: 50px;
    height: 50px;
    overflow: hidden;
    border-radius: 10px;
    padding: 3px;
    cursor: pointer;
    transition: 0.3s;
    flex-shrink: 0;

    img {
      border-radius: 10px;
      height: 100%;
      width: 100%;
      object-fit: contain;
    }
  }
}

.games.hidden_game {
  gap: 10px;
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: c;
  bottom: 0px;
  left: 0px;
  z-index: 1;
  user-select: none;
  overflow: hidden;

  .game-content {
    width: 75px;
    height: 50px;
    display: flex;
    overflow: hidden;
  }

  .game {
    width: 50px;
    height: 50px;
    overflow: hidden;
    border-radius: 10px;
    padding: 3px;
    flex-shrink: 0;
    position: absolute;

    img {
      border-radius: 10px;
      height: 100%;
      width: 100%;
      object-fit: contain;
    }
  }
}

.selected {
  background-color: rgba(255, 255, 255, 0.313);
}

/* --- 底部控制按钮样式 --- */
.control {
  position: absolute;
  right: 0px;
  bottom: 0px;
  display: flex;
  height: 50px;
  padding: 0px 10px;
  align-items: center;
  gap: 10px;
  user-select: none;
}

.control .mod-manage {
  background-color: rgba(255, 255, 255, 0.078);
  padding: 10px;
  color: white;
  border-radius: 99px;
  height: 35px;
  transition: 0.3s;
  cursor: pointer;
}

.control .mod-manage:hover {
  background-color: rgba(128, 128, 128, 0.591);
}

.control .start-game {
  background-color: rgb(255, 212, 118);
  gap: 5px;
  border-radius: 15px;
  font-weight: 600;
  padding: 8px;
  transition: 0.3s;
  cursor: pointer;
}

.control .start-game:hover {
  background-color: rgb(255, 189, 47);
}

.control .start-game .round {
  padding: 5px;
  height: 15px;
  width: 15px;
  border-radius: 999px;
  font-size: 5px;
  background-color: rgb(0, 0, 0);
  color: rgb(237, 194, 99);
}

/* --- 背景图片容器样式 --- */
.bg {
  width: 100vw;
  height: 100vh;
  position: fixed;
  top: 0;
  left: 0;
  z-index: -1;
  overflow: hidden;

  /* 关键：图片默认绝对定位，确保能在过渡中同时存在且位置正确 */
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    position: absolute;
    /* 使图片脱离文档流，可以在同一位置重叠 */
    top: 0;
    left: 0;
  }
}

/* --- 背景图片横向滑动过渡动画样式 (调整为交叉过渡) --- */
.bg-slide-enter-active,
.bg-slide-leave-active {
  /* 调整过渡时间，让两者有重叠 */
  transition: transform 1.2s ease, opacity 1.2s ease;
  position: absolute;
  /* 确保过渡中的图片脱离文档流 */
}

/* 新图片进入时的初始状态 */
.bg-slide-enter-from {
  opacity: 0;
  /* 新图片从透明开始 */
  transform: translateX(100%);
  /* 从右侧屏幕外滑入 */
}

/* 新图片进入的最终状态（默认透明度1，位移0） */
.bg-slide-enter-to {
  opacity: 1;
  transform: translateX(0);
}

/* 旧图片离开时的最终状态 */
.bg-slide-leave-to {
  opacity: 0;
  /* 旧图片最终变为完全透明 */
  transform: translateX(-100%);
  /* 向左侧屏幕外滑出 */
}

/* 旧图片离开的初始状态（默认透明度1，位移0） */
.bg-slide-leave-from {
  opacity: 1;
  transform: translateX(0);
}
</style>