<script setup lang="ts">
import { onMounted, ref } from "vue";
import ProblemItem from "../components/ProblemItem.vue";
import { API } from "../api";
import { Problem } from "../types";

const problems = ref<Problem[]>([]);

onMounted(async () => {
  const api = new API();

  problems.value = await api.getProblems();
});
</script>

<template>
  <div>
    <ProblemItem
      v-for="problem in problems"
      :problem="problem"
      :key="problem.id"
    />
  </div>
</template>
