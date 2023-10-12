<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Problem } from "../types";
import { API } from "../api";
import { useRoute } from "vue-router";

const problem = ref<Problem | null>(null);
onMounted(async () => {
  const api = new API();

  const route = useRoute();
  const id: number = parseInt(route.params.id as string, 10);

  problem.value = await api.getProblem(id);
});
</script>

<template>
  <div v-if="problem">{{ problem.id }}</div>
</template>
