<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Color, Tile, TileEdition, TileKind } from "../tiles";
import { newTile, boardSize, getInitialBoard, resetBoard } from "../tiles";
import { TilePosition } from "../types";
import { API } from "../api";
import { store } from "../store";
import SimulatorBoard from "../components/SimulatorBoard.vue";
import PlaceNewTile from "../components/PlaceNewTile.vue";
import ChangeColor from "../components/ChangeColor.vue";
import SelectFrame from "../components/SelectFrame.vue";
import SelectBackground from "../components/SelectBackground.vue";
import { translate } from "../locales/translate";

const tiles = ref<(Tile | null)[][]>([]);
const placingTile = ref<Tile | null>(null);
const placeablePositions = ref<TilePosition[]>([]);
const placeableDirections = ref<number[]>([]);
const placingPosition = ref<TilePosition | null>(null);
const focusingPosition = ref<TilePosition | null>(null);
const currentColor = ref<Color>("red");
const addingFrame = ref<boolean>(false);
const selectedFrame = ref<Color>(null);
const backgroundTheme = ref<string>("white");
const tileEdition = ref<TileEdition>("second");

const handleTileSelected = (tileKind: TileKind) => {
  defocus();

  placingTile.value = newTile(0, tileKind, null, -1, -1, tileEdition.value);
  for (let y = 1; y < boardSize - 1; y++) {
    for (let x = 1; x < boardSize - 1; x++) {
      if (tiles.value[y][x] === null) {
        if (
          tiles.value[y - 1][x] === null &&
          tiles.value[y + 1][x] === null &&
          tiles.value[y][x - 1] === null &&
          tiles.value[y][x + 1] === null
        ) {
          continue;
        }
        for (let dir = 0; dir < 4; dir++) {
          placingTile.value.rotate();
          if (
            (tiles.value[y - 1][x] !== null &&
              tiles.value[y - 1][x]?.bottom() !== placingTile.value.top()) ||
            (tiles.value[y + 1][x] !== null &&
              tiles.value[y + 1][x]?.top() !== placingTile.value.bottom()) ||
            (tiles.value[y][x - 1] !== null &&
              tiles.value[y][x - 1]?.right() !== placingTile.value.left()) ||
            (tiles.value[y][x + 1] !== null &&
              tiles.value[y][x + 1]?.left() !== placingTile.value.right())
          ) {
            continue;
          }
          placeablePositions.value.push({ y, x });
        }
      }
    }
  }
  if (placeablePositions.value.length === 0) {
    placingTile.value = null;
    alert(translate("no_fitting_square"));
  }
};

const handlePositionSelected = (pos: TilePosition) => {
  placeableDirections.value = [];
  placingTile.value?.resetDirection();
  const y = pos.y;
  const x = pos.x;
  const dirs = [];
  for (let dir = 0; dir < 4; dir++) {
    let placeable = true;
    if (
      (tiles.value[y - 1][x] !== null &&
        tiles.value[y - 1][x]?.bottom() !== placingTile.value?.top()) ||
      (tiles.value[y + 1][x] !== null &&
        tiles.value[y + 1][x]?.top() !== placingTile.value?.bottom()) ||
      (tiles.value[y][x - 1] !== null &&
        tiles.value[y][x - 1]?.right() !== placingTile.value?.left()) ||
      (tiles.value[y][x + 1] !== null &&
        tiles.value[y][x + 1]?.left() !== placingTile.value?.right())
    ) {
      placeable = false;
    }
    if (placeable) {
      dirs.push(dir);
    }
    placingTile.value?.rotate();
  }
  placeableDirections.value = dirs;

  // initial valid direction
  placingPosition.value = pos;
  while (
    !placeableDirections.value.includes(
      placingTile.value ? placingTile.value.direction : -1
    )
  ) {
    placingTile.value?.rotate();
  }
};

const handleTurnTile = () => {
  placingTile.value?.rotate();
  while (
    !placeableDirections.value.includes(
      placingTile.value ? placingTile.value.direction : -1
    )
  ) {
    placingTile.value?.rotate();
  }
};

const confirm = () => {
  if (!placingPosition.value) {
    return;
  }

  tiles.value[placingPosition.value.y][placingPosition.value.x] =
    placingTile.value;
  placingTile.value = null;
  placeablePositions.value = [];
  placeableDirections.value = [];
  placingPosition.value = null;
  saveBoardInCache();
};

const cancel = () => {
  placingTile.value = null;
  placeablePositions.value = [];
  placeableDirections.value = [];
  placingPosition.value = null;

  addingFrame.value = false;
  selectedFrame.value = null;
};

const reset = () => {
  if (!window.confirm(translate("reset_confirm"))) {
    return;
  }

  resetBoard(tiles.value, tileEdition.value);

  placingTile.value = null;
  placeablePositions.value = [];
  placeableDirections.value = [];
  placingPosition.value = null;
  saveBoardInCache();
};

const handleEditTile = (pos: TilePosition) => {
  focusingPosition.value = pos;
};

const placeMeeple = (meeplePosIdx: number, pos: TilePosition) => {
  tiles.value[pos.y][pos.x]?.placeMeeple(meeplePosIdx, currentColor.value, -1);
  saveBoardInCache();
};

const removeMeeple = (pos: TilePosition) => {
  tiles.value[pos.y][pos.x]?.removeMeeple();
  saveBoardInCache();
};

const removeTile = (pos: TilePosition) => {
  tiles.value[pos.y][pos.x] = null;
  saveBoardInCache();
};

const handleChangeColor = (color: Color) => {
  currentColor.value = color;
};

const handleSelectFrame = (color: Color) => {
  addingFrame.value = true;
  selectedFrame.value = color;
};

const handleSelectBackground = (theme: string) => {
  backgroundTheme.value = theme;
  saveBackgroundThemeInCache();
};

const addFrame = (pos: TilePosition) => {
  tiles.value[pos.y][pos.x]?.addFrame(selectedFrame.value);
  selectedFrame.value = null;
  addingFrame.value = false;
  saveBoardInCache();
};

const defocus = () => {
  focusingPosition.value = null;
};

const saveBoardInCache = () => {
  localStorage.setItem("board", JSON.stringify(tiles.value));
};

const getBoardInCache = (tileEdition: TileEdition): (Tile | null)[][] => {
  const boardStr = localStorage.getItem("board");
  if (boardStr) {
    const boardWithoutMethods: (Tile | null)[][] = JSON.parse(boardStr);
    const board: (Tile | null)[][] = getInitialBoard(tileEdition);
    board[(boardSize - 1) / 2][(boardSize - 1) / 2] = null;
    const ysz = Math.min(boardWithoutMethods.length, board.length);
    const xsz = Math.min(boardWithoutMethods[0].length, board[0].length);
    for (let y = 0; y < ysz; y++) {
      for (let x = 0; x < xsz; x++) {
        const t = boardWithoutMethods[y][x];
        if (t) {
          board[y][x] = newTile(
            t.direction,
            t.kind,
            t.meepleColor,
            t.meepledPosition,
            t.meepleID,
            tileEdition,
            t.frame
          );
        }
      }
    }

    return board;
  }
  return getInitialBoard(tileEdition);
};

const saveBackgroundThemeInCache = () => {
  localStorage.setItem("backgroundTheme", backgroundTheme.value);
};

const getBackgroundThemeInCache = (): string => {
  const theme = localStorage.getItem("backgroundTheme");
  return theme ? theme : "wood";
};

onMounted(async () => {
  const api = new API();

  tileEdition.value = "second";
  if (store.userID) {
    const player = await api.getPlayerByUserID(store.userID);
    tileEdition.value = player.tileEdition;
  }

  tiles.value = getBoardInCache(tileEdition.value);
  backgroundTheme.value = getBackgroundThemeInCache();
});
</script>

<template>
  <div class="simulator">
    <div class="flex flex-wrap items-center gap-3 p-3">
      <PlaceNewTile
        @placingTile="handleTileSelected"
        :disabled="placingTile !== null || addingFrame"
        :tileEdition="tileEdition ?? 'second'"
      />
      <ChangeColor
        @changeColor="handleChangeColor"
        :currentColor="currentColor"
        :disabled="placingTile !== null || addingFrame"
      />
      <SelectFrame
        @selectFrame="handleSelectFrame"
        :disabled="
          placingTile !== null || addingFrame || focusingPosition !== null
        "
      />
      <button
        class="bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-2 py-1 disabled:bg-gray-300"
        @click="confirm"
        :disabled="
          placingTile === null ||
          addingFrame ||
          placeableDirections.length === 0
        "
      >
        {{ translate("confirm") }}
      </button>
      <button
        class="bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-2 py-1 disabled:bg-gray-300"
        @click="cancel"
        :disabled="placingTile === null && !addingFrame"
      >
        {{ translate("cancel") }}
      </button>
      <SelectBackground @selectBackground="handleSelectBackground" />
      <button @click="reset" :disabled="false" :style="{ color: '#DC143C' }">
        {{ translate("reset") }}
      </button>
    </div>
    <div class="board">
      <SimulatorBoard
        :placeablePositions="placeablePositions"
        :tiles="tiles"
        :placingTile="placingTile"
        :placingPosition="placingPosition"
        :isLarge="true"
        @selectingPosition="handlePositionSelected"
        :placeableDirections="placeableDirections"
        @turnTile="handleTurnTile"
        @editTile="handleEditTile"
        :focusingPosition="focusingPosition"
        @placeMeeple="placeMeeple"
        @removeMeeple="removeMeeple"
        @removeTile="removeTile"
        @defocus="defocus"
        :selectedFrame="selectedFrame"
        :addingFrame="addingFrame"
        @addFrame="addFrame"
        :backgroundTheme="backgroundTheme"
      />
    </div>
  </div>
</template>
