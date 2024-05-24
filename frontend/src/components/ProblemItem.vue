<script setup lang="ts">
import { Problem } from "../types";
import PersonIcon from "../components/PersonIcon.vue";
import HeartIcon from "../components/HeartIcon.vue";
import { computed } from "vue";

const props = defineProps<{
  problem: Problem;
}>();

const formatNumber = computed(() => {
  if (!props.problem || !props.problem.num) {
    return "000";
  }
  const num = props.problem.num;
  if (num < 10) {
    return `00${num}`;
  }
  if (num < 100) {
    return `0${num}`;
  }
  if (num < 1000) {
    return `${num}`;
  }
  return "XXX";
});
</script>

<template>
  <a
    :href="`/problems/${problem.id}`"
    class="flex justify-between border rounded-md px-2 py-2 mb-2 bg-white hover:bg-gray-50 hover:cursor-pointer"
  >
    <div class="flex flex-col justify-center">
      <div class="text-gray-700 text-xs">
        {{ formatNumber }}.
        {{ problem.name }}
      </div>
    </div>
    <div class="flex w-28 items-center">
      <div class="flex w-12 text-xs text-gray-500 gap-1 items-center">
        <PersonIcon />
        {{ problem.voteCount }}
      </div>
      <div class="flex w-12 text-xs text-gray-500 gap-1 items-center">
        <HeartIcon :isLarge="false" :isRed="problem.favorited" />
        {{ problem.favoriteCount }}
      </div>
      <div class="flex w-4 flex-col justify-center">
        <div>
          <div v-if="problem.voted">
            <img src="../assets/img/check.png" />
          </div>
          <div v-else></div>
        </div>
      </div>
    </div>
  </a>
</template>
