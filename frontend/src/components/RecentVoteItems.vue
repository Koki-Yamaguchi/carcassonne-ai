<script setup lang="ts">
import { Vote } from "../types";
import RecentVoteItem from "../components/RecentVoteItem.vue";

defineProps<{
  votes: Vote[];
  handleClickProblemName: (problemID: number) => void;
}>();
</script>
<template>
  <div class="overflow-scroll border-2 rounded-md p-2">
    <div v-for="vote in votes" :key="vote.id" class="mb-1">
      <RecentVoteItem
        :vote="vote"
        :handleClickProblemName="handleClickProblemName"
      />
    </div>
  </div>
</template>
