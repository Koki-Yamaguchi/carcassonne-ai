<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { API } from "../api";
import { boardSize, newTile, Tile, TileKind } from "../tiles";
import { Board, DiscardMove, Game, TileMove, Player } from "../types";
import { translate } from "../locales/translate";
import GameBoard from "../components/GameBoard.vue";
import PlayerInfo from "../components/PlayerInfo.vue";
import ReplayIcon from "../components/ReplayIcon.vue";
import TrashIcon from "../components/TrashIcon.vue";
import { store } from "../store";

const TILE_TOTAL_COUNT = 72;
const player = ref<Player | null>(null);
const game = ref<Game>();
const maxMoveOrd = ref<number>(0);
const currentMoveOrd = ref<number>(0);
const board = ref<Board>();
const tiles = ref<(Tile | null)[][]>([]);
const player0Point = ref<number>(0);
const player1Point = ref<number>(0);
const player0Meeples = ref<Set<number>>(new Set([0, 1, 2, 3, 4, 5, 6]));
const player1Meeples = ref<Set<number>>(new Set([7, 8, 9, 10, 11, 12, 13]));
const tileCount = ref<number>(1);
const discardedTileKinds = ref<TileKind[]>([]);
const showDiscardedTiles = ref<boolean>(false);

const initGame = async () => {
  const api = new API();
  const route = useRoute();

  const gameID: number = parseInt(route.params.id as string, 10);
  game.value = await api.getGame(gameID);

  player.value = await api.getPlayerByUserID(store.userID);

  const moves = await api.getMoves(game.value.id);
  maxMoveOrd.value = moves[moves.length - 1].ord;
  currentMoveOrd.value = maxMoveOrd.value;
};

const resetMeeples = () => {
  player0Meeples.value = new Set([0, 1, 2, 3, 4, 5, 6]);
  player1Meeples.value = new Set([7, 8, 9, 10, 11, 12, 13]);
};

const useMeeple = (
  meeples: Set<number>,
  // pos: TilePosition,
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
  // meepledPositions.value.set(mid, pos);

  return mid;
};

const update = async () => {
  if (!game.value || !player.value) {
    return;
  }

  const api = new API();

  board.value = await api.getBoard(
    game.value.id,
    game.value.player0Color,
    game.value.player1Color,
    player.value.tileEdition,
    currentMoveOrd.value
  );

  tiles.value = board.value.tiles;
  player0Point.value = board.value.player0Point;
  player1Point.value = board.value.player1Point;

  resetMeeples();
  // manage meeples
  for (let y = 0; y < tiles.value.length; y++) {
    for (let x = 0; x < tiles.value[y].length; x++) {
      if (tiles.value[y][x] !== null) {
        const meepleID = tiles.value[y][x]?.meepleID;
        if (meepleID !== -1) {
          if ((meepleID as number) < 7) {
            useMeeple(player0Meeples.value, meepleID);
          } else {
            useMeeple(player1Meeples.value, meepleID);
          }
        }
      }
    }
  }

  const moves = await api.getMoves(game.value.id, currentMoveOrd.value);
  tileCount.value = moves.filter((m) => !("meepleID" in m)).length;

  // frame tiles from last 1 or 2 tile moves
  let count = 0;
  for (let i = moves.length - 1; i >= 2 && count < 2; i--) {
    // not tile move
    if (!("rot" in moves[i])) {
      continue;
    }
    count++;
    const tileMove = moves[i] as TileMove;
    const tilePosY = tileMove.pos.y + Math.floor(boardSize / 2);
    const tilePosX = tileMove.pos.x + Math.floor(boardSize / 2);
    if (tileMove.playerID === game.value?.player0ID) {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player0Color);
    } else {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player1Color);
    }
  }

  // list discarded tiles
  discardedTileKinds.value = moves
    .filter((mv) => !("rot" in mv) && "tile" in mv)
    .map((mv) => {
      const dm = mv as DiscardMove;
      return dm.tile;
    });

  if (currentMoveOrd.value === maxMoveOrd.value) {
    player0Point.value = game.value.player0Point;
    player1Point.value = game.value.player1Point;
    resetMeeples();
  }
};

const winner = computed(() => {
  if (!game.value) {
    return "";
  }
  const winnerPlayerID = game.value?.winnerPlayerID;
  if (winnerPlayerID === game.value?.player0ID) {
    return game.value.player0Name;
  } else {
    return game.value.player1Name;
  }
});

onMounted(async () => {
  await initGame();
  await update();
});
</script>

<template>
  <div
    class="bg-gray-100 text-gray-900 text-sm px-4 py-3 shadow-md flex justify-between"
  >
    <div
      @click="
        () => {
          currentMoveOrd = 1;
          update();
        }
      "
    >
      <ReplayIcon :kind="'beginning'" />
    </div>
    <div
      @click="
        () => {
          currentMoveOrd = Math.max(currentMoveOrd - 1, 1);
          update();
        }
      "
    >
      <ReplayIcon :kind="'prev'" />
    </div>
    <div
      @click="
        () => {
          currentMoveOrd = Math.min(currentMoveOrd + 1, maxMoveOrd);
          update();
        }
      "
    >
      <ReplayIcon :kind="'next'" />
    </div>

    <div
      @click="
        () => {
          currentMoveOrd = maxMoveOrd;
          update();
        }
      "
    >
      <ReplayIcon :kind="'end'" />
    </div>
    <div class="flex">
      <div class="flex flex-col justify-center">
        {{ Math.max(TILE_TOTAL_COUNT - tileCount - 2, 0) }}/{{
          TILE_TOTAL_COUNT
        }}
      </div>
      <div class="flex flex-col justify-center ml-2 relative">
        <TrashIcon @click="showDiscardedTiles = !showDiscardedTiles" />
        <div
          v-if="showDiscardedTiles"
          class="absolute top-10 right-0 w-36 bg-gray-100 p-4 rounded-2xl shadow-md"
        >
          <p>{{ translate("discarded") }}</p>
          <div v-if="discardedTileKinds.length > 0" class="mt-2 flex gap-2">
            <img
              v-for="(discardedTileKind, idx) in discardedTileKinds"
              :src="
                newTile(
                  0,
                  discardedTileKind,
                  null,
                  -1,
                  -1,
                  player ? player.tileEdition : 'second'
                ).src
              "
              class="min-h-[30px]"
              width="30"
              height="30"
              :key="idx"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
  <div v-if="currentMoveOrd === maxMoveOrd">
    <div class="bg-gray-100 rounded text-gray-900 px-4 py-3 shadow-md">
      <p class="flex flex-col justify-center mr-3">
        {{ winner }} {{ translate("wins") }}
      </p>
    </div>
  </div>
  <div class="infos flex flex-wrap">
    <PlayerInfo
      :name="game ? game.player0Name : ''"
      :point="player0Point"
      :meepleNumber="player0Meeples.size"
      :meepleColor="game ? game.player0Color : null"
      :tileSrc="null"
    />
    <PlayerInfo
      :name="game ? game.player1Name : ''"
      :point="player1Point"
      :meepleNumber="player1Meeples.size"
      :meepleColor="game ? game.player1Color : null"
      :tileSrc="null"
    />
  </div>
  <div class="board mt-3">
    <GameBoard
      :tiles="tiles"
      :placeablePositions="[]"
      :placingTile="null"
      :placingPosition="null"
      :meepleablePositions="[]"
      :isLarge="true"
    />
  </div>
</template>
<style scoped>
.board {
  height: 1000px;
  border-radius: 0.5%;
}
</style>
