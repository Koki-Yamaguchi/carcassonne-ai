<script setup lang="ts">
import { Color } from "../tiles";
import { standingMeepleSrc } from "../meeples";

defineProps<{
  name: string;
  point: number;
  meepleNumber: number;
  meepleColor: Color;
}>();
</script>
<template>
  <div
    class="w-64 max-w-xs ml-3 mt-3 px-3 py-3 block border border-gray-200 shadow rounded-lg"
  >
    <div class="text-sm font-bold">{{ name }}</div>
    {{ point }} pt
    <div class="meeples flex">
      <div v-for="i in meepleNumber" :key="i">
        <img :src="standingMeepleSrc(meepleColor)" />
      </div>
    </div>
  </div>
</template>
