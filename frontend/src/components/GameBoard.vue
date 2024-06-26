<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { Tile } from "../tiles";
import Panzoom from "@panzoom/panzoom";
import TileSquare from "./TileSquare.vue";
import { TilePosition } from "../types";
import WoodImg from "../assets/img/background-wood.png";

defineProps<{
  placeablePositions: TilePosition[];
  tiles: (Tile | null)[][];
  placingTile: Tile | null;
  placingPosition: TilePosition | null;
  meepleablePositions: number[];
  isLarge: boolean;
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
  return {
    "background-image": "url(" + WoodImg + ")",
  };
});
</script>

<template>
  <div
    :class="
      isLarge
        ? 'rounded h-[calc(100vh-270px)]'
        : 'rounded h-[350px] md:h-[600px]'
    "
    :style="boardStyle"
  >
    <div class="relative" ref="elem">
      <div class="flex" v-for="(row, y) in tiles" :key="y">
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
          <TileSquare
            v-else-if="tile !== null"
            :state="'normal'"
            :tile="tile"
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
      <div
        v-if="placingPosition !== null && meepleablePositions.length === 0"
        :style="{
          position: 'absolute',
          top: `${60 * (placingPosition ? placingPosition.y : 0)}px`,
          left: `${60 * (placingPosition ? placingPosition.x : 0)}px`,
          transition: '0.5s',
        }"
      >
        <TileSquare
          :state="'placing'"
          :tile="placingTile"
          :onClick="() => $emit('turnTile')"
        />
      </div>
    </div>
  </div>
</template>
