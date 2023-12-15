<script setup lang="ts">
import { Problem } from "../types";
import DraftProblemItem from "../components/DraftProblemItem.vue";
import { translate } from "../locales/translate";

defineProps<{
  problems: Problem[];
}>();
</script>

<template>
  <div v-if="problems.length === 0" class="text-sm text-gray-600">
    {{ translate("no_problems_waiting_for_review") }}
  </div>
  <div v-else>
    <div v-for="problem in problems" :key="problem.id">
      <DraftProblemItem :problem="problem" />
    </div>
  </div>
</template>
