<script setup lang="ts">
import { Color } from "../tiles";
import { standingMeepleSrc } from "../meeples";
import ProfileImage from "./ProfileImage.vue";

defineProps<{
  name: string;
  point: number;
  meepleNumber: number;
  meepleColor: Color;
  tileSrc: any;
  profileImageURL?: string;
}>();
</script>
<template>
  <div
    class="w-40 max-w-xs ml-3 mt-3 px-3 py-3 border border-gray-200 shadow rounded-lg"
  >
    <div class="flex gap-2">
      <div class="flex justify-center items-center">
        <ProfileImage :src="profileImageURL ?? ''" />
      </div>
      <div>
        <div class="text-xs font-bold">{{ name }}</div>
        <div class="text-xs">{{ point }} pt</div>
      </div>
    </div>
    <div class="meeples flex mt-2">
      <div v-for="i in meepleNumber" :key="i">
        <img class="w-4" :src="standingMeepleSrc(meepleColor)" />
      </div>
    </div>
  </div>
</template>
