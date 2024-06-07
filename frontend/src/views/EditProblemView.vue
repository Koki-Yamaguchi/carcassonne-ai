<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import {
  Problem,
  Player,
  Game,
  Board,
  TileMove,
  DiscardMove,
  TilePosition,
} from "../types";
import { API } from "../api";
import { useRoute, useRouter } from "vue-router";
import { store } from "../store";
import GameBoard from "../components/GameBoard.vue";
import PlayerInfo from "../components/PlayerInfo.vue";
import ChevronIcon from "../components/ChevronIcon.vue";
import {
  boardSize,
  idToTileKind,
  newTile,
  Tile,
  getRemainingTileKinds,
  TileKind,
} from "../tiles";
import { translate, translate_with_arg } from "../locales/translate";

const router = useRouter();

const problem = ref<Problem | null>(null);
const game = ref<Game | null>(null);
const player = ref<Player | null>(null);
const board = ref<Board | null>(null);
const tiles = ref<(Tile | null)[][]>([]);
const player0Point = ref<number>(0);
const player1Point = ref<number>(0);
const player0Meeples = ref<Set<number>>(new Set([0, 1, 2, 3, 4, 5, 6]));
const player1Meeples = ref<Set<number>>(new Set([7, 8, 9, 10, 11, 12, 13]));
const tileCount = ref<number>(1);

const placingTile = ref<Tile | null>(null);
const placeablePositions = ref<TilePosition[]>([]);
const placeableDirections = ref<number[]>([]);
const placingPosition = ref<TilePosition | null>(null);
const meepleablePositions = ref<number[]>([]);
const canConfirm = ref<boolean>(false);
const fixBoard = ref<boolean>(false);

const name = ref<string>("");
const startAt = ref<Date | null>(null);

const showRemainingTiles = ref<boolean>(false);
const remainingTilesSrc = ref<string[]>([]);
const showPointDiff = ref<boolean>(false);

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

  canConfirm.value = true;
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

const currentTile = () => {
  if (!game.value || !player.value) {
    return null;
  }
  if (game.value.currentTileID !== null) {
    return newTile(
      0,
      idToTileKind(game.value.currentTileID),
      null,
      -1,
      -1,
      player.value.tileEdition
    );
  }
};

const update = async () => {
  if (!problem.value) {
    return;
  }
  const api = new API();

  const strtAt = startAt.value + ":00";

  await api.updateProblem(problem.value.id, name.value, strtAt);

  router.push(`/draft-problems`);
};

const publish = async () => {
  if (!problem.value) {
    return;
  }
  const api = new API();

  await api.publishProblem(problem.value.id, name.value);

  router.push(`/draft-problems`);
};

const del = async () => {
  if (!problem.value) {
    return;
  }

  const api = new API();

  await api.deleteProblem(problem.value.id);

  if (isAdmin.value) {
    router.push(`/draft-problems`);
  } else {
    router.push(`/problems/propose`);
  }
};

onMounted(async () => {
  const api = new API();

  const route = useRoute();
  const id: number = parseInt(route.params.id as string, 10);

  player.value = await api.getPlayerByUserID(store.userID);
  if (!player.value) {
    return;
  }

  problem.value = await api.getProblem(id, player.value.id);
  game.value = await api.getGame(problem.value.gameID);
  board.value = await api.getBoard(
    game.value.id,
    game.value.player0Color,
    game.value.player1Color,
    player.value.tileEdition
  );
  tiles.value = board.value.tiles;
  player0Point.value = board.value.player0Point;
  player1Point.value = board.value.player1Point;
  for (let y = 0; y < tiles.value.length; y++) {
    for (let x = 0; x < tiles.value[y].length; x++) {
      if (tiles.value[y][x] !== null) {
        const meepleID = tiles.value[y][x]?.meepleID;
        if (meepleID !== -1) {
          if ((meepleID as number) < 7) {
            player0Meeples.value.delete(meepleID as number);
          } else {
            player1Meeples.value.delete(meepleID as number);
          }
        }
      }
    }
  }
  const moves = await api.getMoves(game.value.id);
  tileCount.value = moves.filter((m) => !("meepleID" in m)).length;
  let count = 0;
  for (let i = moves.length - 1; i >= 2 && count < 2; i--) {
    // not tile move
    if (!("rot" in moves[i])) {
      continue;
    }
    count++;
    const tileMove = moves[i] as TileMove;
    const tilePosY = tileMove.pos.y;
    const tilePosX = tileMove.pos.x;
    if (tileMove.playerID === game.value?.player0ID) {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player0Color);
    } else {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player1Color);
    }
  }

  // TODO support discarded tiles

  const placingTileID = game.value.currentTileID;
  const placingTileKind = idToTileKind(placingTileID);
  placingTile.value = newTile(
    0,
    placingTileKind,
    null,
    -1,
    -1,
    player.value.tileEdition
  );
  placeablePositions.value = getPlaceablePositions(placingTile.value);

  // remaining tiles
  const outTiles: TileKind[] = moves
    .filter((m) => !("meepleID" in m))
    .map((m) => (m as TileMove | DiscardMove).tile)
    .concat([placingTileKind]);
  const remainingTiles = getRemainingTileKinds(outTiles);
  remainingTilesSrc.value = remainingTiles.map(
    (t) =>
      newTile(
        0,
        t,
        null,
        -1,
        -1,
        player.value ? player.value.tileEdition : "second"
      ).src
  );

  name.value = problem.value.name;
});

const creatorName = computed(() => {
  if (!problem.value) {
    return "";
  }
  return problem.value.creatorName ?? "admin";
});

const testerName = computed(() => {
  if (!problem.value) {
    return "";
  }
  return (
    (problem.value.testerName ? `${problem.value.testerName}, ` : "") + "admin"
  );
});

const isAdmin = computed(() => {
  return player.value && player.value.id === 2;
});

const isCreator = computed(() => {
  return (
    player.value && problem.value && player.value.id === problem.value.creatorID
  );
});
</script>

<template>
  <div v-if="!isAdmin && !isCreator" class="p-4">
    {{ translate("not_authorized") }}
  </div>
  <div v-else>
    <div class="mt-4 mx-4 flex justify-between">
      <div class="flex">
        <div v-if="!isAdmin" class="flex flex-col justify-center items-center">
          {{ problem && problem.name !== "" ? problem.name : "untitled" }}
        </div>
        <div v-else>
          <input v-model="name" class="border" type="text" />
        </div>
      </div>
      <div class="text-xs ml-1 mt-1 flex">
        <div>
          {{ translate("created_by") }} <b>{{ creatorName }}</b>
        </div>
        <div v-if="problem && problem.isSolved" class="ml-2">
          {{ translate("tested_by") }} <b>{{ testerName }}</b>
        </div>
      </div>
    </div>
    <div class="infos flex flex-wrap">
      <PlayerInfo
        :name="game ? game.player0Name : ''"
        :point="player0Point"
        :meepleNumber="player0Meeples.size"
        :meepleColor="game ? game.player0Color : null"
        :tileSrc="null"
        :profileImageURL="player ? player.profileImageURL : ''"
      />
      <PlayerInfo
        :name="game ? game.player1Name : ''"
        :point="player1Point"
        :meepleNumber="player1Meeples.size"
        :meepleColor="game ? game.player1Color : null"
        :tileSrc="null"
        :profileImageURL="''"
      />
    </div>
    <div class="mt-3">
      <div :class="fixBoard ? 'fixed top-0 w-full' : ''">
        <GameBoard
          :tiles="tiles"
          :placeablePositions="placeablePositions"
          :placingTile="placingTile"
          :placingPosition="placingPosition"
          :meepleablePositions="meepleablePositions"
          @tilePositionSelected="handleTilePositionSelected"
          @turnTile="handleTurnTile"
          @placeMeeple="(_: number) => {}"
          :isLarge="false"
        />
      </div>
      <div v-if="fixBoard" class="h-[350]x] md:h-[600px]">
        <!-- keeps height for fixing a board -->
      </div>
    </div>
    <div class="bg-gray-100 rounded text-gray-900 text-sm px-4 py-3 shadow-md">
      <div
        v-if="problem && problem.isSolved"
        class="bg-green-200 rounded-md p-2 mb-2"
      >
        {{
          translate_with_arg(
            "solved_problem_description",
            problem.optimalMoveCount
          )
        }}
      </div>
      <div class="flex">
        <div class="flex flex-col justify-center mr-3">
          <p>{{ translate("tile_in_hand") }}</p>
        </div>
        <div class="flex flex-col justify-center min-w-[30px] mr-3">
          <img
            v-if="game?.currentTileID !== -1"
            class="min-h-[30px]"
            width="30"
            height="30"
            :src="currentTile() ? currentTile()!.src : null"
          />
        </div>
      </div>
      <div
        class="flex mt-2 hover:cursor-pointer"
        @click="showRemainingTiles = !showRemainingTiles"
      >
        <div class="flex flex-col justify-center mr-2">
          <ChevronIcon :direction="showRemainingTiles ? 'bottom' : 'right'" />
        </div>
        <div>{{ translate("remaining_tiles") }}</div>
      </div>
      <div v-if="showRemainingTiles" class="flex flex-wrap gap-1 mt-2">
        <img
          v-for="(src, idx) in remainingTilesSrc"
          width="30"
          height="30"
          :src="src"
          :key="idx"
        />
      </div>
      <div
        class="flex mt-2 hover:cursor-pointer"
        @click="showPointDiff = !showPointDiff"
      >
        <div class="flex flex-col justify-center mr-2">
          <ChevronIcon :direction="showPointDiff ? 'bottom' : 'right'" />
        </div>
        <div>{{ translate("point_diff") }}</div>
      </div>
      <div v-if="showPointDiff" class="flex flex-wrap gap-1 mt-2">
        {{
          translate_with_arg(
            "point_diff_description",
            problem ? problem.pointDiff : 0
          )
        }}
      </div>
      <div class="mt-2">{{ translate("proposal_note") }}</div>
      <div>
        {{ problem ? problem.note : "" }}
      </div>
      <input
        v-if="isAdmin && problem && !problem.isDraft"
        class="mt-4"
        type="datetime-local"
        v-model="startAt"
      />
    </div>
    <div v-if="isAdmin" class="flex justify-center mt-4">
      <button
        v-if="problem && !problem.isDraft"
        class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded mb-4"
        type="button"
        @click="update"
      >
        {{ translate("update") }}
      </button>
      <button
        class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded mb-4"
        type="button"
        @click="publish"
        v-else
      >
        {{ translate("publish") }}
      </button>
    </div>
    <div class="flex justify-center mt-4">
      <button
        v-if="problem && problem.isDraft"
        class="shadow bg-red-500 hover:bg-red-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded mb-4"
        type="button"
        @click="del"
      >
        {{ translate("del") }}
      </button>
    </div>
  </div>
</template>
