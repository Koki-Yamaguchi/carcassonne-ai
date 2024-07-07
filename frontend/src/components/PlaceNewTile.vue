<script setup lang="ts">
import { allTiles } from "../tiles";
import type { TileEdition, TileKind } from "../tiles";
import PlusIcon from "./PlusIcon.vue";

defineEmits<{
  (e: "placingTile", tile: TileKind): void;
}>();
defineProps<{
  disabled: boolean;
  tileEdition: TileEdition;
}>();
</script>

<template>
  <div class="relative">
    <div v-if="disabled" class="opacity-50">
      <PlusIcon />
    </div>
    <div v-else class="group hover:cursor-pointer">
      <PlusIcon />
      <div class="hidden group-hover:block group-active:block">
        <div class="w-48 flex flex-wrap absolute top-8 z-20">
          <img
            v-for="tile in allTiles(tileEdition)"
            class="w-16 h-16 cursor-pointer rounded-sm"
            :key="tile.kind"
            @click="
              () => {
                $emit('placingTile', tile.kind);
              }
            "
            :src="tile.src"
          />
        </div>
      </div>
    </div>
  </div>
</template>
