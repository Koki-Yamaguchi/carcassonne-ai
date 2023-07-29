<script setup lang="ts">
import { Color } from "../tiles";
import { standingMeepleSrc } from "../meeples";

defineProps<{
  name: string;
  point: number;
  meepleNumber: number;
  meepleColor: Color;
  tileSrc: any;
}>();
</script>
<template>
  <div
    class="w-44 max-w-xs ml-3 mt-3 px-3 py-3 flex border border-gray-200 shadow rounded-lg"
  >
    <div>
      <div class="text-sm font-bold">{{ name }}</div>
      {{ point }} pt
      <div class="meeples flex">
        <div v-for="i in meepleNumber" :key="i">
          <img class="w-4" :src="standingMeepleSrc(meepleColor)" />
        </div>
      </div>
    </div>
    <img v-if="tileSrc" class="ml-2 w-8 h-8" :src="tileSrc" />
  </div>
</template>
