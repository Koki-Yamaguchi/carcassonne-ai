<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { API } from "./../api";
import { useRoute } from "vue-router";
import {
  CompleteEvent,
  Game,
  TilePosition,
  TileMove,
  MeepleMove,
  Player,
} from "../types";
import { Color, Tile } from "../tiles";
import GameBoard from "../components/GameBoard.vue";
import { newTile, idToTileKind, boardSize, getInitialBoard } from "../tiles";
import PlayerInfo from "../components/PlayerInfo.vue";
import SpinnerIcon from "../components/SpinnerIcon.vue";
import WoodImg from "../assets/img/background-wood.png";
import { store } from "../store";

const TILE_TOTAL_COUNT = 72;
const game = ref<Game | null>(null);
const tiles = ref<(Tile | null)[][]>(getInitialBoard());
const currentTileID = ref<number | null>(null);
const nextTileID = ref<number | null>(null);
const placingTile = ref<Tile | null>(null);
const placeablePositions = ref<TilePosition[]>([]);
const placeableDirections = ref<number[]>([]);
const placingPosition = ref<TilePosition>({ y: -1, x: -1 });
const meepleablePositions = ref<number[]>([]);
const player = ref<Player | null>(null);
const player0Meeples = ref<Set<number>>(new Set([0, 1, 2, 3, 4, 5, 6]));
const player1Meeples = ref<Set<number>>(new Set([7, 8, 9, 10, 11, 12, 13]));
const player0Point = ref<number>(0);
const player1Point = ref<number>(0);
const player0LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const player1LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const meepledPositions = ref<Map<number, TilePosition>>(new Map());
const finished = ref<boolean>(false);
const tileCount = ref<number>(1);
const replayMove = ref<number>(TILE_TOTAL_COUNT * 2 - 1);
const meepleColor = ref<Color>("yellow");
const AIMeepleColor = ref<Color>("red");
const AIThinking = ref<boolean>(false);
const player0Name = ref<string>("");
const player1Name = ref<string>("");
const isMyGame = ref<boolean>(false);

const useMeeple = (
  meeples: Set<number>,
  pos: TilePosition,
  meepleID?: number
): number => {
  let mid = meepleID ? meepleID : -1;
  if (mid === -1) {
    for (let meeple of meeples.keys()) {
      mid = meeple;
      break;
    }
  }
  meeples.delete(mid);
  meepledPositions.value.set(mid, pos);

  return mid;
};
const retrieveMeeple = (meeples: Set<number>, meeple: number): TilePosition => {
  meeples.add(meeple);
  const pos = meepledPositions.value.get(meeple);
  if (!pos) {
    return { y: -1, x: -1 };
  }
  return pos;
};

const resetMeeples = () => {
  player0Meeples.value = new Set([0, 1, 2, 3, 4, 5, 6]);
  player1Meeples.value = new Set([7, 8, 9, 10, 11, 12, 13]);
};

const getPlaceablePositions = (placingTile: Tile): TilePosition[] => {
  const pos = [];
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
          placingTile.rotate();
          if (
            (tiles.value[y - 1][x] !== null &&
              tiles.value[y - 1][x]?.bottom() !== placingTile.top()) ||
            (tiles.value[y + 1][x] !== null &&
              tiles.value[y + 1][x]?.top() !== placingTile.bottom()) ||
            (tiles.value[y][x - 1] !== null &&
              tiles.value[y][x - 1]?.right() !== placingTile.left()) ||
            (tiles.value[y][x + 1] !== null &&
              tiles.value[y][x + 1]?.left() !== placingTile.right())
          ) {
            continue;
          }
          pos.push({ y, x });
        }
      }
    }
  }
  return pos;
};

const handleTilePositionSelected = (pos: TilePosition) => {
  placingPosition.value = pos;
  placeableDirections.value = [];
  placingTile.value?.resetDirection();
  const y = pos.y;
  const x = pos.x;
  const dirs = [];
  for (let dir = 0; dir < 4; dir++) {
    if (
      (tiles.value[y - 1][x] === null ||
        tiles.value[y - 1][x]?.bottom() === placingTile.value?.top()) &&
      (tiles.value[y + 1][x] === null ||
        tiles.value[y + 1][x]?.top() === placingTile.value?.bottom()) &&
      (tiles.value[y][x - 1] === null ||
        tiles.value[y][x - 1]?.right() === placingTile.value?.left()) &&
      (tiles.value[y][x + 1] === null ||
        tiles.value[y][x + 1]?.left() === placingTile.value?.right())
    ) {
      dirs.push(dir);
    }
    placingTile.value?.rotate();
  }
  placeableDirections.value = dirs;

  // initial valid direction
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

const confirm = async () => {
  if (
    game.value === null ||
    nextTileID.value === null ||
    currentTileID.value === null ||
    placingTile.value === null
  ) {
    return;
  }

  tiles.value[placingPosition.value.y][placingPosition.value.x] =
    placingTile.value;

  const api = new API();
  const res = await api.createTileMove(
    game.value.id,
    game.value.player0ID,
    currentTileID.value,
    placingTile.value.direction,
    placingPosition.value.y - Math.floor(boardSize / 2),
    placingPosition.value.x - Math.floor(boardSize / 2)
  );

  meepleablePositions.value = res.meepleablePositions;

  placingTile.value = null;

  if (
    meepleablePositions.value.length === 0 ||
    player0Meeples.value.size === 0
  ) {
    skip();
  }
};

const skip = () => {
  handlePlaceMeeple(-1);
};

const processCompleteEvents = (completeEvents: CompleteEvent[]) => {
  const pos = [];
  for (const e of completeEvents) {
    let player0Count = 0;
    let player1Count = 0;
    for (const meepleID of e.meepleIDs) {
      if (meepleID < 7) {
        player0Count++;
        pos.push(retrieveMeeple(player0Meeples.value, meepleID));
      } else {
        player1Count++;
        pos.push(retrieveMeeple(player1Meeples.value, meepleID));
      }
    }
    if (player0Count >= player1Count) {
      player0Point.value += e.point;
    }
    if (player1Count >= player0Count) {
      player1Point.value += e.point;
    }
  }
  for (const p of pos) {
    tiles.value[p.y][p.x]?.removeMeeple();
  }
};

const finishGame = async (gameID: number) => {
  const api = new API();
  const finalEvents = await api.getFinalEevnts(gameID);
  processCompleteEvents(finalEvents.completeEvents);
  finished.value = true;
  placeablePositions.value = [];
};

const handlePlaceMeeple = async (pos: number) => {
  if (!game.value) {
    return;
  }

  let meepleID = -1;
  if (pos !== -1) {
    meepleID = useMeeple(player0Meeples.value, {
      y: placingPosition.value.y,
      x: placingPosition.value.x,
    });

    tiles.value[placingPosition.value.y][placingPosition.value.x]?.placeMeeple(
      pos,
      meepleColor.value,
      meepleID
    );
  }

  if (player0LastTilePos.value.y !== -1) {
    tiles.value[player0LastTilePos.value.y][
      player0LastTilePos.value.x
    ]?.addFrame(null);
  }
  tiles.value[placingPosition.value.y][placingPosition.value.x]?.addFrame(
    meepleColor.value
  );
  player0LastTilePos.value = placingPosition.value;

  const tilePosY = placingPosition.value.y - Math.floor(boardSize / 2);
  const tilePosX = placingPosition.value.x - Math.floor(boardSize / 2);

  const api = new API();
  const res = await api.createMeepleMove(
    game.value.id,
    game.value.player0ID,
    meepleID,
    pos,
    tilePosY,
    tilePosX
  );

  processCompleteEvents(res.completeEvents);
  tileCount.value++;

  placingPosition.value = { y: -1, x: -1 };
  placeablePositions.value = [];
  meepleablePositions.value = [];

  if (res.currentTileID === -1) {
    await finishGame(game.value.id);
    return;
  }
  currentTileID.value = res.currentTileID;
  nextTileID.value = res.nextTileID;

  placingTile.value = newTile(0, idToTileKind(res.nextTileID), null, -1, -1);
  placeablePositions.value = getPlaceablePositions(placingTile.value);

  await processAIMove();

  placeablePositions.value = getPlaceablePositions(placingTile.value);

  if (currentTileID.value === -1) {
    await finishGame(game.value.id);
    return;
  }
};

const processAIMove = async () => {
  if (!game.value) {
    return -1;
  }

  AIThinking.value = true;
  const api = new API();
  const res = await api.waitAIMove(game.value.id);

  const moves = await api.getMoves(game.value.id);

  const tileMove = moves[moves.length - 2] as TileMove;
  const meepleMove = moves[moves.length - 1] as MeepleMove;
  const tile = newTile(tileMove.rot, tileMove.tile, null, -1, -1);
  const tilePosY = tileMove.pos.y + Math.floor(boardSize / 2);
  const tilePosX = tileMove.pos.x + Math.floor(boardSize / 2);

  if (meepleMove.meepleID !== -1) {
    tile.placeMeeple(meepleMove.pos, AIMeepleColor.value, meepleMove.meepleID);
    useMeeple(
      player1Meeples.value,
      { y: tilePosY, x: tilePosX },
      meepleMove.meepleID
    );
  }

  tiles.value[tilePosY][tilePosX] = tile;

  if (player1LastTilePos.value.y !== -1) {
    tiles.value[player1LastTilePos.value.y][
      player1LastTilePos.value.x
    ]?.addFrame(null);
  }
  tiles.value[tilePosY][tilePosX]?.addFrame(AIMeepleColor.value);
  player1LastTilePos.value = { y: tilePosY, x: tilePosX };

  processCompleteEvents(res.completeEvents);

  tileCount.value++;

  currentTileID.value = res.currentTileID;
  nextTileID.value = -1; // invisible yet

  AIThinking.value = false;
};

const winner = computed(() => {
  if (player0Point.value > player1Point.value) {
    return player0Name;
  } else if (player0Point.value < player1Point.value) {
    return player1Name;
  } else {
    return "tie";
  }
});

const updateSituation = async (gameID: number, moveID?: number) => {
  const api = new API();
  const board = await api.getBoard(
    gameID,
    meepleColor.value,
    AIMeepleColor.value,
    moveID
  );
  tiles.value = board.tiles;
  player0Point.value = board.player0Point;
  player1Point.value = board.player1Point;

  resetMeeples();
  for (let y = 0; y < tiles.value.length; y++) {
    for (let x = 0; x < tiles.value[y].length; x++) {
      if (tiles.value[y][x] !== null) {
        const meepleID = tiles.value[y][x]?.meepleID;
        if (meepleID !== -1) {
          if ((meepleID as number) < 7) {
            useMeeple(player0Meeples.value, { y, x }, meepleID);
          } else {
            useMeeple(player1Meeples.value, { y, x }, meepleID);
          }
        }
      }
    }
  }

  const moves = await api.getMoves(gameID, moveID);

  tileCount.value = moves.length / 2;
  if (moves.length === 2) {
    return;
  }

  // frame tiles from last 1 or 2 tile moves
  for (let i = moves.length - 4; i < moves.length; i += 2) {
    const tileMove = moves[i] as TileMove;
    const tilePosY = tileMove.pos.y + Math.floor(boardSize / 2);
    const tilePosX = tileMove.pos.x + Math.floor(boardSize / 2);
    if (tileMove.playerID === player.value?.id) {
      tiles.value[tilePosY][tilePosX]?.addFrame(meepleColor.value);
      player0LastTilePos.value = { y: tilePosY, x: tilePosX };
    } else {
      tiles.value[tilePosY][tilePosX]?.addFrame(AIMeepleColor.value);
      player1LastTilePos.value = { y: tilePosY, x: tilePosX };
    }
  }

  if (tileCount.value === TILE_TOTAL_COUNT) {
    finished.value = true;
    placeablePositions.value = [];
  }
};

const goPrev = () => {
  replayMove.value -= 2;
  updateSituation(game.value?.id as number, replayMove.value);
};
const goNext = () => {
  replayMove.value += 2;
  updateSituation(game.value?.id as number, replayMove.value);
};

onMounted(async () => {
  const api = new API();
  const route = useRoute();

  const gameID: number = parseInt(route.params.id as string, 10);
  game.value = await api.getGame(gameID);

  player.value = await api.getPlayer(store.userID);

  meepleColor.value = game.value.player0Color;
  AIMeepleColor.value = game.value.player1Color;
  player0Name.value = game.value.player0Name;
  player1Name.value = game.value.player1Name;

  isMyGame.value = player.value.id === game.value.player0ID;

  await updateSituation(gameID);

  if (finished.value) {
    return;
  }

  currentTileID.value = game.value.currentTileID;
  nextTileID.value = game.value.nextTileID;

  const placingTileKind = idToTileKind(
    game.value.currentPlayerID === 1 ? nextTileID.value : currentTileID.value
  );
  placingTile.value = newTile(0, placingTileKind, null, -1, -1);
  placeablePositions.value = getPlaceablePositions(placingTile.value);

  if (game.value.currentPlayerID === 1) {
    await processAIMove();
  }

  placeablePositions.value = getPlaceablePositions(placingTile.value);
});

const currentTile = () => {
  if (currentTileID.value !== null) {
    return newTile(0, idToTileKind(currentTileID.value), null, -1, -1);
  }
};
const placingTileSrc = () => {
  const tileID = AIThinking.value ? nextTileID.value : currentTileID.value;
  if (tileID !== null) {
    const t = newTile(0, idToTileKind(tileID), null, -1, -1);
    return t.src;
  }
};
const boardStyle = computed(() => {
  return {
    "background-image": "url(" + WoodImg + ")",
  };
});
</script>
<template>
  <div v-if="!finished">
    <div
      class="bg-orange-100 rounded text-orange-900 px-4 py-3 shadow-md flex justify-between"
    >
      <div class="flex">
        <p class="flex flex-col justify-center mr-3">
          {{ AIThinking ? "AI" : "You" }} must place
        </p>
        <div class="flex flex-col justify-center min-w-[30px] mr-3">
          <img
            class="min-h-[30px]"
            width="30"
            height="30"
            :src="currentTile() ? currentTile()!.src : null"
          />
        </div>
        <SpinnerIcon v-if="AIThinking" />
        <div class="flex flex-col justify-center">
          <button
            class="bg-orange-400 hover:bg-orange-300 text-white rounded px-4 py-2"
            v-if="isMyGame && placingPosition.y !== -1 && placingTile !== null"
            @click="confirm"
          >
            Confirm
          </button>
          <button
            class="bg-orange-400 hover:bg-orange-300 text-white rounded px-4 py-2"
            v-else-if="meepleablePositions.length !== 0"
            @click="skip"
          >
            Skip
          </button>
        </div>
      </div>
      <div class="flex flex-col justify-center">
        {{ Math.max(TILE_TOTAL_COUNT - tileCount - 2, 0) }} / 72
      </div>
    </div>
  </div>
  <div v-else>
    <div class="bg-orange-100 rounded text-orange-900 px-4 py-3 shadow-md">
      <div v-if="tileCount === TILE_TOTAL_COUNT">
        <p v-if="winner !== 'tie'" class="flex flex-col justify-center mr-3">
          {{ winner }} wins!
        </p>
        <p v-else>tie!</p>
      </div>
      <div class="flex gap-2">
        <button
          class="bg-orange-400 hover:bg-orange-300 text-white rounded px-4 py-2"
          @click="goPrev"
        >
          Previous
        </button>
        <button
          class="bg-orange-400 hover:bg-orange-300 text-white rounded px-4 py-2"
          @click="goNext"
        >
          Next
        </button>
      </div>
    </div>
  </div>
  <div class="infos flex flex-wrap">
    <PlayerInfo
      :name="player0Name"
      :point="player0Point"
      :meepleNumber="player0Meeples.size"
      :meepleColor="meepleColor"
      :tileSrc="placingTileSrc()"
    />
    <PlayerInfo
      :name="player1Name"
      :point="player1Point"
      :meepleNumber="player1Meeples.size"
      :meepleColor="AIMeepleColor"
      :tileSrc="null"
    />
  </div>
  <div class="board mt-3" :style="boardStyle">
    <GameBoard
      :tiles="tiles"
      :placeablePositions="placeablePositions"
      @tilePositionSelected="handleTilePositionSelected"
      :placingTile="placingTile"
      :placingPosition="placingPosition"
      @turnTile="handleTurnTile"
      :meepleablePositions="meepleablePositions"
      @placeMeeple="handlePlaceMeeple"
    />
  </div>
</template>
<style scoped>
.board {
  height: 1000px;
  border-radius: 0.5%;
}
</style>
