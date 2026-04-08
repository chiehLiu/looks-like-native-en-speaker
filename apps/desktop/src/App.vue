<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { LogicalSize } from "@tauri-apps/api/dpi";
import ResultCard from "./components/ResultCard.vue";

interface RewriteResult {
  natural: string;
  professional: string;
  casual: string;
  notes: string[];
}

const input = ref("");
const result = ref<RewriteResult | null>(null);
const loading = ref(false);
const error = ref("");
const copied = ref<string | null>(null);
const textareaRef = ref<HTMLTextAreaElement | null>(null);

const API_KEY = import.meta.env.VITE_ANTHROPIC_API_KEY;

async function rewrite() {
  if (!input.value.trim() || loading.value) return;
  if (!API_KEY || API_KEY === "your-api-key-here") {
    error.value = "Set VITE_ANTHROPIC_API_KEY in .env";
    return;
  }

  loading.value = true;
  error.value = "";
  result.value = null;

  try {
    result.value = await invoke<RewriteResult>("rewrite", {
      text: input.value,
      apiKey: API_KEY,
    });
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function copyToClipboard(text: string, label: string) {
  await writeText(text);
  copied.value = label;
  setTimeout(() => (copied.value = null), 1500);
}

function handleKeydown(e: KeyboardEvent) {
  if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
    e.preventDefault();
    rewrite();
  }
  if (e.key === "Escape") {
    e.preventDefault();
    getCurrentWebviewWindow().hide();
  }
}

// Auto-resize window based on content
async function resizeWindow() {
  await nextTick();
  const app = document.getElementById("app");
  if (!app) return;
  const height = Math.max(160, Math.min(app.scrollHeight + 20, 600));
  const window = getCurrentWebviewWindow();
  await window.setSize(new LogicalSize(480, height));
}

watch([result, error, loading], resizeWindow);

onMounted(() => {
  document.addEventListener("keydown", handleKeydown);
  textareaRef.value?.focus();
});

onUnmounted(() => {
  document.removeEventListener("keydown", handleKeydown);
});
</script>

<template>
  <div class="min-h-screen bg-gray-950 text-gray-100 p-4 select-none font-sans">
    <!-- Input area -->
    <div class="relative">
      <textarea
        ref="textareaRef"
        v-model="input"
        placeholder="Type English text to rewrite..."
        rows="2"
        class="w-full bg-gray-900 border border-gray-700 rounded-lg px-3 py-2 text-sm text-gray-100 placeholder-gray-500 resize-none focus:outline-none focus:border-blue-500 transition-colors"
        :disabled="loading"
      />
      <button
        @click="rewrite"
        :disabled="loading || !input.trim()"
        class="absolute right-2 bottom-2 px-3 py-1 text-xs font-medium rounded-md bg-blue-600 hover:bg-blue-500 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      >
        {{ loading ? "..." : "Rewrite ⌘↵" }}
      </button>
    </div>

    <!-- Error -->
    <p v-if="error" class="mt-2 text-xs text-red-400">{{ error }}</p>

    <!-- Loading -->
    <div v-if="loading" class="mt-4 flex justify-center">
      <div class="w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
    </div>

    <!-- Results -->
    <div v-if="result" class="mt-3 space-y-2">
      <ResultCard
        v-for="item in [
          { label: 'Natural', text: result.natural, key: 'natural' },
          { label: 'Professional', text: result.professional, key: 'professional' },
          { label: 'Casual', text: result.casual, key: 'casual' },
        ]"
        :key="item.key"
        :label="item.label"
        :text="item.text"
        :copied="copied === item.key"
        @copy="copyToClipboard(item.text, item.key)"
      />

      <!-- Notes -->
      <div v-if="result.notes.length" class="mt-2 p-2 bg-gray-900 rounded-lg border border-gray-800">
        <p class="text-[10px] uppercase tracking-wider text-gray-500 mb-1">Changes</p>
        <ul class="text-xs text-gray-400 space-y-0.5">
          <li v-for="(note, i) in result.notes" :key="i">• {{ note }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>
