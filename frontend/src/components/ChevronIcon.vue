<script setup lang="ts">
defineProps<{
  direction: "right" | "bottom";
}>();
</script>

<template>
  <svg
    v-if="direction === 'right'"
    class="h-4 w-4 text-gray-500"
    fill="none"
    viewBox="0 0 24 24"
    stroke="currentColor"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M9 5l7 7-7 7"
    />
  </svg>
  <svg
    v-if="direction === 'bottom'"
    class="h-4 w-4 text-gray-500"
    fill="none"
    viewBox="0 0 24 24"
    stroke="currentColor"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M19 9l-7 7-7-7"
    />
  </svg>
</template>
