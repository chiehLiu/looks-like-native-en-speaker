<script setup lang="ts">
defineProps<{
  label: string;
  text: string;
  copied: boolean;
}>();

defineEmits<{
  copy: [];
}>();
</script>

<template>
  <div
    class="flex items-start gap-2 p-2 bg-gray-900 rounded-lg border border-gray-800 hover:border-gray-700 transition-colors cursor-pointer group"
    @click="$emit('copy')"
  >
    <span class="shrink-0 text-[10px] uppercase tracking-wider text-gray-500 mt-0.5 w-20">{{ label }}</span>
    <p class="flex-1 text-sm text-gray-200 leading-relaxed">{{ text }}</p>
    <button
      class="shrink-0 text-xs text-gray-500 hover:text-gray-300 opacity-0 group-hover:opacity-100 transition-opacity"
      @click.stop="$emit('copy')"
    >
      {{ copied ? "✓" : "Copy" }}
    </button>
  </div>
</template>
