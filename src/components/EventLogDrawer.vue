<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { X, Trash2, Copy, Check, Terminal } from 'lucide-vue-next';
import Button from './ui/Button.vue';
import type { LogEntry } from '../types/user';
import { copyToClipboard } from '../lib/utils';

interface Props {
  isOpen: boolean;
  logs: LogEntry[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'clear'): void;
}>();

const logScrollContainer = ref<HTMLDivElement | null>(null);
const isCopied = ref(false);

watch(
  () => props.logs.length,
  async () => {
    await nextTick();
    if (logScrollContainer.value) {
      logScrollContainer.value.scrollTop = logScrollContainer.value.scrollHeight;
    }
  }
);

async function copyAllLogs() {
  const text = props.logs.map((l) => `[${l.time}] [${l.level}] ${l.message}`).join('\n');
  const success = await copyToClipboard(text);
  if (success) {
    isCopied.value = true;
    setTimeout(() => {
      isCopied.value = false;
    }, 1500);
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-x-0 bottom-0 z-50 bg-slate-900 text-slate-100 border-t border-slate-700 shadow-2xl transition-all max-h-[380px] flex flex-col"
  >
    <!-- Drawer Header -->
    <div class="px-5 py-3 border-b border-slate-800 flex items-center justify-between bg-slate-950/80">
      <div class="flex items-center gap-2">
        <Terminal class="w-4 h-4 text-sky-400" />
        <h3 class="text-xs font-bold uppercase tracking-wider text-slate-200">
          Nhật Ký Sự Kiện Hệ Thống (IPC / Rust Core Logs)
        </h3>
        <span class="px-2 py-0.5 rounded-full text-[10px] font-mono bg-slate-800 text-slate-400">
          {{ logs.length }} logs
        </span>
      </div>

      <div class="flex items-center gap-2">
        <Button
          variant="ghost"
          size="sm"
          @click="copyAllLogs"
          class="h-7 px-2 text-xs text-slate-300 hover:text-white hover:bg-slate-800"
          title="Sao chép toàn bộ logs"
        >
          <Check v-if="isCopied" class="w-3.5 h-3.5 text-emerald-400 mr-1" />
          <Copy v-else class="w-3.5 h-3.5 mr-1" />
          {{ isCopied ? 'Đã chép' : 'Chép logs' }}
        </Button>

        <Button
          variant="ghost"
          size="sm"
          @click="emit('clear')"
          class="h-7 px-2 text-xs text-slate-300 hover:text-white hover:bg-slate-800"
          title="Xóa nhật ký"
        >
          <Trash2 class="w-3.5 h-3.5 mr-1 text-rose-400" />
          Xóa
        </Button>

        <button
          @click="emit('close')"
          class="p-1 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-white transition"
          title="Đóng nhật ký"
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Logs Content -->
    <div
      ref="logScrollContainer"
      class="p-4 overflow-y-auto font-mono text-xs space-y-1.5 flex-1 min-h-[160px] bg-slate-900/90"
    >
      <div
        v-for="l in logs"
        :key="l.id"
        class="flex items-start gap-2 leading-relaxed"
      >
        <span class="text-slate-500 text-[11px] shrink-0">[{{ l.time }}]</span>
        <span
          :class="[
            'px-1.5 py-0.2 rounded text-[10px] font-bold shrink-0',
            l.level === 'SUCCESS' && 'bg-emerald-950 text-emerald-400 border border-emerald-800',
            l.level === 'WARN' && 'bg-amber-950 text-amber-400 border border-amber-800',
            l.level === 'ERROR' && 'bg-rose-950 text-rose-400 border border-rose-800',
            l.level === 'INFO' && 'bg-slate-800 text-sky-400 border border-slate-700'
          ]"
        >
          {{ l.level }}
        </span>
        <span class="text-slate-300 break-all">{{ l.message }}</span>
      </div>

      <div v-if="logs.length === 0" class="text-slate-500 text-center py-6">
        Chưa có sự kiện nào được ghi nhận.
      </div>
    </div>
  </div>
</template>
