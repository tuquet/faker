<script setup lang="ts">
import { ref } from 'vue';
import { Mail, Phone, MapPin, Copy, Check, Key, Code2, Download } from 'lucide-vue-next';
import { Card, Badge } from '@tuquet/vue-ui';
import type { UserProfile } from '../types/user';
import { copyToClipboard } from '../lib/utils';

interface Props {
  users: UserProfile[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: 'copied', field: string): void;
  (e: 'downloadAvatar', user: UserProfile): void;
}>();

const copiedKey = ref<string | null>(null);

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
  return `${title}${u.name?.first || ''} ${u.name?.last || ''}`.trim() || 'Người dùng ảo';
}

function getFullAddress(u: UserProfile): string {
  const street = u.location?.street?.name
    ? `${u.location.street.number || ''} ${u.location.street.name}`
    : '';
  const city = u.location?.city || '';
  const country = u.location?.country || '';
  return [street, city, country].filter(Boolean).join(', ');
}
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
    <Card
      v-for="(u, idx) in users"
      :key="u.login?.uuid || idx"
      class="hover:shadow-md hover:border-slate-300 transition-all flex flex-col justify-between overflow-hidden"
    >
      <div class="p-5 space-y-3">
        <!-- Top: Avatar & Name -->
        <div class="flex items-start space-x-3">
          <div class="relative shrink-0 group/avatar">
            <img
              :src="u.picture?.large || u.picture?.medium || 'https://randomuser.me/api/portraits/lego/1.jpg'"
              :alt="getFullName(u)"
              class="w-14 h-14 rounded-2xl object-cover bg-slate-100 border border-slate-200 shadow-2xs"
              loading="lazy"
            />
            <button
              @click.stop="emit('downloadAvatar', u)"
              title="Tải avatar này về máy"
              class="absolute inset-0 bg-slate-900/60 text-white rounded-2xl opacity-0 group-hover/avatar:opacity-100 flex items-center justify-center transition-all cursor-pointer backdrop-blur-2xs"
            >
              <Download class="w-5 h-5 drop-shadow" />
            </button>
            <span
              :class="[
                'absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-white flex items-center justify-center text-[9px] font-bold text-white',
                u.gender === 'female' ? 'bg-pink-500' : 'bg-blue-500'
              ]"
              :title="u.gender === 'female' ? 'Nữ' : 'Nam'"
            >
              {{ u.gender === 'female' ? '♀' : '♂' }}
            </span>
          </div>

          <div class="flex-1 min-w-0">
            <div class="flex items-center justify-between gap-1">
              <Badge variant="secondary" class="text-[10px] font-bold uppercase tracking-wide">
                {{ u.nat || 'GLOBAL' }}
              </Badge>
              <span class="text-xs font-mono font-semibold text-slate-400">#{{ idx + 1 }}</span>
            </div>
            <h3 class="font-bold text-slate-900 text-sm truncate mt-1" :title="getFullName(u)">
              {{ getFullName(u) }}
            </h3>
            <p v-if="u.job" class="text-xs font-semibold text-indigo-600 truncate mt-0.5" :title="u.job">
              {{ u.job }}
            </p>
            <p class="text-xs text-slate-500 truncate flex items-center gap-1 mt-0.5" :title="getFullAddress(u)">
              <MapPin class="w-3 h-3 text-slate-400 shrink-0" />
              <span class="truncate">{{ u.location?.city || u.location?.country || 'N/A' }}</span>
            </p>
          </div>
        </div>

        <!-- Contact Items (One-Click Copy) -->
        <div class="space-y-1.5 pt-2 border-t border-slate-100 text-xs">
          <!-- Email -->
          <div
            @click="handleCopy(u.email, `email-${idx}`, 'Email')"
            class="p-2 rounded-xl bg-slate-50 hover:bg-sky-50/80 border border-slate-100 hover:border-sky-200 transition-all flex items-center justify-between cursor-pointer group"
          >
            <div class="flex items-center gap-2 truncate">
              <Mail class="w-3.5 h-3.5 text-slate-400 group-hover:text-sky-600 shrink-0" />
              <span class="truncate font-medium text-slate-700 group-hover:text-sky-900">{{ u.email }}</span>
            </div>
            <Check v-if="copiedKey === `email-${idx}`" class="w-3.5 h-3.5 text-emerald-600 shrink-0" />
            <Copy v-else class="w-3.5 h-3.5 text-slate-300 group-hover:text-sky-600 opacity-0 group-hover:opacity-100 shrink-0 transition" />
          </div>

          <!-- Phone -->
          <div
            @click="handleCopy(u.phone, `phone-${idx}`, 'Số điện thoại')"
            class="p-2 rounded-xl bg-slate-50 hover:bg-sky-50/80 border border-slate-100 hover:border-sky-200 transition-all flex items-center justify-between cursor-pointer group"
          >
            <div class="flex items-center gap-2 truncate">
              <Phone class="w-3.5 h-3.5 text-slate-400 group-hover:text-sky-600 shrink-0" />
              <span class="truncate font-medium text-slate-700 group-hover:text-sky-900">{{ u.phone }}</span>
            </div>
            <Check v-if="copiedKey === `phone-${idx}`" class="w-3.5 h-3.5 text-emerald-600 shrink-0" />
            <Copy v-else class="w-3.5 h-3.5 text-slate-300 group-hover:text-sky-600 opacity-0 group-hover:opacity-100 shrink-0 transition" />
          </div>

          <!-- Address -->
          <div
            @click="handleCopy(getFullAddress(u), `addr-${idx}`, 'Địa chỉ')"
            class="p-2 rounded-xl bg-slate-50 hover:bg-sky-50/80 border border-slate-100 hover:border-sky-200 transition-all flex items-center justify-between cursor-pointer group"
          >
            <div class="flex items-center gap-2 truncate">
              <MapPin class="w-3.5 h-3.5 text-slate-400 group-hover:text-sky-600 shrink-0" />
              <span class="truncate font-medium text-slate-700 group-hover:text-sky-900">{{ getFullAddress(u) }}</span>
            </div>
            <Check v-if="copiedKey === `addr-${idx}`" class="w-3.5 h-3.5 text-emerald-600 shrink-0" />
            <Copy v-else class="w-3.5 h-3.5 text-slate-300 group-hover:text-sky-600 opacity-0 group-hover:opacity-100 shrink-0 transition" />
          </div>
        </div>

        <!-- Credentials Block -->
        <div class="bg-slate-100/80 p-2.5 rounded-xl border border-slate-200/80 flex items-center justify-between font-mono text-[11px]">
          <div class="truncate mr-2 space-y-0.5">
            <div class="flex items-center gap-1.5 truncate">
              <span class="text-slate-400">User:</span>
              <strong class="text-slate-800 truncate">{{ u.login?.username || 'user' }}</strong>
            </div>
            <div class="flex items-center gap-1.5 truncate">
              <span class="text-slate-400">Pass:</span>
              <span class="text-emerald-700 font-bold truncate">{{ u.login?.password || 'Pass@123' }}</span>
            </div>
          </div>
          <button
            @click="handleCopy(u.login?.password || '', `pass-${idx}`, 'Mật khẩu')"
            title="Copy Mật Khẩu"
            class="p-1.5 rounded-lg bg-white border border-slate-200 hover:bg-slate-50 text-slate-600 hover:text-slate-900 transition shadow-2xs shrink-0"
          >
            <Check v-if="copiedKey === `pass-${idx}`" class="w-3.5 h-3.5 text-emerald-600" />
            <Key v-else class="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      <!-- Footer: Copy raw JSON -->
      <div class="px-5 py-2.5 bg-slate-50/50 border-t border-slate-100 flex items-center justify-between text-[11px] text-slate-500">
        <span>Tuổi: <strong class="text-slate-700">{{ u.dob?.age || '25' }}</strong></span>
        <button
          @click="handleCopy(JSON.stringify(u, null, 2), `json-${idx}`, 'JSON Profile')"
          class="flex items-center gap-1 text-slate-500 hover:text-sky-600 font-semibold transition cursor-pointer"
        >
          <Code2 class="w-3.5 h-3.5" />
          <span>Copy JSON</span>
        </button>
      </div>
    </Card>
  </div>
</template>
