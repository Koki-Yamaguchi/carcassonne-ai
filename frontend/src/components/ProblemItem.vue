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
    class="border rounded-md py-2 bg-white hover:bg-gray-50 hover:cursor-pointer"
    @click="router.push(`/problems/${problem.id}`)"
  >
    {{ problem.name }}
    {{ problem.id }}
  </div>
</template>
