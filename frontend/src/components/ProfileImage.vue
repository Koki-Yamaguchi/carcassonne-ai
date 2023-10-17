<script setup lang="ts">
defineProps<{
  src: string;
}>();
</script>
<template>
  <div v-if="src !== ''">
    <img class="w-8 h-8 rounded-full object-cover" :src="src" />
  </div>
  <div v-else>
    <img
      class="w-8 h-8 border-2 border-gray-200 rounded-full object-cover"
      src="../assets/img/profile_meeple.png"
    />
  </div>
</template>
