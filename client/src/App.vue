<script setup lang="ts">
import DefaultLayout from '@/layouts/DefaultLayout.vue'
</script>

<template>
  <DefaultLayout v-slot="{ status }">
    <RouterView v-slot="{ Component }">
      <component :is="Component" :status="status" />
    </RouterView>
  </DefaultLayout>
</template>
