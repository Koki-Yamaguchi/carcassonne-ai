<script setup lang="ts">
import { Vote } from "../types";
import ChevronIcon from "../components/ChevronIcon.vue";
import ProfileImage from "../components/ProfileImage.vue";
import { onMounted, computed, ref } from "vue";

const props = defineProps<{
  vote: Vote;
  isOpen: boolean;
  isJapanese: boolean;
}>();
const emit = defineEmits<{
  (e: "clickVote", voteID: number): void;
}>();

const clickVote = () => {
  emit("clickVote", props.vote.id);
};

const translated = ref<boolean>(false);
const showOriginal = ref<boolean>(false);

const note = computed(() => {
  if (!showOriginal.value && translated.value) {
    return props.vote.translation;
  }
  return props.vote.note;
});

const toggleShowOriginal = () => {
  showOriginal.value = !showOriginal.value;
};

onMounted(() => {
  translated.value =
    props.vote.lang !== null &&
    ((props.isJapanese && props.vote.lang === "en") ||
      (!props.isJapanese && props.vote.lang === "ja"));
});
</script>

<template>
  <div>
    <div
      class="bg-white border rounded-md p-2"
      :class="isOpen ? 'border-4 border-gray-400' : ''"
    >
      <div @click="clickVote" class="flex hover:cursor-pointer">
        <div class="flex flex-col justify-center mr-2">
          <ChevronIcon :direction="isOpen ? 'bottom' : 'right'" />
        </div>
        <ProfileImage class="mr-2" :src="vote.playerProfileImageURL" />
        <div class="flex flex-col justify-center">
          {{ vote.playerName }}
        </div>
      </div>
      <div
        v-if="isOpen && vote.note !== ''"
        class="pl-2 pt-2 text-xs break-words"
      >
        <div>
          {{ note }}
        </div>
        <div
          v-if="translated"
          class="text-[10px] mt-2 ml-1 text-gray-600 underline hover:cursor-pointer"
          @click="toggleShowOriginal"
        >
          {{ showOriginal ? "See translation" : "See original" }}
        </div>
      </div>
    </div>
  </div>
</template>
