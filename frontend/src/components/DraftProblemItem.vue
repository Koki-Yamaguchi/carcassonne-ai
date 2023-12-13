<script setup lang="ts">
import { useRouter } from "vue-router";
import { Problem } from "../types";

defineProps<{
  problem: Problem;
}>();

const router = useRouter();
</script>

<template>
  <div
    @click="router.push(`/problems/${problem.id}/edit`)"
    class="flex justify-between border rounded-md px-2 py-2 mb-2 bg-white hover:bg-gray-50 hover:cursor-pointer"
  >
    <div class="text-gray-700">untitled</div>
  </div>
</template>
