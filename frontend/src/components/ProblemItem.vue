<script setup lang="ts">
import { useRouter } from "vue-router";
import { Problem } from "../types";
import PersonIcon from "../components/PersonIcon.vue";

defineProps<{
  problem: Problem;
  voted: boolean;
  voteCount: number;
}>();

const router = useRouter();
</script>

<template>
  <div
    @click="router.push(`/problems/${problem.id}`)"
    class="flex justify-between border rounded-md px-2 py-2 mb-2 bg-white hover:bg-gray-50 hover:cursor-pointer"
  >
    <div class="text-gray-700">
      {{ problem.name }}
    </div>
    <div class="flex justify-between w-24">
      <div class="flex text-sm text-gray-500">
        <div class="flex flex-col justify-center">
          <PersonIcon />
        </div>
        <div class="flex flex-col justify-center">
          {{ voteCount }}
        </div>
      </div>
      <div class="w-5">
        <div v-if="voted">
          <img src="../assets/img/check.png" />
        </div>
        <div v-else></div>
      </div>
    </div>
  </div>
</template>
