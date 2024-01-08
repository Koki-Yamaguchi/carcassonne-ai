<script setup lang="ts">
import { Vote } from "../types";
import { useFormatDate } from "../composables/date";

defineProps<{
  vote: Vote;
  handleClickProblemName: (problemID: number) => void;
}>();
</script>

<template>
  <div class="text-sm text-gray-700 whitespace-nowrap">
    <span class="text-xs">{{ useFormatDate(vote.createdAt) }}</span>
    {{ vote.playerName }} voted on
    <span
      class="underline hover:cursor-pointer"
      @click="handleClickProblemName(vote.problemID)"
      >{{ vote.problemName }}</span
    >
  </div>
</template>
