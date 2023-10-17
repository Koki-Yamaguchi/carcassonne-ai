<script setup lang="ts">
import { Vote } from "../types";
import ChevronIcon from "../components/ChevronIcon.vue";
import ProfileImage from "../components/ProfileImage.vue";

defineProps<{
  vote: Vote;
  isOpen: boolean;
}>();
</script>

<template>
  <div>
    <div
      class="bg-white border rounded-md p-2 hover:cursor-pointer"
      :class="isOpen ? 'border-4 border-gray-400' : ''"
    >
      <div class="flex">
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
        {{ vote.note }}
      </div>
    </div>
  </div>
</template>
