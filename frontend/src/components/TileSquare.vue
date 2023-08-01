<script setup lang="ts">
import { Tile, Color } from "../tiles";
import { lyingMeepleSrc, standingMeepleSrc } from "../meeples";

defineProps<{
  tile: Tile | null;
  placeable: boolean;
  placing: boolean;
  meepling: boolean;
  meepleablePositions: number[];
  onClick: () => void;
}>();
defineEmits<{
  (e: "placeMeeple", pos: number): void;
}>();

const tileSize = 60;
const spotRadius = 4; // px
const spotColor = "#ffffff";
const boxStyle = {
  height: `${tileSize}px`,
  width: `${tileSize}px`,
};
const frameStyle = (frame: Color) => {
  return {
    outline: frame !== null ? `2px solid ${frame}` : "none",
    "outline-offset": frame !== null ? "-2px" : "none",
  };
};
</script>

<template>
  <div v-if="tile" :style="boxStyle" class="box">
    <img
      v-if="placing"
      class="placing"
      :style="{ transform: `rotate(${tile.direction * 90}deg)` }"
      :src="tile.src"
      @click="onClick"
    />
    <div v-else>
      <img
        :style="{
          transform: `rotate(${tile.direction * 90}deg)`,
          ...frameStyle(tile.frame),
        }"
        :src="tile.src"
      />
      <div v-if="meepling" class="meeple-spots">
        <div
          v-for="pos in tile.meepleablePositions(meepleablePositions)"
          :key="pos.idx"
          class="empty"
          @click.once="$emit('placeMeeple', pos.idx)"
          :style="{
            position: 'absolute',
            left: `${tileSize / 2 + (pos.x * tileSize) / 2 - spotRadius}px`,
            top: `${tileSize / 2 - (pos.y * tileSize) / 2 - spotRadius}px`,
            border: `${spotRadius}px solid ${spotColor}`,
            opacity: 0.7,
          }"
        ></div>
      </div>
      <div v-else>
        <div
          v-for="pos in tile.meepleablePositions([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
          ])"
          :key="pos.idx"
        >
          <img
            v-if="tile.meepledPosition === pos.idx"
            class="meeple"
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
    </div>
  </div>
  <div
    v-else-if="placeable"
    :style="boxStyle"
    class="box placeable"
    @click="onClick"
  ></div>
  <div v-else :style="boxStyle" class="box">
    <!-- empty -->
  </div>
</template>

<style scoped>
.box {
  position: relative;
}
img {
  width: 100%;
}
.placeable {
  border-color: rgba(0, 0, 0, 0.3);
  background-color: rgba(0, 0, 0, 0.3);
}
img.placing {
  opacity: 0.5;
}
.empty {
  border-radius: 50%;
  cursor: pointer;
}
.meeple {
  z-index: 10;
}
img.meeple {
  width: 18px;
}
</style>
