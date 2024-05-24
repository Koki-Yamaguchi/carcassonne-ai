<script setup lang="ts">
defineProps<{
  isRed: boolean;
  isLarge: boolean;
}>();
</script>
<template>
  <svg
    :class="`${isLarge ? 'h-6 w-6 ' : 'h-4 w-4 '}  ${
      isRed ? 'text-red-500 fill-red-500' : 'text-gray-500 fill-none'
    }`"
    viewBox="0 0 24 24"
    stroke="currentColor"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
    />
  </svg>
</template>
