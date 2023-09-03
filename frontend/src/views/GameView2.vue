<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useRoute } from "vue-router";
import { API } from "../api";
import GameBoard from "../components/GameBoard.vue";
import PlayerInfo from "../components/PlayerInfo.vue";
import { translate } from "../locales/translate";
import { store } from "../store";
import WoodImg from "../assets/img/background-wood.png";
import SpinnerIcon from "../components/SpinnerIcon.vue";
import {
  boardSize,
  getInitialBoard,
  idToTileKind,
  newTile,
  Tile,
  TileKind,
} from "../tiles";
import {
  Board,
  CompleteEvent,
  DiscardMove,
  Game,
  MeepleMove,
  Move,
  TileMove,
  TilePosition,
} from "../types";
import TrashIcon from "../components/TrashIcon.vue";

// common variables
const TILE_TOTAL_COUNT = 72;
const game = ref<Game>();
const board = ref<Board>();
const moves = ref<Move[]>();
const isMyGame = ref<boolean>(false);
const tiles = ref<(Tile | null)[][]>(getInitialBoard());
const meepledPositions = ref<Map<number, TilePosition>>(new Map());
const player0Meeples = ref<Set<number>>(new Set([0, 1, 2, 3, 4, 5, 6]));
const player1Meeples = ref<Set<number>>(new Set([7, 8, 9, 10, 11, 12, 13]));
const tileCount = ref<number>(1);
const player0LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const player1LastTilePos = ref<TilePosition>({ y: -1, x: -1 });
const player0Point = ref<number>(0);
const player1Point = ref<number>(0);
const discardedTileKinds = ref<TileKind[]>([]);
const finished = ref<boolean>(false);
const showDiscardedTiles = ref<boolean>(false);
const evtSource = ref<any>(null);

// variables that is only needed from player0's point of view
const placingTile = ref<Tile | null>(null);
const placeablePositions = ref<TilePosition[]>([]);
const placeableDirections = ref<number[]>([]);
const placingPosition = ref<TilePosition | null>(null);
const meepleablePositions = ref<number[]>([]);
const confirming = ref<boolean>(false);
const handlingPlaceMeeple = ref<boolean>(false);
const mustDiscard = ref<boolean>(false);

const initGame = async () => {
  const api = new API();
  const route = useRoute();

  const gameID: number = parseInt(route.params.id as string, 10);
  game.value = await api.getGame(gameID);

  const player = await api.getPlayer(store.userID);

  isMyGame.value = player.id === game.value.player0ID;
};

const joinGame = async () => {
  if (!game.value) {
    return;
  }

  const api = new API();
  const updateHandler = () => {
    update();
  };

  evtSource.value = await api.join(game.value.id, updateHandler);
};

onUnmounted(() => {
  evtSource.value.close();
});

const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
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
    !game.value ||
    !placingTile.value ||
    !placingPosition.value ||
    confirming.value
  ) {
    return;
  }
  confirming.value = true;

  const api = new API();
  await api.createTileMove(
    game.value.id,
    game.value.player0ID,
    game.value.currentTileID,
    placingTile.value.direction,
    placingPosition.value.y - Math.floor(boardSize / 2),
    placingPosition.value.x - Math.floor(boardSize / 2)
  );

  await api.sendEvent(game.value.id);

  confirming.value = false;
};

const handlePlaceMeeple = async (pos: number) => {
  if (!game.value || !placingPosition.value || handlingPlaceMeeple.value) {
    return;
  }
  handlingPlaceMeeple.value = true;

  const meepleID = pos === -1 ? -1 : getNextMeepleID(player0Meeples.value);

  const tilePosY = placingPosition.value.y - Math.floor(boardSize / 2);
  const tilePosX = placingPosition.value.x - Math.floor(boardSize / 2);

  const api = new API();
  await api.createMeepleMove(
    game.value.id,
    game.value.player0ID,
    meepleID,
    pos,
    tilePosY,
    tilePosX
  );

  api.sendEvent(game.value.id);

  handlingPlaceMeeple.value = false;
};

const discard = async () => {
  if (!game.value) {
    return;
  }
  const api = new API();

  await api.createDiscardMove(
    game.value.id,
    game.value.currentPlayerID,
    game.value.currentTileID
  );
  await api.sendEvent(game.value.id);
};

const update = async () => {
  if (!game.value) {
    return;
  }

  const api = new API();
  game.value = await api.getGame(game.value.id);
  moves.value = await api.getMoves(game.value.id);
  board.value = await api.getBoard(
    game.value.id,
    game.value.player0Color,
    game.value.player1Color
  );

  const lastMove = moves.value[moves.value.length - 1];

  const updateTileMove = async (tm: TileMove) => {
    if (!game.value || !board.value) {
      return;
    }

    const tile = newTile(tm.rot, tm.tile, null, -1, -1);
    const tilePosY = tm.pos.y + Math.floor(boardSize / 2);
    const tilePosX = tm.pos.x + Math.floor(boardSize / 2);
    tiles.value[tilePosY][tilePosX] = tile;

    if (tm.playerID === game.value.player0ID) {
      if (player0LastTilePos.value.y !== -1) {
        tiles.value[player0LastTilePos.value.y][
          player0LastTilePos.value.x
        ]?.addFrame(null);
      }
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player0Color);
      player0LastTilePos.value = { y: tilePosY, x: tilePosX };
    } else {
      if (player1LastTilePos.value.y !== -1) {
        tiles.value[player1LastTilePos.value.y][
          player1LastTilePos.value.x
        ]?.addFrame(null);
      }
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player1Color);
      player1LastTilePos.value = { y: tilePosY, x: tilePosX };
    }
  };
  const updateMeepleMove = async (mm: MeepleMove, tm: TileMove) => {
    if (!game.value || !board.value) {
      return;
    }

    if (mm.meepleID !== -1) {
      const tilePosY = tm.pos.y + Math.floor(boardSize / 2);
      const tilePosX = tm.pos.x + Math.floor(boardSize / 2);
      const meepleColor =
        tm.playerID === game.value.player0ID
          ? game.value.player0Color
          : game.value.player1Color;
      tiles.value[tilePosY][tilePosX]?.placeMeeple(
        mm.pos,
        meepleColor,
        mm.meepleID
      );
      useMeeple(
        tm.playerID === game.value.player0ID
          ? player0Meeples.value
          : player1Meeples.value,
        { y: tilePosY, x: tilePosX },
        mm.meepleID
      );
    }

    if (isMyGame.value) {
      meepleablePositions.value = [];
    }

    tileCount.value++;

    await sleep(500);

    await processCompleteEvents(board.value.completeEvents);
  };
  const updateDiscardMove = async (dm: DiscardMove) => {
    discardedTileKinds.value.push(dm.tile);

    tileCount.value++;
  };

  if (lastMove.playerID === 1) {
    // discard move
    if ("tile" in lastMove && !("rot" in lastMove)) {
      await updateDiscardMove(lastMove as DiscardMove);

      alert(`AI discarded a tile.`);

      waitAIMove();
    } else {
      // AI's tile move and meeple move must happen at once
      const tileMove = moves.value[moves.value.length - 2] as TileMove;
      const meepleMove = moves.value[moves.value.length - 1] as MeepleMove;

      await updateTileMove(tileMove);

      if (isMyGame.value && placingTile.value) {
        placeablePositions.value = getPlaceablePositions(placingTile.value);
      }

      await sleep(500);

      await updateMeepleMove(meepleMove, tileMove);

      if (game.value.currentTileID === -1) {
        await finishGame();
      }

      if (isMyGame.value && placeablePositions.value.length === 0) {
        mustDiscard.value = true;
      }
    }
  } else {
    if ("tile" in lastMove && !("rot" in lastMove)) {
      await updateDiscardMove(lastMove as DiscardMove);

      if (isMyGame.value) {
        if (game.value.nextTileID !== -1) {
          const placingTileKind = idToTileKind(game.value.currentTileID);
          placingTile.value = newTile(0, placingTileKind, null, -1, -1);
          placeablePositions.value = getPlaceablePositions(placingTile.value);
          if (placeablePositions.value.length !== 0) {
            mustDiscard.value = false;
          }
        }
      }
    } else if ("rot" in lastMove) {
      await updateTileMove(lastMove as TileMove);

      if (isMyGame.value) {
        placeablePositions.value = [];
        meepleablePositions.value = board.value.meepleablePositions;
        if (
          meepleablePositions.value.length === 0 ||
          player0Meeples.value.size === 0
        ) {
          handlePlaceMeeple(-1);
        }
      }
    } else if ("meepleID" in lastMove) {
      placingTile.value = null;

      await updateMeepleMove(
        lastMove as MeepleMove,
        moves.value[moves.value.length - 2] as TileMove
      );

      if (isMyGame.value) {
        placingPosition.value = null;
        if (game.value.nextTileID !== -1) {
          const placingTileKind = idToTileKind(game.value.nextTileID);
          placingTile.value = newTile(0, placingTileKind, null, -1, -1);
          placeablePositions.value = getPlaceablePositions(placingTile.value);
        }

        if (game.value.currentTileID === -1) {
          await finishGame();
        } else {
          waitAIMove();
        }
      }
    }
  }
};

const finishGame = async () => {
  if (!game.value) {
    return;
  }
  const api = new API();

  const finalEvents = await api.getFinalEevnts(game.value.id);

  await processCompleteEvents(finalEvents.completeEvents);

  game.value = await api.getGame(game.value.id);

  finished.value = true;
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

const initialUpdate = async () => {
  if (!game.value) {
    return;
  }

  const api = new API();
  board.value = await api.getBoard(
    game.value.id,
    game.value.player0Color,
    game.value.player1Color
  );

  tiles.value = board.value.tiles;
  player0Point.value = board.value.player0Point;
  player1Point.value = board.value.player1Point;

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
    const tilePosY = tileMove.pos.y + Math.floor(boardSize / 2);
    const tilePosX = tileMove.pos.x + Math.floor(boardSize / 2);
    if (tileMove.playerID === game.value?.player0ID) {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player0Color);
      player0LastTilePos.value = { y: tilePosY, x: tilePosX };
    } else {
      tiles.value[tilePosY][tilePosX]?.addFrame(game.value.player1Color);
      player1LastTilePos.value = { y: tilePosY, x: tilePosX };
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
    const lastMove = moves.value[moves.value.length - 1];
    if ("rot" in lastMove) {
      meepleablePositions.value = board.value.meepleablePositions;
      const lastTileMove = lastMove as TileMove;
      placingPosition.value = {
        y: lastTileMove.pos.y + Math.floor(boardSize / 2),
        x: lastTileMove.pos.x + Math.floor(boardSize / 2),
      };
    } else {
      const placingTileID =
        game.value.currentPlayerID === game.value.player0ID
          ? game.value.currentTileID
          : game.value.nextTileID;
      const placingTileKind = idToTileKind(placingTileID);
      placingTile.value = newTile(0, placingTileKind, null, -1, -1);
      placeablePositions.value = getPlaceablePositions(placingTile.value);

      if (
        game.value.currentPlayerID === game.value.player0ID &&
        placeablePositions.value.length === 0
      )
        mustDiscard.value = true;
    }
  }
};

const currentTile = () => {
  if (!game.value) {
    return null;
  }
  if (game.value.currentTileID !== null) {
    return newTile(0, idToTileKind(game.value.currentTileID), null, -1, -1);
  }
};

const skip = () => {
  handlePlaceMeeple(-1);
};

const boardStyle = computed(() => {
  return {
    "background-image": "url(" + WoodImg + ")",
  };
});

const placingTileSrc = computed(() => {
  return placingTile.value?.src;
});

const waitAIMove = () => {
  if (!game.value) {
    return;
  }

  const api = new API();
  api.waitAIMove(game.value.id);
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
  await joinGame();
  await initialUpdate();
  if (isMyGame.value && game.value?.currentPlayerID !== game.value?.player0ID) {
    waitAIMove();
  }
});
</script>
<template>
  <div v-if="!finished">
    <div
      class="bg-gray-100 rounded text-gray-900 text-sm px-4 py-3 shadow-md flex justify-between"
    >
      <div class="flex" v-if="game?.currentTileID !== -1">
        <div
          v-if="game?.currentPlayerID !== game?.player0ID"
          class="flex flex-col justify-center mr-3"
        >
          <p>
            {{ translate("ai_must_place") }}
          </p>
        </div>
        <div v-else class="flex flex-col justify-center mr-3">
          <p v-if="mustDiscard">{{ translate("you_must_discard") }}</p>
          <p v-else>{{ translate("you_must_place") }}</p>
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
        <SpinnerIcon v-if="game?.currentPlayerID !== game?.player0ID" />
        <div
          class="flex flex-col justify-center"
          v-if="game?.currentPlayerID === game?.player0ID"
        >
          <button
            class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
            v-if="
              isMyGame && placingPosition && meepleablePositions.length === 0
            "
            @click.once="confirm"
            :disabled="confirming"
          >
            {{ translate("confirm") }}
          </button>
          <button
            class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
            v-else-if="isMyGame && meepleablePositions.length !== 0"
            @click.once="skip"
            :disabled="handlingPlaceMeeple"
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
        </div>
      </div>
      <div v-else>{{ translate("calculating_final_points") }}</div>
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
                :src="newTile(0, discardedTileKind, null, -1, -1).src"
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
    </div>
  </div>
  <div class="infos flex flex-wrap">
    <PlayerInfo
      :name="game ? game.player0Name : ''"
      :point="player0Point"
      :meepleNumber="player0Meeples.size"
      :meepleColor="game ? game.player0Color : null"
      :tileSrc="placingTileSrc"
    />
    <PlayerInfo
      :name="game ? game.player1Name : ''"
      :point="player1Point"
      :meepleNumber="player1Meeples.size"
      :meepleColor="game ? game.player1Color : null"
      :tileSrc="null"
    />
  </div>
  <div class="board mt-3" :style="boardStyle">
    <GameBoard
      :tiles="tiles"
      :placeablePositions="placeablePositions"
      :placingTile="placingTile"
      :placingPosition="placingPosition"
      :meepleablePositions="meepleablePositions"
      @tilePositionSelected="handleTilePositionSelected"
      @turnTile="handleTurnTile"
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
