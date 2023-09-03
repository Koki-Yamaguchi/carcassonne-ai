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
  placingPosition: TilePosition | null;
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
        <TileSquare
          v-if="
            tile !== null &&
            meepleablePositions.length > 0 &&
            placingPosition !== null &&
            placingPosition.y === y &&
            placingPosition.x === x
          "
          :state="'meepling'"
          :tile="tile"
          :meepleablePositions="meepleablePositions"
          @placeMeeple="handlePlaceMeeple"
        />
        <TileSquare v-else-if="tile !== null" :state="'normal'" :tile="tile" />
        <TileSquare
          v-else-if="
            placingPosition !== null &&
            placingPosition.y === y &&
            placingPosition.x === x
          "
          :state="'placing'"
          :tile="placingTile"
          :onClick="() => $emit('turnTile')"
        />
        <TileSquare
          v-else-if="
            placeablePositions.filter((pos) => {
              return pos.y === y && pos.x === x;
            }).length > 0
          "
          :tile="null"
          :state="'shadow'"
          :onClick="() => $emit('tilePositionSelected', { y, x })"
        />
        <TileSquare v-else :state="'empty'" :tile="null" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.row {
  display: flex;
}
</style>
