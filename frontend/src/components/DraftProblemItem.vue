<script setup lang="ts">
import { Problem } from "../types";
import { useFormatDate } from "../composables/date";

defineProps<{
  problem: Problem;
}>();
</script>

<template>
  <a
    :href="`/problems/${problem.id}/edit`"
    class="flex justify-between border rounded-md px-2 py-2 mb-2 bg-white hover:bg-gray-50 hover:cursor-pointer text-gray-700"
  >
    <div>{{ problem.name !== "" ? problem.name : "untitled" }}</div>
    <div v-if="problem.startAt" class="text-sm">
      {{ useFormatDate(problem.startAt) }}
    </div>
    <div class="text-sm">{{ problem.creatorName }}</div>
  </a>
</template>
