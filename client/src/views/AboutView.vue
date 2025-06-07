<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { marked } from 'marked'

const content = ref('')

onMounted(async () => {
  const res = await fetch('README.md')
  const md = await res.text()
  content.value = marked(md)
})
</script>

<template>
  <div class="prose prose-invert max-w-4xl mx-auto text-sm text-color-text" v-html="content" />
</template>
