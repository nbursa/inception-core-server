<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/vue'
import { RouterLink } from 'vue-router'
import {
  ChevronDownIcon,
  SignalIcon,
  ChatBubbleLeftEllipsisIcon,
  Cog6ToothIcon,
} from '@heroicons/vue/24/outline'

const status = ref('Online')

const env = import.meta.env.MODE
</script>

<template>
  <div class="max-w-4xl mx-auto p-6 space-y-6">
    <h1 class="text-2xl font-bold text-heading">🧠 Inception Dev UI</h1>

    <Disclosure
      defaultOpen
      v-slot="{ open }"
      as="section"
      class="bg-background-soft border border-border rounded"
    >
      <DisclosureButton
        class="w-full flex items-center justify-between px-4 py-3 text-left text-lg font-semibold text-heading"
      >
        <span class="flex items-center justify-center gap-4"
          ><SignalIcon class="size-6" /> Status</span
        >
        <ChevronDownIcon
          class="size-5 text-text-secondary"
          :class="open && 'rotate-180 transform'"
        />
      </DisclosureButton>

      <DisclosurePanel v-show="open" class="px-4 pb-4 text-sm text-text font-mono space-y-1">
        <p>
          ICORE: <span class="text-success">{{ status }}</span>
        </p>
        <p>
          Env: <span class="text-accent">{{ env }}</span>
        </p>
      </DisclosurePanel>
    </Disclosure>

    <Disclosure
      defaultOpen
      v-slot="{ open }"
      as="section"
      class="bg-background-soft border border-border rounded"
    >
      <DisclosureButton
        class="w-full flex items-center justify-between px-4 py-3 text-left text-lg font-semibold text-color-heading"
      >
        <span class="flex items-center justify-center gap-4"
          ><ChatBubbleLeftEllipsisIcon class="size-6" /> Quick Tests</span
        >
        <ChevronDownIcon
          class="size-5 text-text-secondary"
          :class="open && 'rotate-180 transform'"
        />
      </DisclosureButton>

      <DisclosurePanel v-show="open" class="px-4 pb-4 text-sm text-text space-y-1">
        <RouterLink to="/chat" class="text-accent hover:underline">Chat prompt tester →</RouterLink
        ><br />
        <RouterLink to="/mem" class="text-accent hover:underline">Memory overview →</RouterLink>
      </DisclosurePanel>
    </Disclosure>

    <Disclosure
      defaultOpen
      v-slot="{ open }"
      as="section"
      class="bg-background-soft border border-border rounded"
    >
      <DisclosureButton
        class="w-full flex items-center justify-between px-4 py-3 text-left text-lg font-semibold text-color-heading"
      >
        <span class="flex items-center justify-center gap-4"
          ><Cog6ToothIcon class="size-6" /> Sentience</span
        >
        <ChevronDownIcon
          class="size-5 text-text-secondary"
          :class="open && 'rotate-180 transform'"
        />
      </DisclosureButton>

      <DisclosurePanel v-show="open" class="px-4 pb-4 text-sm text-color-text">
        Coming soon: REPL, DSL editor, memory cluster analysis.
      </DisclosurePanel>
    </Disclosure>
  </div>
</template>
