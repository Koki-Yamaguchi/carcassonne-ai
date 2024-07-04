<script setup lang="ts">
import type { Tile, Color } from "../tiles";
import { lyingMeepleSrc, standingMeepleSrc } from "../meeples";
import CrossIcon from "./CrossIcon.vue";

const tileSize = 60; // px
const spotRadius = 4; // px
const spotColor = "#ffffff";

defineProps<{
  tile: Tile | null;
  placeable: boolean;
  placing: boolean;
  focusing: boolean;
  addingFrame: boolean;
}>();
defineEmits<{
  (e: "placeMeeple", idx: number): void;
  (e: "removeMeeple"): void;
  (e: "removeTile"): void;
  (e: "defocus"): void;
  (e: "addFrame"): void;
}>();

const boxStyle = {
  height: `${tileSize}px`,
  width: `${tileSize}px`,
};
const tileStyle = (dir: number, frame: Color) => {
  return {
    transform: `rotate(${dir * 90}deg)`,
    outline: frame !== null ? `2px solid ${frame}` : "none",
    "outline-offset": frame !== null ? "-2px" : "none",
  };
};
</script>

<template>
  <div class="box placing" :style="boxStyle" v-if="tile && placing">
    <img
      :style="{ transform: `rotate(${tile.direction * 90}deg)` }"
      :src="tile.src"
    />
  </div>
  <div class="box focusing" :style="boxStyle" v-else-if="tile && focusing">
    <img
      :style="{ transform: `rotate(${tile.direction * 90}deg)` }"
      :src="tile.src"
    />
    <div class="remove-tile" @click="$emit('removeTile')">
      <CrossIcon size="tiny" color="black" />
    </div>
    <div
      class="meeple-spots"
      v-for="pos in tile.meepleablePositions([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
      ])"
      :key="pos.idx"
    >
      <img
        class="meeple"
        v-if="tile.meepledPosition === pos.idx"
        @click="$emit('removeMeeple')"
        :style="{
          position: 'absolute',
          left: `${tileSize / 2 + (pos.x * tileSize) / 2 - 10}px`,
          top: `${tileSize / 2 - (pos.y * tileSize) / 2 - 10}px`,
        }"
        :src="
          pos.isField
            ? lyingMeepleSrc(tile.meepleColor)
            : standingMeepleSrc(tile.meepleColor)
        "
      />
      <div
        class="empty"
        v-else
        @click="$emit('placeMeeple', pos.idx)"
        :style="{
          position: 'absolute',
          left: `${tileSize / 2 + (pos.x * tileSize) / 2 - spotRadius}px`,
          top: `${tileSize / 2 - (pos.y * tileSize) / 2 - spotRadius}px`,
          border: `${spotRadius}px solid ${spotColor}`,
          opacity: 0.7,
        }"
      ></div>
    </div>
  </div>
  <div class="box" :style="boxStyle" v-else-if="tile">
    <img
      v-if="addingFrame"
      :style="tileStyle(tile.direction, tile.frame)"
      :src="tile.src"
      @click="$emit('addFrame')"
    />
    <img
      v-else
      :style="tileStyle(tile.direction, tile.frame)"
      :src="tile.src"
    />
    <div
      class="meeple-spots"
      v-for="pos in tile.meepleablePositions([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
      ])"
      :key="pos.idx"
    >
      <img
        class="meeple"
        v-if="tile.meepledPosition === pos.idx"
        :style="{
          position: 'absolute',
          left: `${tileSize / 2 + (pos.x * tileSize) / 2 - 10}px`,
          top: `${tileSize / 2 - (pos.y * tileSize) / 2 - 10}px`,
        }"
        :src="
          pos.isField
            ? lyingMeepleSrc(tile.meepleColor)
            : standingMeepleSrc(tile.meepleColor)
        "
      />
    </div>
  </div>
  <div v-else-if="placeable" class="box placeable" :style="boxStyle"></div>
  <div v-else class="box" :style="boxStyle" @click="$emit('defocus')">
    <!-- just some space -->
  </div>
</template>

<style scoped>
div.box {
  position: relative;
}
img {
  width: 100%;
}
.placeable {
  border-color: rgba(0, 0, 0, 0.3);
  background-color: rgba(0, 0, 0, 0.3);
}
.placing img {
  opacity: 0.5;
}
.focusing > img {
  outline: 2px solid black;
  outline-offset: -2px;
}
.empty {
  border-radius: 50%;
  cursor: pointer;
}
.meeple {
  cursor: pointer;
  z-index: 10;
}
img.meeple {
  width: 18px;
}
.remove-tile {
  cursor: pointer;
  position: absolute;
  right: 2px;
  top: 2px;
}
</style>
