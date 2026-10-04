<script setup lang="ts">
import { Sparkles, FileSpreadsheet, FileCode, FolderArchive, Users, Globe, UserCheck } from 'lucide-vue-next';
import { Button, Input } from '@tuquet/vue-ui';

interface Props {
  count: number;
  gender: string;
  nat: string;
  isLoading: boolean;
  totalLoaded: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: 'update:count', val: number): void;
  (e: 'update:gender', val: string): void;
  (e: 'update:nat', val: string): void;
  (e: 'generate'): void;
  (e: 'exportCsv'): void;
  (e: 'exportJson'): void;
  (e: 'exportBundle'): void;
}>();

const quickCounts = [1, 5, 10, 20, 50, 100];

const natOptions = [
  { value: 'all', label: 'Toàn Cầu (Global)' },
  { value: 'vn', label: 'Việt Nam 🇻🇳' },
  { value: 'us', label: 'Hoa Kỳ 🇺🇸' },
  { value: 'gb', label: 'Vương Quốc Anh 🇬🇧' },
  { value: 'fr', label: 'Pháp 🇫🇷' },
  { value: 'de', label: 'Đức 🇩🇪' },
  { value: 'jp', label: 'Nhật Bản 🇯🇵' },
  { value: 'au', label: 'Úc 🇦🇺' },
  { value: 'ca', label: 'Canada 🇨🇦' },
];
</script>

<template>
  <div class="bg-white rounded-2xl p-5 border border-slate-200 shadow-xs space-y-4">
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      <!-- 1. Count -->
      <div>
        <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider mb-1.5 flex items-center gap-1.5">
          <Users class="w-3.5 h-3.5 text-sky-600" />
          Số Lượng Profile
        </label>
        <Input
          type="number"
          :min="1"
          :max="500"
          :model-value="count"
          @update:model-value="(val) => emit('update:count', Number(val))"
          placeholder="Nhập số lượng..."
          class="font-semibold text-slate-800"
        />
        <div class="flex flex-wrap gap-1 mt-2">
          <button
            v-for="qc in quickCounts"
            :key="qc"
            type="button"
            @click="emit('update:count', qc)"
            :class="[
              'px-2 py-0.5 text-xs font-semibold rounded-md transition-all',
              count === qc
                ? 'bg-sky-600 text-white shadow-2xs'
                : 'bg-slate-100 text-slate-600 hover:bg-slate-200'
            ]"
          >
            {{ qc }}
          </button>
        </div>
      </div>

      <!-- 2. Gender -->
      <div>
        <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider mb-1.5 flex items-center gap-1.5">
          <UserCheck class="w-3.5 h-3.5 text-sky-600" />
          Giới Tính
        </label>
        <select
          :value="gender"
          @change="(e) => emit('update:gender', (e.target as HTMLSelectElement).value)"
          class="flex h-10 w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-sm font-semibold text-slate-800 focus:outline-none focus:ring-2 focus:ring-sky-500"
        >
          <option value="all">Tất cả (Ngẫu nhiên)</option>
          <option value="male">Nam (Male)</option>
          <option value="female">Nữ (Female)</option>
        </select>
        <p class="text-[11px] text-slate-400 mt-2">Chọn giới tính của profile cần sinh.</p>
      </div>

      <!-- 3. Nationality -->
      <div>
        <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider mb-1.5 flex items-center gap-1.5">
          <Globe class="w-3.5 h-3.5 text-sky-600" />
          Quốc Tịch (Nationality)
        </label>
        <select
          :value="nat"
          @change="(e) => emit('update:nat', (e.target as HTMLSelectElement).value)"
          class="flex h-10 w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-sm font-semibold text-slate-800 focus:outline-none focus:ring-2 focus:ring-sky-500"
        >
          <option v-for="opt in natOptions" :key="opt.value" :value="opt.value">
            {{ opt.label }}
          </option>
        </select>
        <p class="text-[11px] text-slate-400 mt-2">Việt Nam ưu tiên từ điển họ tên, nghề nghiệp & CCCD.</p>
      </div>
    </div>

    <!-- Actions Row -->
    <div class="pt-3 border-t border-slate-100 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-2">
        <Button
          variant="default"
          size="default"
          :disabled="isLoading"
          @click="emit('generate')"
          class="gap-2 shadow-md shadow-sky-600/20"
        >
          <Sparkles class="w-4 h-4" />
          <span>{{ isLoading ? 'Đang Xử Lý...' : 'Sinh Profile Mới' }}</span>
        </Button>
      </div>

      <div class="flex flex-wrap items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="totalLoaded === 0 || isLoading"
          @click="emit('exportCsv')"
          class="gap-1.5 font-medium"
        >
          <FileSpreadsheet class="w-4 h-4 text-emerald-600" />
          <span>Xuất CSV</span>
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="totalLoaded === 0 || isLoading"
          @click="emit('exportJson')"
          class="gap-1.5 font-medium"
        >
          <FileCode class="w-4 h-4 text-sky-600" />
          <span>Xuất JSON</span>
        </Button>
        <Button
          variant="default"
          size="sm"
          :disabled="totalLoaded === 0 || isLoading"
          @click="emit('exportBundle')"
          class="gap-1.5 font-medium bg-indigo-600 hover:bg-indigo-700 text-white shadow-xs"
          title="Xuất trọn gói: CSV, JSON và toàn bộ file ảnh avatar SVG offline về máy"
        >
          <FolderArchive class="w-4 h-4 text-white" />
          <span>Xuất Gói Hoàn Chỉnh (Bundle)</span>
        </Button>
      </div>
    </div>
  </div>
</template>
