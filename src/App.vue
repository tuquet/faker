<script setup lang="ts">
import { ref, onMounted } from 'vue';
import HeaderBar from './components/HeaderBar.vue';
import FilterControls from './components/FilterControls.vue';
import UserCardGrid from './components/UserCardGrid.vue';
import UserDataTable from './components/UserDataTable.vue';
import EventLogDrawer from './components/EventLogDrawer.vue';
import type { UserProfile, LogEntry } from './types/user';
import {
  fetchUsersApi,
  saveFileDialog,
  logClientMessage,
  isTauri
} from './services/tauri';
import { generateCSV, downloadBrowserFile } from './lib/utils';

// State
const users = ref<UserProfile[]>([]);
const count = ref(10);
const gender = ref('all');
const nat = ref('all');
const mode = ref<'auto' | 'api' | 'local'>('auto');
const viewMode = ref<'cards' | 'table'>('cards');
const isLoading = ref(false);
const isFallback = ref(false);
const sourceText = ref('');
const isLogOpen = ref(false);
const logs = ref<LogEntry[]>([]);

// Toast Notification
const toastMessage = ref('');
const isToastVisible = ref(false);
let toastTimer: any = null;

function showToast(msg: string) {
  toastMessage.value = msg;
  isToastVisible.value = true;
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => {
    isToastVisible.value = false;
  }, 2200);
}

function addLog(level: 'INFO' | 'SUCCESS' | 'WARN' | 'ERROR', message: string) {
  const now = new Date();
  const time = `${now.toTimeString().split(' ')[0]}.${String(now.getMilliseconds()).padStart(3, '0')}`;
  logs.value.push({
    id: `${Date.now()}-${Math.random()}`,
    time,
    level,
    message,
  });
  logClientMessage(level, message);
}

// Generate Users
async function handleGenerate() {
  if (isLoading.value) return;
  isLoading.value = true;
  const startTime = Date.now();
  addLog('INFO', `Bắt đầu sinh ${count.value} profiles (Gender: ${gender.value}, Nat: ${nat.value}, Mode: ${mode.value})...`);

  try {
    const res = await fetchUsersApi({
      count: count.value,
      gender: gender.value === 'all' ? undefined : gender.value,
      nat: nat.value === 'all' ? undefined : nat.value,
      mode: mode.value,
    });

    users.value = res.results || [];
    isFallback.value = Boolean(res.offlineFallback);
    sourceText.value = res.source || (isFallback.value ? 'Offline Rust' : 'API');

    const duration = Date.now() - startTime;
    if (res.offlineFallback) {
      addLog('WARN', `Hoàn tất với Rust Offline Generator (${duration}ms) - Nguồn: ${sourceText.value}`);
      showToast(`Đã sinh ${users.value.length} profile (Offline Engine)!`);
    } else {
      addLog('SUCCESS', `Đã sinh thành công ${users.value.length} profile (${duration}ms) từ ${sourceText.value}`);
      showToast(`Đã sinh ${users.value.length} profile thành công!`);
    }
  } catch (err: any) {
    addLog('ERROR', `Lỗi khi sinh profile: ${err.message}`);
    showToast(`Lỗi: ${err.message}`);
  } finally {
    isLoading.value = false;
  }
}

// Export CSV
async function handleExportCsv() {
  if (users.value.length === 0) return;
  const csvContent = generateCSV(users.value);
  const filename = `tuquet_users_${Date.now()}.csv`;

  addLog('INFO', `Đang xuất ${users.value.length} dòng ra định dạng CSV...`);

  if (isTauri()) {
    try {
      const savedPath = await saveFileDialog(filename, csvContent, 'csv');
      if (savedPath) {
        addLog('SUCCESS', `Đã lưu file CSV thành công tại: ${savedPath}`);
        showToast('Đã lưu file CSV thành công!');
      } else {
        addLog('INFO', 'Người dùng đã hủy hộp thoại lưu file CSV.');
      }
    } catch (err: any) {
      addLog('ERROR', `Lỗi khi lưu file CSV: ${err.message}`);
    }
  } else {
    downloadBrowserFile(csvContent, filename, 'text/csv;charset=utf-8;');
    addLog('SUCCESS', `Đã tải xuống file ${filename} qua trình duyệt web.`);
    showToast('Đã tải xuống file CSV!');
  }
}

// Export JSON
async function handleExportJson() {
  if (users.value.length === 0) return;
  const jsonContent = JSON.stringify(users.value, null, 2);
  const filename = `tuquet_users_${Date.now()}.json`;

  addLog('INFO', `Đang xuất ${users.value.length} dòng ra định dạng JSON...`);

  if (isTauri()) {
    try {
      const savedPath = await saveFileDialog(filename, jsonContent, 'json');
      if (savedPath) {
        addLog('SUCCESS', `Đã lưu file JSON thành công tại: ${savedPath}`);
        showToast('Đã lưu file JSON thành công!');
      } else {
        addLog('INFO', 'Người dùng đã hủy hộp thoại lưu file JSON.');
      }
    } catch (err: any) {
      addLog('ERROR', `Lỗi khi lưu file JSON: ${err.message}`);
    }
  } else {
    downloadBrowserFile(jsonContent, filename, 'application/json');
    addLog('SUCCESS', `Đã tải xuống file ${filename} qua trình duyệt web.`);
    showToast('Đã tải xuống file JSON!');
  }
}

function onCopied(field: string) {
  showToast(`Đã chép ${field} vào Clipboard!`);
}

onMounted(() => {
  addLog('INFO', `Ứng dụng khởi động (${isTauri() ? 'Tauri Native Desktop' : 'Web Vite Dev'})`);
  handleGenerate();
});
</script>

<template>
  <div class="min-h-full flex flex-col bg-slate-50 text-slate-800 antialiased selection:bg-sky-500 selection:text-white">
    <!-- Header -->
    <HeaderBar
      :view-mode="viewMode"
      @update:view-mode="(mode) => (viewMode = mode)"
      :is-generating="isLoading"
      :total-count="users.length"
      :log-count="logs.length"
      :is-log-open="isLogOpen"
      @toggle-log="isLogOpen = !isLogOpen"
      :source-text="sourceText"
      :is-fallback="isFallback"
    />

    <!-- Main Container -->
    <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
      <!-- Filter & Controls -->
      <FilterControls
        :count="count"
        @update:count="(v) => (count = v)"
        :gender="gender"
        @update:gender="(v) => (gender = v)"
        :nat="nat"
        @update:nat="(v) => (nat = v)"
        :mode="mode"
        @update:mode="(v) => (mode = v)"
        :is-loading="isLoading"
        :total-loaded="users.length"
        @generate="handleGenerate"
        @export-csv="handleExportCsv"
        @export-json="handleExportJson"
      />

      <!-- Content Views (Cards vs Table) -->
      <div v-if="users.length > 0">
        <UserCardGrid
          v-if="viewMode === 'cards'"
          :users="users"
          @copied="onCopied"
        />
        <UserDataTable
          v-else
          :users="users"
          @copied="onCopied"
        />
      </div>

      <!-- Empty State -->
      <div
        v-else-if="!isLoading"
        class="bg-white rounded-2xl border border-slate-200 p-12 text-center space-y-3"
      >
        <div class="w-12 h-12 rounded-2xl bg-sky-50 text-sky-600 flex items-center justify-center mx-auto">
          <svg class="w-6 h-6 fill-current" viewBox="0 0 24 24">
            <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
          </svg>
        </div>
        <h3 class="font-bold text-slate-800 text-sm">Chưa có profile nào</h3>
        <p class="text-xs text-slate-500 max-w-sm mx-auto">
          Nhấn nút "Sinh Profile Mới" ở trên để tạo dữ liệu ngẫu nhiên với động cơ trực tuyến hoặc offline.
        </p>
      </div>
    </main>

    <!-- Event Log Drawer -->
    <EventLogDrawer
      :is-open="isLogOpen"
      :logs="logs"
      @close="isLogOpen = false"
      @clear="logs = []"
    />

    <!-- Global Floating Toast Notification -->
    <transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="transform translate-y-4 opacity-0"
      enter-to-class="transform translate-y-0 opacity-100"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="transform translate-y-0 opacity-100"
      leave-to-class="transform translate-y-4 opacity-0"
    >
      <div
        v-if="isToastVisible"
        class="fixed bottom-6 right-6 z-50 bg-slate-900/95 text-white px-4 py-2.5 rounded-xl shadow-xl flex items-center gap-2.5 text-xs font-semibold backdrop-blur-xs border border-slate-800"
      >
        <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
        <span>{{ toastMessage }}</span>
      </div>
    </transition>
  </div>
</template>
