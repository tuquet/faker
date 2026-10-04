<script setup lang="ts">
import { LayoutGrid, TableProperties, CheckCircle2, Loader2, Zap } from 'lucide-vue-next';
import { Badge } from '@tuquet/vue-ui';

interface Props {
  viewMode: 'cards' | 'table';
  isGenerating: boolean;
  totalCount: number;
  sourceText?: string;
  isFallback?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: 'update:viewMode', mode: 'cards' | 'table'): void;
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
            <Badge variant="secondary" class="text-[10px] font-bold uppercase tracking-wider bg-sky-50 text-sky-700 border-sky-200">
              Tuquet Suite
            </Badge>
          </div>
          <p class="text-xs text-slate-500">
            Tauri v2 + Vue 3 • {{ totalCount }} profile hiện tại
          </p>
        </div>
      </div>

      <!-- Center & Right Controls -->
      <div class="flex items-center space-x-3">
        <!-- Status indicator -->
        <Badge v-if="isGenerating" variant="outline" class="gap-1.5 py-1 bg-amber-50 text-amber-700 border-amber-200">
          <Loader2 class="w-3.5 h-3.5 animate-spin" />
          <span>Đang sinh dữ liệu...</span>
        </Badge>
        <Badge v-else variant="outline" class="gap-1.5 py-1 bg-emerald-50 text-emerald-700 border-emerald-200">
          <Zap class="w-3.5 h-3.5 text-emerald-600" />
          <span>{{ sourceText || 'Offline Core (0ms)' }}</span>
        </Badge>

        <!-- View Mode Switcher -->
        <div class="bg-slate-100 p-1 rounded-xl flex items-center gap-1 border border-slate-200">
          <button
            @click="emit('update:viewMode', 'cards')"
            :class="[
              'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer',
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
              'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer',
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
      </div>
    </div>
  </header>
</template>
