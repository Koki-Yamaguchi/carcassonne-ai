<script setup lang="ts">
import { Vote } from "../types";
import { computed } from "vue";

const props = defineProps<{
  vote: Vote;
  handleClickProblemName: (problemID: number) => void;
}>();

const createdAt = computed(() => {
  let d = props.vote.createdAt;
  d.setTime(d.getTime() + 1000 * 60 * 60 * 9);
  return d
    .toLocaleDateString("ja-JP", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    })
    .split("/")
    .join("-");
});
</script>

<template>
  <div class="text-sm text-gray-700 whitespace-nowrap">
    <span class="text-xs">{{ createdAt }}</span>
    {{ vote.playerName }} voted on
    <span
      class="underline hover:cursor-pointer"
      @click="handleClickProblemName(vote.problemID)"
      >{{ vote.problemName }}</span
    >
  </div>
</template>
