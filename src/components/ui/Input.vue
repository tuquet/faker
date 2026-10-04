<script setup lang="ts">
import { cn } from '../../lib/utils';

interface Props {
  modelValue?: string | number;
  class?: string;
  type?: string;
  placeholder?: string;
  disabled?: boolean;
  min?: number | string;
  max?: number | string;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void;
}>();

function onInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit('update:modelValue', props.type === 'number' ? Number(target.value) : target.value);
}
</script>

<template>
  <input
    :type="type"
    :value="modelValue"
    :disabled="disabled"
    :placeholder="placeholder"
    :min="min"
    :max="max"
    @input="onInput"
    :class="
      cn(
        'flex h-10 w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-sm ring-offset-white file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-slate-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 transition-all font-medium text-slate-800',
        props.class
      )
    "
  />
</template>
