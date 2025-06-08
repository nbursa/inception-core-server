<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { RouterLink, RouterView } from 'vue-router'
import { SignalIcon } from '@heroicons/vue/24/outline'

const nav = [
  { path: '/', label: 'Home' },
  { path: '/chat', label: 'Chat' },
  { path: '/mem', label: 'Memory' },
  { path: '/sentience', label: 'Sentience' },
  { path: '/about', label: 'About' },
]

const env = import.meta.env.MODE
const status = ref<'online' | 'offline' | 'loading'>('loading')
let timeoutId: number

async function checkHealth() {
  try {
    const res = await fetch('http://localhost:8080/health')
    status.value = res.ok ? 'online' : 'offline'
  } catch {
    status.value = 'offline'
  }
  // finally {
  //   timeoutId = window.setTimeout(() => {
  //     window.requestIdleCallback(checkHealth)
  //   }, 5000)
  // }
}

onMounted(() => {
  checkHealth()
})

// onBeforeUnmount(() => {
//   clearTimeout(timeoutId)
// })
</script>

<template>
  <div class="flex h-screen w-screen bg-background text-color-text font-mono">
    <!-- Sidebar -->
    <aside class="w-60 bg-background-soft p-4 border-r border-border">
      <div class="text-xl font-bold mb-12">Inception</div>
      <nav class="space-y-2">
        <RouterLink
          v-for="item in nav"
          :key="item.path"
          :to="item.path"
          class="block px-3 py-2 rounded hover:bg-background-mute"
          :class="{ 'bg-background-mute': $route.path === item.path }"
        >
          {{ item.label }}
        </RouterLink>
      </nav>
    </aside>

    <div class="flex-1 flex flex-col">
      <header
        class="h-14 bg-background-soft border-b border-border px-6 flex items-center justify-between"
      >
        <div class="text-sm text-text">
          Env: <span class="capitalize text-accent">{{ env }}</span>
        </div>
        <div
          class="flex items-center gap-1 text-sm font-semibold capitalize"
          :class="{
            'text-success': status === 'online',
            'text-danger': status === 'offline',
            'text-warning': status === 'loading',
          }"
        >
          <SignalIcon class="size-6" /> {{ status }}
        </div>
      </header>

      <main class="flex-1 overflow-y-auto p-6">
        <slot :status="status" />
      </main>
    </div>
  </div>
</template>
