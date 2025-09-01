<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {getVersion} from '@tauri-apps/api/app';

import {arch, type, version} from '@tauri-apps/plugin-os';
import axios from "axios";

const  osVersion = version();
const osType = type();
const osArch = arch();
const greetMsg = ref("");
const name = ref("");
const appVersion = ref("");

async function getAppVersion() {
  console.log('getAppVersion---')
  let v= await getVersion()
  console.log('getAppVersion2222---',v)
  appVersion.value=v
  return v
}

async function update() {
  console.log('update---')
  return 'update'
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
  // greetMsg.value = name.value+' aaaaa';
}
onMounted(() => {
  console.log('onMounted---')
  getAppVersion()
});

const send = () => {
  axios({
    method: "get",
    // url: "https://httpbin.org/ip",
    url: "https://httpbin.org/uuid",
    // url: "https://httpbin.org/image/png",
  })
      .then(function (resp) {
        greetMsg.value=JSON.stringify(resp.data)
        //resp表示执行成功的结果
        console.log("axios1111-12eedf3331---11dd-", resp);
      })
      .catch((err) => {
        //catch表示执行失败的调用函数 err表示失败的结果
        console.log("请求失败", err);
      });
};

// const initWebsocket = () => {
//   //初始化websocket
//   let planWebsocket: any = null;
//   let planIP: any = "127.0.0.1"; // IP地址
//   let planPort: any = "8765"; // 端口号
//   if ("WebSocket" in window) {
//     planWebsocket = new WebSocket("ws://" + planIP + ":" + planPort); // 通信地址
//     planWebsocket.onopen = function (event: any) {
//       console.log("建立连接", event);
//       let sendData = "你好啊";
//       planWebsocket.send(sendData); // 发送获取数据的接口
//     };
//
//     planWebsocket.onmessage = function (event: any) {
//       console.log("收到消息:" + event.data);
//     };
//
//     planWebsocket.onclose = function (event: any) {
//       console.log("连接关闭", event);
//     };
//
//     planWebsocket.onerror = function () {
//       alert("websocket通信发生错误！");
//     };
//   } else {
//     alert("该浏览器不支持websocket!");
//   }
// };

// 发送websokcet消息
// const sendWebSocket = () => {
//   console.log("发送websocket消息");
//   initWebsocket();
// };
const toSite = () => {
  console.log(window)
  window.location.href='https://xiaoqiaotq.top'
};


</script>

<template>
  <main class="container">

    <!--    <div class="row">-->
    <!--      <a href="https://vitejs.dev" target="_blank">-->
    <!--        <img src="/vite.svg" class="logo vite" alt="Vite logo" />-->
    <!--      </a>-->
    <!--      <a href="https://tauri.app" target="_blank">-->
    <!--        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />-->
    <!--      </a>-->
    <!--      <a href="https://vuejs.org/" target="_blank">-->
    <!--        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />-->
    <!--      </a>-->
    <!--    </div>-->

    <div>
      <div>Os:{{osType}}-{{osArch}} {{osVersion}} AppVersion:{{appVersion}} </div>
      <button @click="send">发送axios请求</button>
      <button @click="update">更新</button>
      &nbsp; &nbsp; &nbsp;
      <!--      <button @click="sendWebSocket">发送Websocket</button>-->
      <button @click="toSite">百度22</button>
    </div>

    <p>Click on 1221</p>

    <a class="navbar-brand" href="#" @click.prevent="$router.push('/monitor')">
      <i class="bi bi-bar-chart-line me-2"></i>系统监控
    </a>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>

  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>