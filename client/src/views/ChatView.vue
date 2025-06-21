<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { ChatBubbleLeftEllipsisIcon } from '@heroicons/vue/24/outline'

const apiUrl = import.meta.env.VITE_API_URL

const prompt = ref('')
const response = ref('')
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

async function send() {
  if (!prompt.value.trim()) return
  loading.value = true
  response.value = ''

  try {
    const res = await fetch(`${apiUrl}/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: prompt.value }),
    })

    const data = await res.json()
    // const data = await res.text()

    console.log('data =>', data)
    response.value = data || '[no response]'
  } catch (err) {
    response.value = '[error]'
  } finally {
    loading.value = false
  }
}

onMounted(() => nextTick(resize))
</script>

<template>
  <div class="max-w-2xl w-full mx-auto space-y-6">
    <h1 class="flex items-center gap-4 text-xl font-semibold text-heading">
      <ChatBubbleLeftEllipsisIcon class="size-6" /> Model Chat
    </h1>

    <form @submit.prevent="send" class="space-y-4">
      <fieldset>
        <textarea
          ref="textarea"
          v-model="prompt"
          @input="resize"
          class="w-full bg-background-soft text-text border border-border rounded p-3 font-mono resize-none focus-within:outline-none"
          rows="1"
          placeholder="Enter prompt..."
        />
      </fieldset>

      <div class="flex justify-end">
        <button
          type="submit"
          :disabled="loading || !prompt.trim()"
          class="px-4 py-2 rounded bg-accent text-heading hover:bg-accent-hover disabled:opacity-40 font-mono"
        >
          {{ loading ? '...' : 'Send' }}
        </button>
      </div>
    </form>

    <div
      v-if="response"
      class="border-t pt-4 border-border text-sm whitespace-pre-wrap text-text font-mono"
    >
      <strong class="block mb-1 text-heading">Response:</strong>
      {{ response }}
    </div>
  </div>
</template>
