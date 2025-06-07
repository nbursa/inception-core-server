<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'

const nav = [
  { path: '/', label: 'Home' },
  { path: '/chat', label: 'Chat' },
  { path: '/mem', label: 'Memory' },
  { path: '/sentience', label: 'Sentience' },
  { path: '/about', label: 'About' },
]

const env = import.meta.env.MODE
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
        <div class="text-sm text-text">Env: {{ env }}</div>
        <div class="text-sm text-green-500">Status: online</div>
      </header>

      <main class="flex-1 overflow-y-auto p-6">
        <slot />
      </main>
    </div>
  </div>
</template>
