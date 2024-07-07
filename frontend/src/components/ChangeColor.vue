<script setup lang="ts">
import { standingMeepleSrc } from "../meeples";
import type { Color } from "../tiles";

defineEmits<{
  (e: "changeColor", color: Color): void;
}>();
defineProps<{
  currentColor: Color;
  disabled: boolean;
}>();
const colors: Color[] = ["red", "yellow", "green", "black", "blue"];
</script>

<template>
  <div class="relative">
    <div v-if="disabled" class="opacity-50">
      <img :src="standingMeepleSrc(currentColor)" />
    </div>
    <div v-else class="group hover:cursor-pointer">
      <img :src="standingMeepleSrc(currentColor)" />
      <div class="hidden group-hover:block group-active:block">
        <div class="w-48 absolute top-6 z-20 flex bg-white rounded-md">
          <img
            v-for="(color, idx) in colors"
            :key="idx"
            @click="
              () => {
                $emit('changeColor', color);
              }
            "
            :src="standingMeepleSrc(color)"
            class="p-1"
          />
        </div>
      </div>
    </div>
  </div>
</template>
