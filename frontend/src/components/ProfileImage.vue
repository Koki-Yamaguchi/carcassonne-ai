<script setup lang="ts">
defineProps<{
  src: string;
}>();
</script>
<template>
  <div v-if="src !== ''">
    <img class="w-6 rounded-full" :src="src" />
  </div>
  <div v-else>
    <img class="w-6 rounded-full" src="../assets/img/profile_meeple.png" />
  </div>
</template>
