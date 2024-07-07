<script setup lang="ts">
import type { Color } from "../tiles";
import CrossIcon from "./CrossIcon.vue";
import { translate } from "../locales/translate";

defineEmits<{
  (e: "selectFrame", color: Color): void;
}>();
defineProps<{
  disabled: boolean;
}>();
const colors: Color[] = ["red", "yellow", "green", "black", "blue", null];
</script>

<template>
  <div class="relative">
    <div class="group">
      <button
        class="bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-2 py-1 disabled:bg-gray-300"
        :disabled="disabled"
      >
        {{ translate("frame") }}
      </button>
      <div v-if="!disabled" class="hidden group-hover:block group-active:block">
        <div class="w-44 p-1 bg-white absolute z-20 top-8 rounded-md">
          <div class="flex gap-2 items-center">
            <div
              v-for="(color, idx) in colors"
              :key="idx"
              @click="
                () => {
                  $emit('selectFrame', color);
                }
              "
              class="hover:cursor-pointer"
            >
              <div
                v-if="color"
                class="h-5 w-5"
                :style="{ border: `1.5px solid ${color}` }"
              ></div>
              <div v-if="!color"><CrossIcon size="small" color="gray" /></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
