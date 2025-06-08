<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ArchiveBoxIcon } from '@heroicons/vue/24/outline'

const apiUrl = import.meta.env.VITE_API_URL

const entries = ref<{ key: string; value: string }[]>([])
const loading = ref(false)

async function fetchMemory() {
  loading.value = true
  const res = await fetch(`${apiUrl}/mem/short/all`)
  const data = await res.json()
  console.log('short memory: ', data)
  entries.value = Object.entries(data).map(([key, value]) => ({ key, value }))
  loading.value = false
}

onMounted(fetchMemory)
</script>

<template>
  <div class="space-y-6 max-w-4xl mx-auto">
    <h1 class="flex items-center gap-4 text-xl font-semibold text-color-heading">
      <ArchiveBoxIcon class="size-5" /> Short-Term Memory
    </h1>

    <div v-if="loading" class="text-sm text-color-text font-mono">Loading...</div>

    <div v-else-if="entries.length === 0" class="text-sm text-color-text font-mono">
      No memory entries found.
    </div>

    <ul v-else class="space-y-2">
      <li
        v-for="entry in entries"
        :key="entry.key"
        class="p-3 border border-color-border rounded bg-background-soft font-mono text-sm"
      >
        <strong class="text-color-heading">{{ entry.key }}</strong
        ><br />
        {{ entry.value }}
      </li>
    </ul>
  </div>
</template>
