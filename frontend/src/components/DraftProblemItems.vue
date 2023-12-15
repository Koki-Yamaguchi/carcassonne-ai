<script setup lang="ts">
import { Problem } from "../types";
import DraftProblemItem from "../components/DraftProblemItem.vue";

defineProps<{
  problems: Problem[];
}>();
</script>

<template>
  <div v-if="problems.length === 0" class="text-sm text-gray-600">
    レビュー待ちの問題はありません。
  </div>
  <div v-else>
    <div v-for="problem in problems" :key="problem.id">
      <DraftProblemItem :problem="problem" />
    </div>
  </div>
</template>
