<script setup lang="ts">
import { useRouter } from "vue-router";
import { Problem } from "../types";

defineProps<{
  problem: Problem;
  voted: boolean;
}>();

const router = useRouter();
</script>

<template>
  <div
    class="flex justify-between border rounded-md px-2 py-2 mb-2 bg-white hover:bg-gray-50 hover:cursor-pointer"
  >
    <div class="" @click="router.push(`/problems/${problem.id}`)">
      {{ problem.name }}
    </div>
    <div v-if="voted">
      <img width="20" src="../assets/img/check.png" />
    </div>
    <div v-else></div>
  </div>
</template>
