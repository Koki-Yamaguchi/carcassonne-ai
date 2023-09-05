<script setup lang="ts">
type Kind = "beginning" | "prev" | "next" | "end";

defineProps<{
  kind: Kind;
}>();
</script>
<template>
  <svg
    v-if="kind === 'beginning'"
    class="h-10 w-10 text-gray-500"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    stroke-width="2"
    stroke="currentColor"
    fill="none"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path stroke="none" d="M0 0h24v24H0z" />
    <polyline points="11 7 6 12 11 17" />
    <polyline points="17 7 12 12 17 17" />
  </svg>
  <svg
    v-if="kind === 'prev'"
    class="h-10 w-10 text-gray-500"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    stroke-width="2"
    stroke="currentColor"
    fill="none"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path stroke="none" d="M0 0h24v24H0z" />
    <polyline points="15 6 9 12 15 18" />
  </svg>
  <svg
    v-if="kind === 'next'"
    class="h-10 w-10 text-gray-500"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    stroke-width="2"
    stroke="currentColor"
    fill="none"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path stroke="none" d="M0 0h24v24H0z" />
    <polyline points="9 6 15 12 9 18" />
  </svg>
  <svg
    v-if="kind === 'end'"
    class="h-10 w-10 text-gray-500"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    stroke-width="2"
    stroke="currentColor"
    fill="none"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path stroke="none" d="M0 0h24v24H0z" />
    <polyline points="7 7 12 12 7 17" />
    <polyline points="13 7 18 12 13 17" />
  </svg>
</template>
