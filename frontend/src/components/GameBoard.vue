<script setup lang="ts">
import { ref, onMounted } from "vue";
import { Tile } from "../tiles";
import Panzoom from "@panzoom/panzoom";
import TileSquare from "./TileSquare.vue";
import { TilePosition } from "../types";

defineProps<{
  placeablePositions: TilePosition[];
  tiles: (Tile | null)[][];
  placingTile: Tile | null;
  placingPosition: TilePosition;
  meepleablePositions: number[];
}>();
const emit = defineEmits<{
  (e: "tilePositionSelected", pos: TilePosition): void;
  (e: "turnTile"): void;
  (e: "placeMeeple", pos: number): void;
}>();

const handlePlaceMeeple = (pos: number) => {
  emit("placeMeeple", pos);
};

const elem = ref<HTMLElement>();
onMounted(() => {
  if (elem.value) {
    const panzoom = Panzoom(elem.value, {
      maxScale: 20,
      startX: -60 * 18,
      startY: -60 * 18,
    });
    if (elem.value.parentElement) {
      elem.value.parentElement.addEventListener("wheel", panzoom.zoomWithWheel);
    }
  }
});
</script>

<template>
  <div ref="elem">
    <div class="row" v-for="(row, y) in tiles" :key="y">
      <div v-for="(tile, x) in row" :key="x">
        <!-- placing tile (translucent) -->
        <TileSquare
          v-if="
            placingTile && placingPosition.y === y && placingPosition.x === x
          "
          :onClick="() => $emit('turnTile')"
          :tile="placingTile"
          :placeable="false"
          :placing="true"
          :meepling="false"
          :meepleablePositions="[]"
        />
        <!-- meepling -->
        <TileSquare
          v-else-if="placingPosition.y === y && placingPosition.x === x"
          @placeMeeple="handlePlaceMeeple"
          :onClick="() => {}"
          :tile="tile"
          :placeable="false"
          :placing="false"
          :meepling="true"
          :meepleablePositions="meepleablePositions"
        />
        <!-- placeable position (shadow) -->
        <TileSquare
          v-else-if="
            placeablePositions.filter((pos) => {
              return pos.y === y && pos.x === x;
            }).length > 0
          "
          :onClick="() => $emit('tilePositionSelected', { y, x })"
          :tile="null"
          :placeable="true"
          :placing="false"
          :meepling="false"
          :meepleablePositions="[]"
        />
        <!-- normal tile or empty -->
        <TileSquare
          v-else
          :onClick="() => {}"
          :tile="tile"
          :placeable="false"
          :placing="false"
          :meepling="false"
          :meepleablePositions="[]"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.row {
  display: flex;
}
</style>
