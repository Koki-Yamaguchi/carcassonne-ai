<script setup lang="ts">
import { Tile, Color } from "../tiles";
import { lyingMeepleSrc, standingMeepleSrc } from "../meeples";
import { ref, watch } from "vue";

type State = "meepling" | "normal" | "placing" | "shadow" | "empty";

const props = defineProps<{
  tile: Tile | null;
  state: State;
  meepleablePositions?: number[];
  onClick?: () => void;
}>();
defineEmits<{
  (e: "placeMeeple", pos: number): void;
}>();

const tileDeg = ref<number>(props.tile ? props.tile.direction * 90 : 0);

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
watch(
  () => props.tile?.direction,
  (newDirection, oldDirection) => {
    if (newDirection === undefined || oldDirection === undefined) {
      return;
    }

    if (Math.abs(newDirection - oldDirection) === 2) {
      tileDeg.value = tileDeg.value + 180;
    } else {
      if (newDirection === (oldDirection + 1) % 4) {
        tileDeg.value = tileDeg.value + 90;
      } else {
        tileDeg.value = tileDeg.value - 90;
      }
    }
  }
);
</script>

<template>
  <div
    v-if="state === 'meepling' && tile !== null && meepleablePositions"
    class="box"
    :style="boxStyle"
  >
    <img
      :style="{
        transform: `rotate(${tile.direction * 90}deg)`,
        ...frameStyle(tile.frame),
      }"
      :src="tile.src"
    />
    <div class="meeple-spots">
      <div
        v-for="pos in tile.meepleablePositions(meepleablePositions)"
        :key="pos.idx"
        class="empty-spot"
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
  </div>
  <div
    v-else-if="state === 'normal' && tile !== null"
    class="box"
    :style="boxStyle"
  >
    <img
      :style="{
        transform: `rotate(${tile.direction * 90}deg)`,
        ...frameStyle(tile.frame),
      }"
      :src="tile.src"
    />
    <div
      v-for="pos in tile.meepleablePositions([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])"
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
  <div
    v-else-if="state === 'placing' && tile !== null"
    class="placing"
    :style="boxStyle"
  >
    <img
      :style="{
        transform: `rotate(${tileDeg}deg)`,
        transition: '0.5s',
        ...frameStyle(tile.frame),
      }"
      :src="tile.src"
      @click="onClick"
    />
  </div>
  <div
    v-else-if="state === 'shadow'"
    class="shadow"
    :style="boxStyle"
    @click="onClick"
  ></div>
  <div v-else-if="state === 'empty'" :style="boxStyle"></div>
</template>

<style scoped>
.box {
  position: relative;
}
img {
  width: 100%;
}
.shadow {
  border-color: rgba(0, 0, 0, 0.3);
  background-color: rgba(0, 0, 0, 0.3);
}
.placing > img {
  opacity: 0.5;
}
.empty-spot {
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
