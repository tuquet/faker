<script setup lang="ts">
import { LayoutGrid, TableProperties, Terminal, CheckCircle2, Loader2 } from 'lucide-vue-next';
import Button from './ui/Button.vue';
import Badge from './ui/Badge.vue';

interface Props {
  viewMode: 'cards' | 'table';
  isGenerating: boolean;
  totalCount: number;
  logCount: number;
  isLogOpen: boolean;
  sourceText?: string;
  isFallback?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: 'update:viewMode', mode: 'cards' | 'table'): void;
  (e: 'toggleLog'): void;
}>();
</script>

<template>
  <header class="bg-white border-b border-slate-200 sticky top-0 z-40 shadow-xs">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
      <!-- Logo & Branding -->
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-tr from-sky-600 to-indigo-600 flex items-center justify-center text-white shadow-md shadow-sky-500/20">
          <svg class="w-5 h-5 fill-current" viewBox="0 0 24 24">
            <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
          </svg>
        </div>
        <div>
          <div class="flex items-center gap-2">
            <h1 class="text-base font-bold text-slate-900 leading-tight">
              Random User Generator
            </h1>
            <Badge variant="info" class="text-[10px] font-bold uppercase tracking-wider">
              Tuquet Suite
            </Badge>
          </div>
          <p class="text-xs text-slate-500">
            Tauri v2 + Vue 3 + Tuquet Lib UI • {{ totalCount }} profile hiện tại
          </p>
        </div>
      </div>

      <!-- Center & Right Controls -->
      <div class="flex items-center space-x-3">
        <!-- Status indicator -->
        <Badge v-if="isGenerating" variant="warning" class="gap-1.5 py-1">
          <Loader2 class="w-3.5 h-3.5 animate-spin" />
          <span>Đang sinh dữ liệu...</span>
        </Badge>
        <Badge v-else-if="isFallback" variant="warning" class="gap-1.5 py-1">
          <span class="w-2 h-2 rounded-full bg-amber-500 animate-pulse"></span>
          <span>Offline Rust Core</span>
        </Badge>
        <Badge v-else variant="success" class="gap-1.5 py-1">
          <CheckCircle2 class="w-3.5 h-3.5" />
          <span>{{ sourceText || 'Ready' }}</span>
        </Badge>

        <!-- View Mode Switcher -->
        <div class="bg-slate-100 p-1 rounded-xl flex items-center gap-1 border border-slate-200">
          <button
            @click="emit('update:viewMode', 'cards')"
            :class="[
              'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all',
              viewMode === 'cards'
                ? 'bg-white text-sky-700 shadow-xs'
                : 'text-slate-600 hover:text-slate-900'
            ]"
            title="Chế độ Thẻ (Cards)"
          >
            <LayoutGrid class="w-3.5 h-3.5" />
            <span class="hidden sm:inline">Cards</span>
          </button>
          <button
            @click="emit('update:viewMode', 'table')"
            :class="[
              'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all',
              viewMode === 'table'
                ? 'bg-white text-sky-700 shadow-xs'
                : 'text-slate-600 hover:text-slate-900'
            ]"
            title="Chế độ Bảng (Data Table)"
          >
            <TableProperties class="w-3.5 h-3.5" />
            <span class="hidden sm:inline">Data Table</span>
          </button>
        </div>

        <!-- Terminal Log Drawer Toggle -->
        <Button
          variant="outline"
          size="sm"
          class="gap-1.5 font-medium text-xs relative"
          @click="emit('toggleLog')"
        >
          <Terminal class="w-3.5 h-3.5 text-slate-500" />
          <span class="hidden md:inline">Logs</span>
          <span
            v-if="logCount > 0"
            class="ml-1 px-1.5 py-0.2 bg-slate-200 rounded-full text-[10px] font-mono text-slate-700"
          >
            {{ logCount }}
          </span>
        </Button>
      </div>
    </div>
  </header>
</template>
