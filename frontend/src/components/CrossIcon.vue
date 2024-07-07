<script setup lang="ts">
defineProps<{
  size: "small" | "large" | "tiny";
  color: "gray" | "black";
}>();
</script>

<template>
  <div>
    <svg
      :class="
        (size === 'large'
          ? 'h-8 w-8 '
          : size === 'small'
          ? 'h-6 w-6 '
          : 'h-2 w-2 ') + (color === 'gray' ? 'text-gray-500 ' : 'text-black ')
      "
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M6 18L18 6M6 6l12 12"
      />
    </svg>
  </div>
</template>
