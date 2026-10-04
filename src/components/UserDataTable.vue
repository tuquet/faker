<script setup lang="ts">
import { ref, computed } from 'vue';
import { Search, ArrowUpDown, Copy, Check, Mail, Phone, Key, ChevronLeft, ChevronRight } from 'lucide-vue-next';
import { Badge, Button, Input } from '@tuquet/vue-ui';
import { exportToCsv } from '@tuquet/vue-table';
import type { UserProfile } from '../types/user';
import { copyToClipboard } from '../lib/utils';

interface Props {
  users: UserProfile[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: 'copied', field: string): void;
}>();

const searchQuery = ref('');
const copiedKey = ref<string | null>(null);
const sortKey = ref<'name' | 'gender' | 'nat' | 'email'>('name');
const sortOrder = ref<'asc' | 'desc'>('asc');
const currentPage = ref(1);
const pageSize = ref(15);

async function handleCopy(text: string, key: string, label: string) {
  const success = await copyToClipboard(text);
  if (success) {
    copiedKey.value = key;
    emit('copied', label);
    setTimeout(() => {
      if (copiedKey.value === key) {
        copiedKey.value = null;
      }
    }, 1500);
  }
}

function getFullName(u: UserProfile): string {
  const title = u.name?.title ? `${u.name.title} ` : '';
  return `${title}${u.name?.first || ''} ${u.name?.last || ''}`.trim() || 'N/A';
}

function toggleSort(key: 'name' | 'gender' | 'nat' | 'email') {
  if (sortKey.value === key) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortKey.value = key;
    sortOrder.value = 'asc';
  }
}

const filteredUsers = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  let list = props.users;

  if (q) {
    list = list.filter((u) => {
      const name = getFullName(u).toLowerCase();
      const email = (u.email || '').toLowerCase();
      const phone = (u.phone || '').toLowerCase();
      const city = (u.location?.city || '').toLowerCase();
      const user = (u.login?.username || '').toLowerCase();
      return (
        name.includes(q) ||
        email.includes(q) ||
        phone.includes(q) ||
        city.includes(q) ||
        user.includes(q)
      );
    });
  }

  return [...list].sort((a, b) => {
    let aVal = '';
    let bVal = '';

    if (sortKey.value === 'name') {
      aVal = getFullName(a);
      bVal = getFullName(b);
    } else if (sortKey.value === 'gender') {
      aVal = a.gender || '';
      bVal = b.gender || '';
    } else if (sortKey.value === 'nat') {
      aVal = a.nat || '';
      bVal = b.nat || '';
    } else if (sortKey.value === 'email') {
      aVal = a.email || '';
      bVal = b.email || '';
    }

    const cmp = aVal.localeCompare(bVal);
    return sortOrder.value === 'asc' ? cmp : -cmp;
  });
});

const totalPages = computed(() => {
  return Math.ceil(filteredUsers.value.length / pageSize.value) || 1;
});

const paginatedUsers = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  return filteredUsers.value.slice(start, start + pageSize.value);
});
</script>

<template>
  <div class="bg-white rounded-2xl border border-slate-200 shadow-xs overflow-hidden flex flex-col">
    <!-- Table Toolbar -->
    <div class="p-4 border-b border-slate-200 flex flex-wrap items-center justify-between gap-3 bg-slate-50/50">
      <div class="relative w-full sm:w-72">
        <Search class="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
        <Input
          v-model="searchQuery"
          placeholder="Lọc họ tên, email, SĐT, username..."
          class="pl-9 h-9 text-xs"
        />
      </div>

      <div class="flex items-center gap-3 text-xs text-slate-500 font-medium">
        <span>Hiển thị <strong>{{ paginatedUsers.length }}</strong> / <strong>{{ filteredUsers.length }}</strong> dòng</span>
        <select
          v-model="pageSize"
          class="h-8 rounded-lg border border-slate-200 bg-white px-2 text-xs font-semibold text-slate-700 focus:outline-none focus:ring-1 focus:ring-sky-500"
        >
          <option :value="10">10 dòng/trang</option>
          <option :value="15">15 dòng/trang</option>
          <option :value="25">25 dòng/trang</option>
          <option :value="50">50 dòng/trang</option>
        </select>
      </div>
    </div>

    <!-- Table Container -->
    <div class="overflow-x-auto">
      <table class="w-full text-left border-collapse text-xs">
        <thead>
          <tr class="bg-slate-50 border-b border-slate-200 text-slate-500 font-semibold uppercase tracking-wider text-[11px]">
            <th class="py-3 px-4 w-12 text-center">#</th>
            <th class="py-3 px-4">Avatar</th>
            <th class="py-3 px-4 cursor-pointer select-none hover:text-slate-900" @click="toggleSort('name')">
              <div class="flex items-center gap-1.5">
                <span>Họ Tên</span>
                <ArrowUpDown class="w-3 h-3 text-slate-400" />
              </div>
            </th>
            <th class="py-3 px-4 cursor-pointer select-none hover:text-slate-900" @click="toggleSort('gender')">
              <div class="flex items-center gap-1.5">
                <span>Giới Tính</span>
                <ArrowUpDown class="w-3 h-3 text-slate-400" />
              </div>
            </th>
            <th class="py-3 px-4 cursor-pointer select-none hover:text-slate-900" @click="toggleSort('nat')">
              <div class="flex items-center gap-1.5">
                <span>Quốc Tịch</span>
                <ArrowUpDown class="w-3 h-3 text-slate-400" />
              </div>
            </th>
            <th class="py-3 px-4 cursor-pointer select-none hover:text-slate-900" @click="toggleSort('email')">
              <div class="flex items-center gap-1.5">
                <span>Email</span>
                <ArrowUpDown class="w-3 h-3 text-slate-400" />
              </div>
            </th>
            <th class="py-3 px-4">Số Điện Thoại</th>
            <th class="py-3 px-4">Địa Chỉ / Khu Vực</th>
            <th class="py-3 px-4">Tài Khoản / Mật Khẩu</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100">
          <tr
            v-for="(u, idx) in paginatedUsers"
            :key="u.login?.uuid || idx"
            class="hover:bg-sky-50/40 transition-colors"
          >
            <td class="py-2.5 px-4 text-center font-mono text-slate-400 font-medium">
              {{ (currentPage - 1) * pageSize + idx + 1 }}
            </td>
            <td class="py-2.5 px-4">
              <img
                :src="u.picture?.thumbnail || u.picture?.medium || 'https://api.dicebear.com/7.x/avataaars/svg?seed=' + idx"
                class="w-8 h-8 rounded-lg object-cover bg-slate-100 border border-slate-200"
                loading="lazy"
              />
            </td>
            <td class="py-2.5 px-4 font-semibold text-slate-900 whitespace-nowrap">
              {{ getFullName(u) }}
            </td>
            <td class="py-2.5 px-4 whitespace-nowrap">
              <Badge
                variant="outline"
                :class="[
                  'text-[10px]',
                  u.gender === 'female' ? 'bg-pink-50 text-pink-700 border-pink-200' : 'bg-blue-50 text-blue-700 border-blue-200'
                ]"
              >
                {{ u.gender === 'female' ? 'Nữ ♀' : 'Nam ♂' }}
              </Badge>
            </td>
            <td class="py-2.5 px-4 whitespace-nowrap">
              <Badge variant="secondary" class="text-[10px] font-mono font-bold">
                {{ u.nat || 'GLOBAL' }}
              </Badge>
            </td>
            <td class="py-2.5 px-4">
              <button
                @click="handleCopy(u.email, `tbl-email-${idx}`, 'Email')"
                class="flex items-center gap-1.5 text-slate-700 hover:text-sky-600 font-mono transition text-[11px] group cursor-pointer"
                title="Bấm để copy Email"
              >
                <span class="truncate max-w-[180px]">{{ u.email }}</span>
                <Check v-if="copiedKey === `tbl-email-${idx}`" class="w-3 h-3 text-emerald-600 shrink-0" />
                <Copy v-else class="w-3 h-3 text-slate-300 opacity-0 group-hover:opacity-100 shrink-0" />
              </button>
            </td>
            <td class="py-2.5 px-4">
              <button
                @click="handleCopy(u.phone, `tbl-phone-${idx}`, 'Số điện thoại')"
                class="flex items-center gap-1.5 text-slate-700 hover:text-sky-600 font-mono transition text-[11px] group cursor-pointer"
                title="Bấm để copy SĐT"
              >
                <span class="truncate max-w-[120px]">{{ u.phone }}</span>
                <Check v-if="copiedKey === `tbl-phone-${idx}`" class="w-3 h-3 text-emerald-600 shrink-0" />
                <Copy v-else class="w-3 h-3 text-slate-300 opacity-0 group-hover:opacity-100 shrink-0" />
              </button>
            </td>
            <td class="py-2.5 px-4 max-w-[180px] truncate text-slate-600" :title="`${u.location?.city || ''}, ${u.location?.country || ''}`">
              {{ u.location?.city || '' }}, {{ u.location?.country || '' }}
            </td>
            <td class="py-2.5 px-4 whitespace-nowrap">
              <div class="flex items-center gap-2 font-mono text-[11px]">
                <span class="text-slate-800 font-semibold">{{ u.login?.username || 'user' }}</span>
                <span class="text-slate-300">/</span>
                <button
                  @click="handleCopy(u.login?.password || '', `tbl-pass-${idx}`, 'Mật khẩu')"
                  class="flex items-center gap-1 text-emerald-700 font-bold hover:underline cursor-pointer"
                  title="Bấm để copy mật khẩu"
                >
                  <span>{{ u.login?.password || 'Pass@123' }}</span>
                  <Check v-if="copiedKey === `tbl-pass-${idx}`" class="w-3 h-3 text-emerald-600" />
                  <Key v-else class="w-3 h-3 text-slate-400" />
                </button>
              </div>
            </td>
          </tr>

          <tr v-if="filteredUsers.length === 0">
            <td colspan="9" class="py-8 text-center text-slate-400">
              Không tìm thấy người dùng nào phù hợp với bộ lọc.
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Table Pagination -->
    <div class="p-3 border-t border-slate-200 bg-slate-50/50 flex items-center justify-between text-xs text-slate-600">
      <div>
        Trang <strong>{{ currentPage }}</strong> / <strong>{{ totalPages }}</strong>
      </div>
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="currentPage <= 1"
          @click="currentPage--"
          class="h-8 px-2 gap-1 text-xs"
        >
          <ChevronLeft class="w-3.5 h-3.5" />
          Trước
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="currentPage >= totalPages"
          @click="currentPage++"
          class="h-8 px-2 gap-1 text-xs"
        >
          Sau
          <ChevronRight class="w-3.5 h-3.5" />
        </Button>
      </div>
    </div>
  </div>
</template>
