<script setup lang="ts">




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


</script>

<template>
  <main class="container">
    <h1>hello tauri </h1>

    <!-- 导航栏 -->
    <nav class="nav">
      <!-- 路由链接 -->
      <router-link to="/home" class="nav-link" active-class="active">首页</router-link>
      <router-link to="/monitor" class="nav-link" active-class="active">监控</router-link>
<!--      <router-link :to="{ name: 'User', params: { id: 1 } }" class="nav-link" active-class="active">用户1</router-link>-->
<!--      <button @click="goToUser(2)">用户2</button>-->
    </nav>

    <!-- 路由出口 - 匹配的组件将在这里渲染 -->
    <router-view />


  </main>
</template>

<style scoped>
.nav {
  padding: 20px;
  margin-bottom: 20px;
}

.nav-link {
  margin-right: 15px;
  text-decoration: none;
  color: #5480ad;
}

.nav-link.active {
  color: #42b983;
  font-weight: bold;
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