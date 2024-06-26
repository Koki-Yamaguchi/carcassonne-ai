<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { API } from "../api";
import { Color, newTile, Tile, TileEdition, TileKind } from "../tiles";
import {
  Board,
  Game,
  TileMove,
  Player,
  Move,
  MeepleMove,
  TilePosition,
  CompleteEvent,
} from "../types";
import { translate } from "../locales/translate";
import GameBoard from "../components/GameBoard.vue";
import PlayerInfo from "../components/PlayerInfo.vue";
import ReplayIcon from "../components/ReplayIcon.vue";
import TrashIcon from "../components/TrashIcon.vue";
import { store } from "../store";
import slider from "vue3-slider";

const TILE_TOTAL_COUNT = 72;
const player = ref<Player | null>(null);
const game = ref<Game>();
const tiles = ref<(Tile | null)[][]>([]);
const player0Point = ref<number>(0);
const player1Point = ref<number>(0);
const player0Meeples = ref<Set<number>>(new Set([0, 1, 2, 3, 4, 5, 6]));
const player1Meeples = ref<Set<number>>(new Set([7, 8, 9, 10, 11, 12, 13]));
const player0ProfileImageURL = ref<string>("");
const player1ProfileImageURL = ref<string>("");
const tileCount = ref<number>(0);
const discardedTileKinds = ref<TileKind[]>([]);
const showDiscardedTiles = ref<boolean>(false);
const moves = ref<Move[]>([]);
const meepledPositions = ref<Map<number, TilePosition>>(new Map());
const player0LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const player1LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const updating = ref<boolean>(false);
const finished = ref<boolean>(false);

const boardMemo = ref<Map<number, Board>>(new Map());
const getBoard = async (
  gameID: number,
  player0MeepleColor: Color,
  player1MeepleColor: Color,
  tileEdition: TileEdition,
  moveID: number
): Promise<Board> => {
  if (boardMemo.value?.has(moveID)) {
    return boardMemo.value.get(moveID) as Board;
  }

  const api = new API();
  const board = await api.getBoard(
    gameID,
    player0MeepleColor,
    player1MeepleColor,
    tileEdition,
    moveID
  );

  boardMemo.value?.set(moveID, board);

  return board;
};

const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

const initGame = async () => {
  const api = new API();
  const route = useRoute();

  const gameID: number = parseInt(route.params.id as string, 10);
  game.value = await api.getGame(gameID);

  player.value = await api.getPlayerByUserID(store.userID);

  const player0 = await api.getPlayer(game.value.player0ID);
  const player1 = await api.getPlayer(game.value.player1ID);

  player0ProfileImageURL.value = player0.profileImageURL;
  player1ProfileImageURL.value = player1.profileImageURL;

  moves.value = await api.getMoves(game.value.id);
};

const resetMeeples = () => {
  player0Meeples.value = new Set([0, 1, 2, 3, 4, 5, 6]);
  player1Meeples.value = new Set([7, 8, 9, 10, 11, 12, 13]);
};

const retrieveMeeple = (meeples: Set<number>, meeple: number): TilePosition => {
  meeples.add(meeple);
  const pos = meepledPositions.value.get(meeple);
  if (!pos) {
    return { y: -1, x: -1 };
  }
  return pos;
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

const finishGame = async () => {
  if (!game.value) {
    return;
  }
  const api = new API();

  const finalEvents = await api.getFinalEevnts(game.value.id);

  processCompleteEvents(finalEvents.completeEvents);

  finished.value = true;
};

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

const next = async () => {
  if (!game.value || !player.value) {
    return;
  }
  updating.value = true;

  const lastOrd = tileCount.value * 2 + 1;

  const tileMove = moves.value[lastOrd + 1] as TileMove;

  tiles.value[tileMove.pos.y][tileMove.pos.x] = newTile(
    tileMove.rot,
    tileMove.tile,
    null,
    -1,
    -1,
    player.value.tileEdition
  );

  await sleep(500);

  const meepleMove = moves.value[lastOrd + 2] as MeepleMove;

  if (meepleMove.meepleID !== -1) {
    const meepleColor =
      tileMove.playerID === game.value.player0ID
        ? game.value.player0Color
        : game.value.player1Color;
    tiles.value[tileMove.pos.y][tileMove.pos.x]?.placeMeeple(
      meepleMove.pos,
      meepleColor,
      meepleMove.meepleID
    );
    useMeeple(
      tileMove.playerID === game.value.player0ID
        ? player0Meeples.value
        : player1Meeples.value,
      tileMove.pos,
      meepleMove.meepleID
    );
  }

  if (tileMove.playerID === game.value.player0ID) {
    if (player0LastTilePos.value.y !== -1) {
      tiles.value[player0LastTilePos.value.y][
        player0LastTilePos.value.x
      ]?.addFrame(null);
    }
    tiles.value[tileMove.pos.y][tileMove.pos.x]?.addFrame(
      game.value.player0Color
    );
    player0LastTilePos.value = { y: tileMove.pos.y, x: tileMove.pos.x };
  } else {
    if (player1LastTilePos.value.y !== -1) {
      tiles.value[player1LastTilePos.value.y][
        player1LastTilePos.value.x
      ]?.addFrame(null);
    }
    tiles.value[tileMove.pos.y][tileMove.pos.x]?.addFrame(
      game.value.player1Color
    );
    player1LastTilePos.value = { y: tileMove.pos.y, x: tileMove.pos.x };
  }

  const board = await getBoard(
    game.value.id,
    game.value.player0Color,
    game.value.player1Color,
    player.value.tileEdition,
    lastOrd + 2
  );

  tileCount.value++;

  await sleep(500);

  processCompleteEvents(board.completeEvents);

  if (tileCount.value >= TILE_TOTAL_COUNT - 1) {
    finishGame();
  }

  updating.value = false;
};

const refresh = async () => {
  if (!game.value || !player.value || updating.value) {
    return;
  }

  const ord = tileCount.value * 2 + 1;

  const board = await getBoard(
    game.value.id,
    game.value.player0Color,
    game.value.player1Color,
    player.value.tileEdition,
    ord
  );

  // this board.tiles is a reference to one of boardMemo, which should not be directly used as tiles.value
  tiles.value = board.tiles.map((row) => {
    return row.map((tile) =>
      tile
        ? newTile(
            tile.direction,
            tile.kind,
            tile.meepleColor,
            tile.meepledPosition,
            tile.meepleID,
            tile.tileEdition
          )
        : null
    );
  });

  player0Point.value = board.player0Point;
  player1Point.value = board.player1Point;

  resetMeeples();
  // manage meeples
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

  // frame tiles from last 1 or 2 tile moves
  let count = 0;
  for (let i = ord; i >= 2 && count < 2; i--) {
    // not tile move
    if (!("rot" in moves.value[i])) {
      continue;
    }
    count++;
    const tileMove = moves.value[i] as TileMove;
    const tilePosY = tileMove.pos.y;
    const tilePosX = tileMove.pos.x;
    if (tileMove.playerID === game.value?.player0ID) {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player0Color);
    } else {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player1Color);
    }
  }

  if (tileCount.value === TILE_TOTAL_COUNT - 1) {
    player0Point.value = game.value.player0Point;
    player1Point.value = game.value.player1Point;
    resetMeeples();
  }

  finished.value = tileCount.value === TILE_TOTAL_COUNT - 1;
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

const currentTileSrc = () => {
  if (tileCount.value === TILE_TOTAL_COUNT - 1 || !player.value) {
    return "";
  }

  const lastOrd = tileCount.value * 2 + 1;
  const tileMove = moves.value[lastOrd + 1] as TileMove;
  return newTile(0, tileMove.tile, null, -1, -1, player.value.tileEdition).src;
};

const currentPlayerName = () => {
  if (tileCount.value === TILE_TOTAL_COUNT - 1 || !game.value) {
    return "";
  }

  const lastOrd = tileCount.value * 2 + 1;
  const tileMove = moves.value[lastOrd + 1] as TileMove;

  if (tileMove.playerID === game.value.player0ID) {
    return game.value.player0Name;
  }
  return game.value.player1Name;
};

onMounted(async () => {
  await initGame();
  await refresh();
});
</script>

<template>
  <div class="p-4 bg-gray-100">
    <slider
      v-model="tileCount"
      @drag-end="refresh"
      :min="0"
      :max="TILE_TOTAL_COUNT - 1"
      color="#81c784"
      track-color="#FEFEFE"
      :handleScale="6.0"
      :alwaysShowHandle="true"
      :height="4"
    />
  </div>
  <div
    class="bg-gray-100 text-gray-900 text-sm px-4 py-3 shadow-md flex justify-between"
  >
    <div
      @click="
        () => {
          if (updating) {
            return;
          }
          if (tileCount === 0) {
            return;
          }
          tileCount--;
          refresh();
        }
      "
    >
      <ReplayIcon :kind="'prev'" />
    </div>
    <div
      @click="
        () => {
          if (updating) {
            return;
          }
          if (tileCount === TILE_TOTAL_COUNT - 1) {
            return;
          }
          next();
        }
      "
    >
      <ReplayIcon :kind="'next'" />
    </div>
    <div class="flex">
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
  <div v-if="!finished && currentPlayerName() !== ''">
    <div
      class="bg-gray-100 rounded text-gray-900 text-sm px-4 py-3 shadow-md flex items-center gap-2"
    >
      <span>{{ currentPlayerName() }}{{ translate("must_place") }}</span>
      <img
        class="min-h-[30px]"
        width="30"
        height="30"
        :src="currentTileSrc()"
      />
    </div>
  </div>
  <div v-if="finished">
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
      :profileImageURL="player0ProfileImageURL"
    />
    <PlayerInfo
      :name="game ? game.player1Name : ''"
      :point="player1Point"
      :meepleNumber="player1Meeples.size"
      :meepleColor="game ? game.player1Color : null"
      :tileSrc="null"
      :profileImageURL="player1ProfileImageURL"
    />
  </div>
  <div class="mt-3">
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
