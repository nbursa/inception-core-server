<script setup lang="ts">
import { ref } from 'vue'
import { Cog6ToothIcon } from '@heroicons/vue/24/outline'

const apiUrl = import.meta.env.VITE_API_URL

const code = ref('')
const output = ref('')
const loading = ref(false)
const textarea = ref<HTMLTextAreaElement>()

const lineHeight = 24
const maxRows = 5
const maxHeight = lineHeight * maxRows

function resize() {
  if (!textarea.value) return

  textarea.value.style.height = 'auto'
  const scroll = textarea.value.scrollHeight
  const limitedHeight = Math.min(scroll, maxHeight)

  textarea.value.style.height = `${limitedHeight}px`
  textarea.value.style.overflowY = scroll > maxHeight ? 'auto' : 'hidden'
}

async function runSentience() {
  if (!code.value.trim()) return
  loading.value = true
  output.value = ''

  const res = await fetch(`${apiUrl}/sentience/run`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ code: code.value }),
  })

  const data = await res.json()
  console.log('data:', data)
  output.value = data.output || '[no response]'
  loading.value = false
}
</script>

<template>
  <div class="space-y-6 max-w-4xl mx-auto">
    <h1 class="flex items-center gap-4 text-xl font-semibold text-color-heading">
      <Cog6ToothIcon class="size-6" /> Sentience evaluator
    </h1>

    <form @submit.prevent="send" class="space-y-4">
      <fieldset>
        <textarea
          ref="textarea"
          v-model="code"
          @input="resize"
          class="w-full bg-background-soft text-text border border-border rounded p-3 font-mono resize-none focus-within:outline-none"
          rows="6"
          placeholder="Enter Sentience code (e.g. reflect { mem.short['foo'] })"
        />
      </fieldset>

      <div class="flex justify-end">
        <button
          @click="runSentience"
          :disabled="loading || !code.trim()"
          class="px-4 py-2 bg-accent text-heading rounded hover:bg-accent-hover disabled:opacity-40 font-mono"
        >
          {{ loading ? '...' : 'Run' }}
        </button>
      </div>
    </form>

    <div
      v-if="output"
      class="border-t border-color-border pt-4 font-mono text-sm text-color-text whitespace-pre-wrap"
    >
      <strong class="block text-heading mb-1">Output:</strong>
      {{ output }}
    </div>
  </div>
</template>
