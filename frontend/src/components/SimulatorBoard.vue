<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import Panzoom from "@panzoom/panzoom";
import type { Tile, Color } from "../tiles";
import SimulatorTileSquare from "./SimulatorTileSquare.vue";
import { TilePosition } from "../types";
import WoodImg from "../assets/img/background-wood.png";

const props = defineProps<{
  placeablePositions: TilePosition[];
  tiles: (Tile | null)[][];
  placingTile: Tile | null;
  placingPosition: TilePosition | null;
  focusingPosition: TilePosition | null;
  selectedFrame: Color;
  addingFrame: boolean;
  backgroundTheme: string;
}>();
defineEmits<{
  (e: "selectingPosition", pos: TilePosition): void;
  (e: "turnTile"): void;
  (e: "editTile", pos: TilePosition): void;
  (e: "placeMeeple", meeplePosIdx: number, pos: TilePosition): void;
  (e: "removeMeeple", pos: TilePosition): void;
  (e: "removeTile", pos: TilePosition): void;
  (e: "defocus"): void;
  (e: "addFrame", pos: TilePosition): void;
}>();
const elem = ref<HTMLElement>();
onMounted(() => {
  if (elem.value) {
    const panzoom = Panzoom(elem.value, {
      maxScale: 4,
      minScale: 0.3,
      startX: -60 * 18,
      startY: -60 * 18,
    });
    if (elem.value.parentElement) {
      elem.value.parentElement.addEventListener("wheel", panzoom.zoomWithWheel);
    }
  }
});
const boardStyle = computed(() => {
  if (props.backgroundTheme === "white") {
    return {
      "background-color": props.backgroundTheme,
    };
  }
  return {
    "background-image": "url(" + WoodImg + ")",
  };
});
</script>

<template>
  <div class="rounded h-[calc(100vh-200px)]" :style="boardStyle">
    <div ref="elem" class="tile-board">
      <div class="flex" v-for="(row, y) in tiles" :key="y">
        <div v-for="(tile, x) in row" :key="x">
          <div v-if="tile">
            <SimulatorTileSquare
              v-if="
                focusingPosition &&
                focusingPosition.y === y &&
                focusingPosition.x === x
              "
              :tile="tiles[y][x]"
              :placeable="false"
              :placing="false"
              :focusing="true"
              :addingFrame="false"
              @placeMeeple="(idx: number) => $emit('placeMeeple', idx, { y, x })"
              @removeMeeple="() => $emit('removeMeeple', { y, x })"
              @removeTile="() => $emit('removeTile', { y, x })"
            />
            <SimulatorTileSquare
              v-else-if="addingFrame"
              :tile="tiles[y][x]"
              :placeable="false"
              :placing="false"
              :focusing="false"
              :addingFrame="true"
              @addFrame="() => $emit('addFrame', { y, x })"
            />
            <SimulatorTileSquare
              v-else
              @click="$emit('editTile', { y, x })"
              :tile="tiles[y][x]"
              :placeable="false"
              :placing="false"
              :focusing="false"
              :addingFrame="false"
            />
          </div>
          <div
            v-else-if="
              placingPosition &&
              placingPosition.y === y &&
              placingPosition.x === x
            "
          >
            <SimulatorTileSquare
              @click="$emit('turnTile')"
              :tile="placingTile"
              :placeable="false"
              :placing="true"
              :focusing="false"
              :addingFrame="false"
            />
          </div>
          <div
            v-else-if="
              placeablePositions.filter((pos) => {
                return pos.y === y && pos.x === x;
              }).length > 0
            "
          >
            <SimulatorTileSquare
              @click="$emit('selectingPosition', { y, x })"
              :tile="null"
              :placeable="true"
              :placing="false"
              :focusing="false"
              :addingFrame="false"
            />
          </div>
          <div v-else>
            <SimulatorTileSquare
              :tile="null"
              :placeable="false"
              :placing="false"
              :focusing="false"
              :addingFrame="false"
              @defocus="$emit('defocus')"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.row {
  display: flex;
}
</style>
