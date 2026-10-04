<script setup lang="ts">
import { computed } from 'vue';
import { cva } from 'class-variance-authority';
import { cn } from '../../lib/utils';

const badgeVariants = cva(
  'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-sky-500 focus:ring-offset-2',
  {
    variants: {
      variant: {
        default: 'border-transparent bg-sky-600 text-white shadow hover:bg-sky-700',
        secondary: 'border-transparent bg-slate-100 text-slate-900 hover:bg-slate-200',
        destructive: 'border-transparent bg-rose-50 text-rose-700 border-rose-200',
        outline: 'text-slate-950 border-slate-200',
        success: 'border-emerald-200 bg-emerald-50 text-emerald-700',
        warning: 'border-amber-200 bg-amber-50 text-amber-700',
        info: 'border-sky-200 bg-sky-50 text-sky-700',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
);

interface BadgeProps {
  variant?: 'default' | 'secondary' | 'destructive' | 'outline' | 'success' | 'warning' | 'info';
  class?: string;
}

const props = defineProps<BadgeProps>();

const classes = computed(() => {
  return cn(badgeVariants({ variant: props.variant }), props.class);
});
</script>

<template>
  <div :class="classes">
    <slot />
  </div>
</template>
