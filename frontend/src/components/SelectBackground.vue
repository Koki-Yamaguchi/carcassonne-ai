<script setup lang="ts">
import { translate } from "../locales/translate";

defineEmits<{
  (e: "selectBackground", theme: string): void;
}>();
type Theme = "wood" | "white";

const themes: Theme[] = ["wood", "white"];
</script>

<template>
  <div class="relative">
    <div class="group">
      <button
        class="bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-2 py-1"
      >
        {{ translate("background") }}
      </button>
      <div class="hidden group-hover:block group-active:block">
        <div
          class="w-20 p-1 text-gray-500 bg-white absolute z-20 top-8 rounded-md"
        >
          <div class="">
            <div
              v-for="(theme, idx) in themes"
              :key="idx"
              @click="
                () => {
                  $emit('selectBackground', theme);
                }
              "
              class="hover:cursor-pointer"
            >
              {{ translate(theme) }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
