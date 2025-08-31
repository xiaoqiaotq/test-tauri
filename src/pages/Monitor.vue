<template>
  <div class="container">
    <header>
      <h1>系统监控中心</h1>
      <p>实时监控 CPU、内存和网络状态</p>
    </header>

    <main>
      <!-- 状态卡片 -->
      <div class="stats-grid">
        <div class="stat-card">
          <h3>CPU 使用率</h3>
          <div class="stat-value">{{ cpuUsage.toFixed(1) }}%</div>
          <div class="progress-bar">
            <div
                class="progress"
                :style="{ width: `${cpuUsage}%`, backgroundColor: getColor(cpuUsage) }"
            ></div>
          </div>
        </div>

        <div class="stat-card">
          <h3>内存使用率</h3>
          <div class="stat-value">{{ memoryUsage.toFixed(1) }}%</div>
          <div class="progress-bar">
            <div
                class="progress"
                :style="{ width: `${memoryUsage}%`, backgroundColor: getColor(memoryUsage) }"
            ></div>
          </div>
        </div>

        <div class="stat-card">
          <h3>网络接收</h3>
          <div class="stat-value">{{ formatBytes(networkRxSpeed) }}/s</div>
          <div class="sub-info">总接收: {{ formatBytes(networkRxBytes) }}</div>
        </div>

        <div class="stat-card">
          <h3>网络发送</h3>
          <div class="stat-value">{{ formatBytes(networkTxSpeed) }}/s</div>
          <div class="sub-info">总发送: {{ formatBytes(networkTxBytes) }}</div>
        </div>
      </div>

      <!-- 图表区域 -->
      <div class="charts-grid">
        <div class="chart-container">
          <h3>CPU 使用率趋势</h3>
          <div ref="cpuChartRef" class="chart"></div>
        </div>
        <div class="chart-container">
          <h3>内存使用率趋势</h3>
          <div ref="memoryChartRef" class="chart"></div>
        </div>
        <div class="chart-container">
          <h3>网络流量趋势</h3>
          <div ref="networkChartRef" class="chart"></div>
        </div>
      </div>
    </main>

    <footer>
      <p>系统监控应用 &copy; {{ new Date().getFullYear() }}</p>
    </footer>
  </div>
</template>

<script setup>
import { onMounted, ref, watch } from 'vue';
import * as echarts from 'echarts';
import {  listen } from '@tauri-apps/api/event';

// 状态数据
const cpuUsage = ref(0);
const memoryUsage = ref(0);
const networkRxBytes = ref(0);
const networkTxBytes = ref(0);
const networkRxSpeed = ref(0);
const networkTxSpeed = ref(0);

// 图表引用
const cpuChartRef = ref(null);
const memoryChartRef = ref(null);
const networkChartRef = ref(null);

// 图表实例
let cpuChart = null;
let memoryChart = null;
let networkChart = null;

// 数据历史记录
const cpuHistory = ref(Array(30).fill(0));
const memoryHistory = ref(Array(30).fill(0));
const rxHistory = ref(Array(30).fill(0));
const txHistory = ref(Array(30).fill(0));

// 监听系统信息事件
onMounted(() => {
  // 初始化图表
  initCharts();

  // 监听后端发送的系统信息事件
  const unlisten = listen('system-info', (event) => {
    const data = event.payload;

    // 更新状态数据
    cpuUsage.value = data.cpu.usage;
    memoryUsage.value = data.memory.used_percent;
    networkRxBytes.value = data.network.rx_bytes;
    networkTxBytes.value = data.network.tx_bytes;
    networkRxSpeed.value = data.network.rx_speed;
    networkTxSpeed.value = data.network.tx_speed;

    // 更新历史数据
    updateHistoryData();

    // 更新图表
    updateCharts();
  });

  // 组件卸载时取消监听
  return () => {
    unlisten.then(unlistenFn => unlistenFn());
  };
});

// 初始化图表
function initCharts() {
  // CPU 图表
  cpuChart = echarts.init(cpuChartRef.value);
  cpuChart.setOption({
    tooltip: {
      trigger: 'axis',
      formatter: '{b}: {c}%'
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: Array(30).fill('')
    },
    yAxis: {
      type: 'value',
      min: 0,
      max: 100,
      axisLabel: {
        formatter: '{value}%'
      }
    },
    series: [{
      name: 'CPU 使用率',
      type: 'line',
      data: cpuHistory.value,
      smooth: true,
      lineStyle: {
        width: 2
      },
      areaStyle: {}
    }]
  });

  // 内存图表
  memoryChart = echarts.init(memoryChartRef.value);
  memoryChart.setOption({
    tooltip: {
      trigger: 'axis',
      formatter: '{b}: {c}%'
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: Array(30).fill('')
    },
    yAxis: {
      type: 'value',
      min: 0,
      max: 100,
      axisLabel: {
        formatter: '{value}%'
      }
    },
    series: [{
      name: '内存使用率',
      type: 'line',
      data: memoryHistory.value,
      smooth: true,
      lineStyle: {
        width: 2
      },
      areaStyle: {}
    }]
  });

  // 网络图表
  networkChart = echarts.init(networkChartRef.value);
  networkChart.setOption({
    tooltip: {
      trigger: 'axis',
      formatter: function(params) {
        return `${params[0].name}:
          ${params[0].seriesName}: ${formatBytes(params[0].value)}
          ${params[1].seriesName}: ${formatBytes(params[1].value)}`;
      }
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: Array(30).fill('')
    },
    yAxis: {
      type: 'value',
      axisLabel: {
        formatter: function(value) {
          return formatBytes(value);
        }
      }
    },
    series: [
      {
        name: '接收',
        type: 'line',
        data: rxHistory.value,
        smooth: true,
        lineStyle: {
          width: 2
        },
        areaStyle: {}
      },
      {
        name: '发送',
        type: 'line',
        data: txHistory.value,
        smooth: true,
        lineStyle: {
          width: 2
        },
        areaStyle: {}
      }
    ]
  });

  // 监听窗口大小变化，调整图表尺寸
  window.addEventListener('resize', () => {
    cpuChart.resize();
    memoryChart.resize();
    networkChart.resize();
  });
}

// 更新历史数据
function updateHistoryData() {
  // 更新 CPU 历史
  cpuHistory.value.shift();
  cpuHistory.value.push(cpuUsage.value);

  // 更新内存历史
  memoryHistory.value.shift();
  memoryHistory.value.push(memoryUsage.value);

  // 更新网络历史
  rxHistory.value.shift();
  rxHistory.value.push(networkRxSpeed.value);

  txHistory.value.shift();
  txHistory.value.push(networkTxSpeed.value);
}

// 更新图表数据
function updateCharts() {
  cpuChart.setOption({
    series: [{
      data: cpuHistory.value
    }]
  });

  memoryChart.setOption({
    series: [{
      data: memoryHistory.value
    }]
  });

  networkChart.setOption({
    series: [
      { data: rxHistory.value },
      { data: txHistory.value }
    ]
  });
}

// 格式化字节数显示
function formatBytes(bytes) {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// 根据使用率获取颜色
function getColor(percentage) {
  if (percentage < 30) return '#4CAF50'; // 绿色
  if (percentage < 70) return '#FFC107'; // 黄色
  return '#F44336'; // 红色
}
</script>

<style scoped>
.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 20px;
  font-family: Arial, sans-serif;
  color: #333;
}

header {
  text-align: center;
  margin-bottom: 30px;
}

header h1 {
  font-size: 2rem;
  margin-bottom: 10px;
  color: #4097ec;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.stat-card {
  background-color: #fff;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.stat-card h3 {
  margin-top: 0;
  color: #7f8c8d;
  font-size: 1rem;
  font-weight: 600;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  margin: 10px 0;
  color: #2c3e50;
}

.progress-bar {
  height: 8px;
  background-color: #ecf0f1;
  border-radius: 4px;
  overflow: hidden;
}

.progress {
  height: 100%;
  transition: width 0.3s ease;
}

.sub-info {
  color: #7f8c8d;
  font-size: 0.85rem;
  margin-top: 5px;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 20px;
}

.chart-container {
  background-color: #fff;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.chart-container h3 {
  margin-top: 0;
  color: #2c3e50;
  font-size: 1.1rem;
}

.chart {
  width: 100%;
  height: 300px;
  margin-top: 15px;
}

footer {
  text-align: center;
  margin-top: 40px;
  color: #7f8c8d;
  font-size: 0.9rem;
}

@media (max-width: 768px) {
  .charts-grid {
    grid-template-columns: 1fr;
  }

  .stat-value {
    font-size: 1.5rem;
  }
}
</style>
