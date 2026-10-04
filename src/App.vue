<script setup lang="ts">
import { ref, onMounted } from 'vue';
import HeaderBar from './components/HeaderBar.vue';
import FilterControls from './components/FilterControls.vue';
import UserCardGrid from './components/UserCardGrid.vue';
import UserDataTable from './components/UserDataTable.vue';
import type { UserProfile } from './types/user';
import {
  fetchUsersApi,
  saveFileDialog,
  downloadSingleAvatar,
  exportBundleDialog,
  logClientMessage,
  isTauri
} from './services/tauri';
import { generateCSV, downloadBrowserFile } from './lib/utils';

// State
const users = ref<UserProfile[]>([]);
const count = ref(10);
const gender = ref('all');
const nat = ref('vn');
const avatarStyle = ref<'real' | 'svg'>('real');
const mode = ref<'local' | 'api'>('local');
const viewMode = ref<'cards' | 'table'>('cards');
const isLoading = ref(false);
const isFallback = ref(false);
const sourceText = ref('Offline Core (0ms)');

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
  }, 2600);
}

// Generate Users
async function handleGenerate() {
  if (isLoading.value) return;
  isLoading.value = true;

  try {
    const res = await fetchUsersApi({
      count: count.value,
      gender: gender.value === 'all' ? undefined : gender.value,
      nat: nat.value === 'all' ? undefined : nat.value,
      mode: mode.value,
      avatarStyle: avatarStyle.value,
    });

    users.value = res.results || [];
    isFallback.value = Boolean(res.offlineFallback);
    sourceText.value = res.source || (mode.value === 'api' ? 'RandomUser API' : 'Offline Core (0ms)');

    showToast(`Successfully generated ${users.value.length} profiles!`);
    logClientMessage('INFO', `Generated ${users.value.length} profiles (${avatarStyle.value}).`);
  } catch (err: any) {
    showToast(`Error: ${err.message}`);
    logClientMessage('ERROR', `Error generating profiles: ${err.message}`);
  } finally {
    isLoading.value = false;
  }
}

// Download single avatar
async function handleDownloadAvatar(user: UserProfile) {
  const url = user.picture?.large || user.picture?.medium;
  if (!url) return;
  try {
    const res = await downloadSingleAvatar(url, user.login?.username || 'avatar');
    if (res) {
      showToast(`Downloaded avatar for ${user.name.first}!`);
      logClientMessage('SUCCESS', `Downloaded avatar: ${user.login?.username}`);
    }
  } catch (err: any) {
    showToast(`Error downloading avatar: ${err.message}`);
  }
}

// Export CSV
async function handleExportCsv() {
  if (users.value.length === 0) return;
  const csvContent = generateCSV(users.value);
  const filename = `tuquet_users_${Date.now()}.csv`;

  if (isTauri()) {
    try {
      const savedPath = await saveFileDialog(filename, csvContent, 'csv');
      if (savedPath) {
        showToast('CSV file exported successfully!');
        logClientMessage('SUCCESS', `Saved CSV at ${savedPath}`);
      }
    } catch (err: any) {
      showToast(`Error saving CSV: ${err.message}`);
    }
  } else {
    downloadBrowserFile(csvContent, filename, 'text/csv;charset=utf-8;');
    showToast('CSV file downloaded!');
  }
}

// Export JSON
async function handleExportJson() {
  if (users.value.length === 0) return;
  const jsonContent = JSON.stringify(users.value, null, 2);
  const filename = `tuquet_users_${Date.now()}.json`;

  if (isTauri()) {
    try {
      const savedPath = await saveFileDialog(filename, jsonContent, 'json');
      if (savedPath) {
        showToast('JSON file exported successfully!');
        logClientMessage('SUCCESS', `Saved JSON at ${savedPath}`);
      }
    } catch (err: any) {
      showToast(`Error saving JSON: ${err.message}`);
    }
  } else {
    downloadBrowserFile(jsonContent, filename, 'application/json');
    showToast('JSON file downloaded!');
  }
}

// Export Complete Asset Bundle (CSV + JSON + Local HD Avatars)
async function handleExportBundle() {
  if (users.value.length === 0) return;

  if (isTauri()) {
    try {
      const savedFolder = await exportBundleDialog(users.value);
      if (savedFolder) {
        showToast('Full bundle exported with HD avatars folder!');
        logClientMessage('SUCCESS', `Exported bundle at ${savedFolder}`);
      }
    } catch (err: any) {
      showToast(`Export error: ${err.message}`);
      logClientMessage('ERROR', `Export error: ${err.message}`);
    }
  } else {
    // Browser fallback: download both CSV and JSON
    handleExportCsv();
    setTimeout(() => handleExportJson(), 500);
    showToast('Downloading CSV & JSON via browser...');
  }
}

function onCopied(field: string) {
  showToast(`Copied ${field} to clipboard!`);
}

onMounted(() => {
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
        :avatar-style="avatarStyle"
        @update:avatar-style="(v) => { avatarStyle = v; handleGenerate(); }"
        :mode="mode"
        @update:mode="(v) => { mode = v; handleGenerate(); }"
        :is-loading="isLoading"
        :total-loaded="users.length"
        @generate="handleGenerate"
        @export-csv="handleExportCsv"
        @export-json="handleExportJson"
        @export-bundle="handleExportBundle"
      />

      <!-- Content Views (Cards vs Table) -->
      <div v-if="users.length > 0">
        <UserCardGrid
          v-if="viewMode === 'cards'"
          :users="users"
          @copied="onCopied"
          @download-avatar="handleDownloadAvatar"
        />
        <UserDataTable
          v-else
          :users="users"
          @copied="onCopied"
          @download-avatar="handleDownloadAvatar"
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
        <h3 class="font-bold text-slate-800 text-sm">No profiles generated yet</h3>
        <p class="text-xs text-slate-500 max-w-sm mx-auto">
          Click "Generate Profiles" above to generate realistic personas with HD photos or vector avatars.
        </p>
      </div>
    </main>

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
