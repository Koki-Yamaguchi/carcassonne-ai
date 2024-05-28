<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { API } from "../api";
import { useRoute, useRouter } from "vue-router";
import {
  CompleteEvent,
  DiscardMove,
  Game,
  Move,
  Player,
  TileMove,
  TilePosition,
  MoveCreatedEvent,
} from "../types";
import { store } from "../store";
import { Tile, TileKind, boardSize, idToTileKind, newTile } from "../tiles";
import { translate } from "../locales/translate";
import GameBoard from "../components/GameBoard.vue";
import PlayerInfo from "../components/PlayerInfo.vue";
import TrashIcon from "../components/TrashIcon.vue";
import SpinnerIcon from "../components/SpinnerIcon.vue";
import UndoIcon from "../components/UndoIcon.vue";

const router = useRouter();

// common variables
const TILE_TOTAL_COUNT = 72;
const player = ref<Player | null>(null);
const game = ref<Game | null>(null);
const player0ProfileImageURL = ref<string>("");
const player1ProfileImageURL = ref<string>("");
const isMyGame = ref<boolean>(false);
const tiles = ref<(Tile | null)[][]>([]);
const player0Point = ref<number>(0);
const player1Point = ref<number>(0);
const player0Meeples = ref<Set<number>>(new Set([0, 1, 2, 3, 4, 5, 6]));
const player1Meeples = ref<Set<number>>(new Set([7, 8, 9, 10, 11, 12, 13]));
const meepledPositions = ref<Map<number, TilePosition>>(new Map());
const moves = ref<Move[]>();
const tileCount = ref<number>(1);
const player0LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const player1LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const discardedTileKinds = ref<TileKind[]>([]);
const finished = ref<boolean>(false);
const evtSource = ref<any>(null);

// variables that is only needed from player0's point of view
const placingTile = ref<Tile | null>(null);
const placeablePositions = ref<TilePosition[]>([]);
const meepleablePositions = ref<number[]>([]);
const mustDiscard = ref<boolean>(false);
const canConfirm = ref<boolean>(false);
const canPlaceMeeple = ref<boolean>(false);
const placingPosition = ref<TilePosition | null>(null);
const showDiscardedTiles = ref<boolean>(false);
const placeableDirections = ref<number[]>([]);

const initGame = async () => {
  const api = new API();
  const route = useRoute();

  const gameID: number = parseInt(route.params.id as string, 10);
  game.value = await api.getGame(gameID);

  const player0 = await api.getPlayer(game.value.player0ID);
  const player1 = await api.getPlayer(game.value.player1ID);

  player0ProfileImageURL.value = player0.profileImageURL;
  player1ProfileImageURL.value = player1.profileImageURL;

  player.value = await api.getPlayerByUserID(store.userID);

  isMyGame.value =
    player.value.id === game.value.player0ID ||
    player.value.id === game.value.player1ID;
};

const joinGame = async () => {
  if (!game.value) {
    return;
  }

  const api = new API();

  evtSource.value = await api.subscribe(
    "move_created_event",
    game.value.id,
    update
  );
};

onUnmounted(() => {
  evtSource.value.close();
});

const update = async (e: any) => {
  if (!game.value) {
    return;
  }
  const api = new API();

  const d = JSON.parse(e.data);
  const event: MoveCreatedEvent = {
    id: d.id,
    name: d.name,
    playerID: d.player_id,
    tile: d.tile,
    rot: d.rot,
    tilePos: {
      y: d.tile_pos[0] + Math.floor(boardSize / 2),
      x: d.tile_pos[1] + Math.floor(boardSize / 2),
    },
    meepleID: d.meeple_id,
    meeplePos: d.meeple_pos,
    completeEvents: d.complete_events.map((ce: any) => {
      return {
        meepleIDs: ce.meeple_ids,
        feature: ce.feature,
        point: ce.point,
      };
    }),
  };

  if (player.value?.id === event.playerID) {
    return;
  }

  if (event.rot === -1) {
    updateDiscardMove(event.tile, true);
    return;
  }

  updateTileMove(event.rot, event.tile, event.tilePos);

  if (isMyGame.value && placingTile.value) {
    placeablePositions.value = getPlaceablePositions(placingTile.value);

    if (
      placingPosition.value &&
      placingPosition.value.y === event.tilePos.y &&
      placingPosition.value.x === event.tilePos.x
    ) {
      placingPosition.value = null;
    }
  }

  await sleep(500);

  updateMeepleMove(
    event.meepleID,
    event.meeplePos,
    event.tilePos,
    event.playerID
  );

  await sleep(500);

  processCompleteEvents(event.completeEvents);

  if (isMyGame.value && placeablePositions.value.length === 0) {
    mustDiscard.value = true;
  }

  game.value = await api.getGame(game.value.id);

  if (game.value.currentTileID === -1) {
    finishGame();
  }
};

const updateDiscardMove = async (tileKind: TileKind, showWarning: boolean) => {
  if (!game.value) {
    return;
  }

  const api = new API();

  if (showWarning) {
    alert(`The opponent discarded a tile.`);
  }

  tileCount.value++;

  discardedTileKinds.value.push(tileKind);

  game.value = await api.getGame(game.value.id);
};

const updateTileMove = (
  rot: number,
  tileKind: TileKind,
  tilePos: TilePosition
) => {
  if (!player.value || !game.value) {
    return;
  }
  const tile = newTile(rot, tileKind, null, -1, -1, player.value.tileEdition);
  tiles.value[tilePos.y][tilePos.x] = tile;
};

const updateMeepleMove = (
  meepleID: number,
  meeplePos: number,
  tilePos: TilePosition,
  playerID: number
) => {
  if (!game.value) {
    return;
  }

  if (meepleID !== -1) {
    const meepleColor =
      playerID === game.value.player0ID
        ? game.value.player0Color
        : game.value.player1Color;
    tiles.value[tilePos.y][tilePos.x]?.placeMeeple(
      meeplePos,
      meepleColor,
      meepleID
    );
    useMeeple(
      playerID === game.value.player0ID
        ? player0Meeples.value
        : player1Meeples.value,
      tilePos,
      meepleID
    );
  }

  if (playerID === game.value.player0ID) {
    if (player0LastTilePos.value.y !== -1) {
      tiles.value[player0LastTilePos.value.y][
        player0LastTilePos.value.x
      ]?.addFrame(null);
    }
    tiles.value[tilePos.y][tilePos.x]?.addFrame(game.value.player0Color);
    player0LastTilePos.value = { y: tilePos.y, x: tilePos.x };
  } else {
    if (player1LastTilePos.value.y !== -1) {
      tiles.value[player1LastTilePos.value.y][
        player1LastTilePos.value.x
      ]?.addFrame(null);
    }
    tiles.value[tilePos.y][tilePos.x]?.addFrame(game.value.player1Color);
    player1LastTilePos.value = { y: tilePos.y, x: tilePos.x };
  }

  tileCount.value++;
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

const getNextMeepleID = (meeples: Set<number>) => {
  for (let meeple of meeples.keys()) {
    return meeple;
  }
  return -1;
};

const retrieveMeeple = (meeples: Set<number>, meeple: number): TilePosition => {
  meeples.add(meeple);
  const pos = meepledPositions.value.get(meeple);
  if (!pos) {
    return { y: -1, x: -1 };
  }
  return pos;
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

const initialUpdate = async () => {
  if (!game.value || !player.value) {
    return;
  }

  const api = new API();
  const board = await api.getBoard(
    game.value.id,
    game.value.player0Color,
    game.value.player1Color,
    player.value.tileEdition
  );

  tiles.value = board.tiles;
  player0Point.value = board.player0Point;
  player1Point.value = board.player1Point;

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

  moves.value = await api.getMoves(game.value.id);

  tileCount.value = moves.value.filter((m) => !("meepleID" in m)).length;

  // frame tiles from last 1 or 2 tile moves
  let count = 0;
  for (let i = moves.value.length - 1; i >= 2 && count < 2; i--) {
    // not tile move
    if (!("rot" in moves.value[i])) {
      continue;
    }
    count++;
    const tileMove = moves.value[i] as TileMove;
    if (tileMove.playerID === game.value?.player0ID) {
      tiles.value[tileMove.pos.y][tileMove.pos.x]?.addFrame(
        game.value.player0Color
      );
      player0LastTilePos.value = tileMove.pos;
    } else {
      tiles.value[tileMove.pos.y][tileMove.pos.x]?.addFrame(
        game.value.player1Color
      );
      player1LastTilePos.value = tileMove.pos;
    }
  }

  // list discarded tiles
  discardedTileKinds.value = moves.value
    .filter((mv) => !("rot" in mv) && "tile" in mv)
    .map((mv) => {
      const dm = mv as DiscardMove;
      return dm.tile;
    });

  if (tileCount.value === TILE_TOTAL_COUNT) {
    await finishGame();
    return;
  }

  if (isMyGame.value) {
    const placingTileID =
      game.value.currentPlayerID === player.value.id
        ? game.value.currentTileID
        : game.value.nextTileID;
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

    if (
      game.value.currentPlayerID === player.value.id &&
      placeablePositions.value.length === 0
    ) {
      mustDiscard.value = true;
    }
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

const handlePlaceMeeple = async (meeplePos: number) => {
  if (
    !game.value ||
    !player.value ||
    !placingPosition.value ||
    !placingTile.value
  ) {
    return;
  }

  if (isMyGame.value) {
    canPlaceMeeple.value = false;
    meepleablePositions.value = [];
  }

  const meepleID =
    meeplePos === -1
      ? -1
      : getNextMeepleID(
          player.value.id === game.value.player0ID
            ? player0Meeples.value
            : player1Meeples.value
        );

  const api = new API();

  const createMoveResult = await api.createMove(
    game.value.id,
    player.value.id,
    game.value.currentTileID,
    placingTile.value.direction,
    placingPosition.value.y,
    placingPosition.value.x,
    meepleID,
    meeplePos,
    game.value.player0ID === 1 || game.value.player1ID === 1
  );

  game.value = await api.getGame(game.value.id);

  updateMeepleMove(meepleID, meeplePos, placingPosition.value, player.value.id);

  await sleep(500);

  processCompleteEvents(createMoveResult.completeEvents);

  if (isMyGame.value) {
    placingPosition.value = null;
    if (createMoveResult.nextTileID !== -1) {
      const placingTileKind = idToTileKind(createMoveResult.nextTileID);
      placingTile.value = newTile(
        0,
        placingTileKind,
        null,
        -1,
        -1,
        player.value.tileEdition
      );
      placeablePositions.value = getPlaceablePositions(placingTile.value);
    } else {
      placingTile.value = null;
      placeablePositions.value = [];
    }
  }

  if (createMoveResult.currentTileID === -1) {
    finishGame();
  }
};

const skip = async () => {
  await handlePlaceMeeple(-1);
};

const undo = async () => {
  if (!placingPosition.value || !placingTile.value) {
    return;
  }

  meepleablePositions.value = [];
  tiles.value[placingPosition.value.y][placingPosition.value.x] = null;
  placingPosition.value = null;
  placeablePositions.value = getPlaceablePositions(placingTile.value);
  canPlaceMeeple.value = false;
};

const placingTileSrc = computed(() => {
  return placingTile.value?.src;
});

const confirm = async () => {
  if (
    !game.value ||
    !player.value ||
    !placingTile.value ||
    !placingPosition.value
  ) {
    return;
  }

  if (isMyGame.value) {
    canConfirm.value = false;
    placeablePositions.value = [];
  }

  const api = new API();
  const r = await api.tryCreateTileMove(
    game.value.id,
    player.value.id,
    game.value.currentTileID,
    placingTile.value.direction,
    placingPosition.value.y,
    placingPosition.value.x
  );

  updateTileMove(
    placingTile.value.direction,
    idToTileKind(game.value.currentTileID),
    placingPosition.value
  );

  meepleablePositions.value = r.meepleablePositions;

  const meepleable =
    meepleablePositions.value.length > 0 &&
    ((player.value.id === game.value.player0ID &&
      player0Meeples.value.size !== 0) ||
      (player.value.id === game.value.player1ID &&
        player1Meeples.value.size !== 0));
  if (meepleable) {
    canPlaceMeeple.value = true;
  } else {
    skip();
  }
};

const discard = async () => {
  if (!game.value || !player.value) {
    return;
  }
  const api = new API();

  await api.createDiscardMove(
    game.value.id,
    game.value.currentPlayerID,
    game.value.currentTileID
  );

  await updateDiscardMove(idToTileKind(game.value.currentTileID), false);

  if (game.value.currentTileID !== -1) {
    const placingTileKind = idToTileKind(game.value.currentTileID);
    placingTile.value = newTile(
      0,
      placingTileKind,
      null,
      -1,
      -1,
      player.value.tileEdition
    );
    placeablePositions.value = getPlaceablePositions(placingTile.value);
    if (placeablePositions.value.length !== 0) {
      mustDiscard.value = false;
    }
  }
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

const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

const finishGame = async () => {
  if (!game.value) {
    return;
  }
  const api = new API();

  const finalEvents = await api.getFinalEevnts(game.value.id);

  processCompleteEvents(finalEvents.completeEvents);

  game.value = await api.getGame(game.value.id);

  finished.value = true;

  await sleep(2000);

  evtSource.value.close();

  router.push(`/games/${game.value.id}/result`);
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
  if (dirs.length === 0) {
    placingPosition.value = null;
    return;
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

onMounted(async () => {
  await initGame();
  await joinGame();
  await initialUpdate();
});
</script>

<template>
  <div>
    <div v-if="!finished">
      <div
        class="bg-gray-100 rounded text-gray-900 text-sm px-4 py-3 shadow-md flex justify-between"
      >
        <div class="flex" v-if="game?.currentTileID !== -1">
          <div
            v-if="game?.currentPlayerID === 1"
            class="flex flex-col justify-center mr-3"
          >
            <p>
              {{ translate("ai_must_place") }}
            </p>
          </div>
          <div v-else class="flex flex-col justify-center mr-3">
            <p>
              {{
                game?.currentPlayerID === player?.id
                  ? translate("you")
                  : game?.currentPlayerID === game?.player0ID
                  ? game?.player0Name
                  : game?.player1Name
              }}<span v-if="mustDiscard">{{ translate("must_discard") }}</span>
              <span v-else>{{ translate("must_place") }}</span>
            </p>
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
          <SpinnerIcon v-if="game?.currentPlayerID === 1" />
          <div
            class="flex items-center gap-2"
            v-if="game?.currentPlayerID === player?.id"
          >
            <button
              class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
              v-if="isMyGame && canConfirm"
              @click.once="confirm"
            >
              {{ translate("confirm") }}
            </button>
            <button
              class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
              v-else-if="isMyGame && canPlaceMeeple"
              @click.once="skip"
            >
              {{ translate("skip") }}
            </button>
            <button
              class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
              v-else-if="mustDiscard"
              @click.once="discard()"
            >
              {{ translate("discard") }}
            </button>
            <div v-if="isMyGame && canPlaceMeeple" @click="undo">
              <UndoIcon />
            </div>
          </div>
        </div>
        <div v-else>{{ translate("calculating_final_points") }}</div>
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
    </div>
    <div v-else>
      <div class="bg-gray-100 rounded text-gray-900 px-4 py-3 shadow-md">
        <div v-if="!placingPosition && tileCount === TILE_TOTAL_COUNT">
          <p class="flex flex-col justify-center mr-3">
            {{ winner }} {{ translate("wins") }}
          </p>
        </div>
        <div class="flex gap-4">
          <button
            class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
            @click.once="router.push(`/games/${game ? game.id : ''}/replay`)"
          >
            {{ translate("view_replay") }}
          </button>
        </div>
      </div>
    </div>
    <div class="infos flex flex-wrap">
      <PlayerInfo
        :name="game ? game.player0Name : ''"
        :point="player0Point"
        :meepleNumber="player0Meeples.size"
        :meepleColor="game ? game.player0Color : null"
        :tileSrc="placingTileSrc"
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
        :placeablePositions="placeablePositions"
        :placingTile="placingTile"
        :placingPosition="placingPosition"
        :meepleablePositions="meepleablePositions"
        @tilePositionSelected="handleTilePositionSelected"
        @turnTile="handleTurnTile"
        @placeMeeple="handlePlaceMeeple"
        :isLarge="true"
      />
    </div>
    <div class="absolute bottom-36 left-8">
      <div class="relative">
        <img
          class="w-14 rounded-md border-2 shadow-xl opacity-80"
          src="../assets/img/deck.png"
        />
        <div
          class="absolute bottom-1/2 left-1/2 transform translate-y-1/2 -translate-x-1/2 text-md"
        >
          {{ Math.max(TILE_TOTAL_COUNT - tileCount - 2, 0) }}
        </div>
      </div>
    </div>
    <div class="absolute bottom-36 right-8">
      <img
        v-if="placingTileSrc"
        class="w-14 rounded-md border-2 shadow-xl"
        :src="placingTileSrc"
      />
    </div>
  </div>
</template>
