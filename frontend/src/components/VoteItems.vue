<script setup lang="ts">
import { Vote } from "../types";
import VoteItem from "../components/VoteItem.vue";
import { isJapaneseSetting } from "../locales/translate";
import { ref } from "vue";

defineProps<{
  votes: Vote[];
  currentVoteID: number;
}>();
const emit = defineEmits<{
  (e: "clickVote", voteID: number): void;
}>();

const handleClickVote = (voteID: number) => {
  emit("clickVote", voteID);
};

const isJapanese = ref<boolean>(isJapaneseSetting());
</script>

<template>
  <div v-for="vote in votes" :key="vote.id">
    <VoteItem
      :isJapanese="isJapanese"
      :vote="vote"
      :isOpen="vote.id === currentVoteID"
      @clickVote="handleClickVote"
    />
  </div>
</template>
