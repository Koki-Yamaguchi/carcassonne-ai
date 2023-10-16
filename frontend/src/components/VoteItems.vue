<script setup lang="ts">
import { Vote } from "../types";
import VoteItem from "../components/VoteItem.vue";

defineProps<{
  votes: Vote[];
  currentVoteID: number;
}>();
const emit = defineEmits<{
  (e: "clickVote", voteID: number): void;
}>();

const clickVote = (voteID: number) => {
  emit("clickVote", voteID);
};
</script>

<template>
  <div v-for="vote in votes" :key="vote.id" @click="clickVote(vote.id)">
    <VoteItem :vote="vote" :isOpen="vote.id === currentVoteID" />
  </div>
</template>
