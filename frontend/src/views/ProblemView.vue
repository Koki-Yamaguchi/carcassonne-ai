<script setup lang="ts">
import { onMounted, ref, computed, watch } from "vue";
import {
  Problem,
  Player,
  Game,
  Board,
  TileMove,
  DiscardMove,
  TilePosition,
  Vote,
} from "../types";
import { API } from "../api";
import { useRoute } from "vue-router";
import { store } from "../store";
import GameBoard from "../components/GameBoard.vue";
import PlayerInfo from "../components/PlayerInfo.vue";
import VoteItems from "../components/VoteItems.vue";
import ChevronIcon from "../components/ChevronIcon.vue";
import SolvedSign from "../components/SolvedSign.vue";

import {
  boardSize,
  idToTileKind,
  newTile,
  Tile,
  TileKind,
  getRemainingTileKinds,
} from "../tiles";
import { translate, translate_with_arg } from "../locales/translate";
import HeartIcon from "../components/HeartIcon.vue";

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
const canCancel = ref<boolean>(false);
const canMeeple = ref<boolean>(false);
const canSubmit = ref<boolean>(false);
const meeplingPosition = ref<number>(-1);
const note = ref<string>("");
const showRemainingTiles = ref<boolean>(false);
const showPointDiff = ref<boolean>(false);
const remainingTilesSrc = ref<string[]>([]);
const fixBoard = ref<boolean>(false);

const voted = ref<boolean>(false);
const votes = ref<Vote[]>([]);
const currentVoteID = ref<number>(0);
const prevVote = ref<Vote | null>(null);

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
  if (voted.value) {
    return;
  }

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

const getOneMeeple = (meeples: Set<number>): number => {
  for (let meeple of meeples.keys()) {
    return meeple;
  }

  return -1;
};

const confirm = async () => {
  if (
    !game.value ||
    !player.value ||
    !placingTile.value ||
    !placingPosition.value
  ) {
    return;
  }

  canConfirm.value = false;

  const posY = placingPosition.value.y;
  const posX = placingPosition.value.x;
  tiles.value[posY][posX] = placingTile.value;
  tiles.value[posY][posX]?.addFrame("black");

  if (player0Meeples.value.size !== 0) {
    const api = new API();
    const res = await api.tryCreateTileMove(
      game.value.id,
      player.value.id,
      game.value.currentTileID,
      placingTile.value.direction,
      posY,
      posX
    );

    meepleablePositions.value = res.meepleablePositions;

    canMeeple.value = true;
  } else {
    await handlePlaceMeeple(-1);
  }
  placeablePositions.value = [];
};

const skip = () => {
  handlePlaceMeeple(-1);
};

const handlePlaceMeeple = async (pos: number) => {
  if (!game.value || !player.value || !placingPosition.value) {
    return;
  }

  tiles.value[placingPosition.value?.y][placingPosition.value?.x]?.placeMeeple(
    pos,
    "yellow",
    6
  );

  meepleablePositions.value = [];
  meeplingPosition.value = pos;
  canMeeple.value = false;
  canCancel.value = true;
  canSubmit.value = true;
};

const cancel = () => {
  if (!placingTile.value || !placingPosition.value) {
    return;
  }
  tiles.value[placingPosition.value.y][placingPosition.value.x] = null;
  placingPosition.value = null;
  placingTile.value.addFrame(null);
  placingTile.value.removeMeeple();
  meepleablePositions.value = [];
  placeablePositions.value = getPlaceablePositions(placingTile.value);
  canConfirm.value = true;
  canCancel.value = false;
  canSubmit.value = false;
};

const createVote = async () => {
  if (
    !player.value ||
    !game.value ||
    !placingTile.value ||
    !placingPosition.value ||
    !problem.value
  ) {
    return;
  }
  const api = new API();

  const { tileMove, meepleMove } = await api.createMove(
    null,
    player.value.id,
    game.value.currentTileID,
    placingTile.value.direction,
    placingPosition.value.y,
    placingPosition.value.x,
    meeplingPosition.value === -1 ? -1 : getOneMeeple(player0Meeples.value),
    meeplingPosition.value,
    false
  );

  await api.createVote(
    problem.value.id,
    player.value.id,
    player.value.name,
    note.value,
    tileMove.id,
    meepleMove.id
  );

  placingTile.value = null;
  tiles.value[placingPosition.value.y][placingPosition.value.x] = null;
  placingPosition.value = null;
  localStorage.removeItem(`problem-${problem.value.id}-note`);

  voted.value = player.value.id !== 2;
  votes.value = await api.getVotes(problem.value.id, null);
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

  // if there's already a vote from the player, show results
  const tmpVotes = await api.getVotes(problem.value.id, null);
  const myVotes = tmpVotes.filter((v) => v.playerID === player.value?.id);
  if (myVotes.length > 0 && player.value.id !== 2) {
    placeablePositions.value = [];
    votes.value = tmpVotes;
    voted.value = true;
  }

  document.addEventListener("scroll", () => {
    if (window.scrollY >= 203 && voted.value) {
      fixBoard.value = true;
    } else {
      fixBoard.value = false;
    }
  });

  const cachedNote = localStorage.getItem(`problem-${problem.value.id}-note`);
  if (cachedNote) {
    note.value = cachedNote;
  }
});

watch(note, (newNote: string) => {
  if (problem.value) {
    localStorage.setItem(`problem-${problem.value.id}-note`, newNote);
  }
});

const handleClickVote = (voteID: number) => {
  if (currentVoteID.value === voteID) {
    currentVoteID.value = 0;
  } else {
    currentVoteID.value = voteID;
  }
  if (currentVoteID.value !== 0) {
    const vote = votes.value.filter((v) => v.id === currentVoteID.value)[0];
    updateBoard(vote);
  } else {
    updateBoard(null);
  }
};

const updateBoard = async (vote: Vote | null) => {
  if (!player.value) {
    return;
  }

  if (prevVote.value && prevVote.value.tileMove) {
    tiles.value[prevVote.value.tileMove.pos.y][prevVote.value.tileMove.pos.x] =
      null;
  }

  if (vote && vote.tileMove && vote.meepleMove) {
    const tileMove = vote.tileMove;
    const meepleMove = vote.meepleMove;
    const tile = newTile(
      tileMove.rot,
      tileMove.tile,
      "yellow",
      meepleMove.pos,
      meepleMove.meepleID,
      player.value.tileEdition
    );
    const posY = tileMove.pos.y;
    const posX = tileMove.pos.x;
    tiles.value[posY][posX] = tile;
    tiles.value[posY][posX]?.addFrame("black");

    prevVote.value = vote;
  }
};

const creatorName = computed(() => {
  if (!problem.value) {
    return "";
  }
  return problem.value.creatorName ?? "admin";
});

const tweetText = computed(() => {
  if (!problem.value || !player.value) {
    return "";
  }
  if (player.value.id === 5) {
    return (
      `https://twitter.com/intent/tweet?text=` +
      `今日のどこ置くの問題に投票しました！%0a` +
      `%0a` +
      `${formatNumber.value}. ${problem.value.name}%0a` +
      `作成者 ${creatorName.value}%0a` +
      `https://top-carcassonner.com/problems/${problem.value.id}%0a` +
      `%0a` +
      `%23TopCarcassonner`
    );
  } else {
    return (
      `https://twitter.com/intent/tweet?text=` +
      `どこ置くの問題に投票しました！%0a` +
      `%0a` +
      `${formatNumber.value}. ${problem.value.name}%0a` +
      `https://top-carcassonner.com/problems/${problem.value.id}%0a` +
      `%0a` +
      `%23TopCarcassonner`
    );
  }
});

const isDraft = computed(() => {
  return problem.value && problem.value.isDraft;
});

const isAdmin = computed(() => {
  return player.value && player.value.id === 2;
});

const formatNumber = computed(() => {
  if (!problem.value || !problem.value.num) {
    return "000";
  }
  const num = problem.value.num;
  if (num < 10) {
    return `00${num}`;
  }
  if (num < 100) {
    return `0${num}`;
  }
  if (num < 1000) {
    return `${num}`;
  }
  return "XXX";
});

const toggleFavorite = async () => {
  if (!problem.value || !player.value) {
    return;
  }

  const api = new API();

  if (!problem.value.favorited) {
    await api.createFavorite(
      player.value.id,
      player.value.name,
      problem.value.id
    );

    problem.value.favorited = true;
  } else {
    await api.deleteFavorite(player.value.id, problem.value.id);

    problem.value.favorited = false;
  }
};
</script>

<template>
  <div v-if="isDraft && !isAdmin" class="p-4">
    {{ translate("not_authorized") }}
  </div>
  <div v-else>
    <div class="mt-4 mx-4 flex justify-between">
      <div class="flex">
        <div class="flex flex-col justify-center text-sm">
          {{ formatNumber }}.
          {{ problem ? problem.name : "" }}
        </div>
        <div
          v-if="problem && problem.isSolved"
          class="flex flex-col justify-center items-center ml-2"
        >
          <SolvedSign />
        </div>
      </div>
      <div class="text-xs ml-1 mt-1 flex">
        <div>
          {{ translate("created_by") }} <b>{{ creatorName }}</b>
        </div>
        <div v-if="problem && problem.isSolved" class="ml-2">
          {{ translate("tested_by") }} <b>admin</b>
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
          @placeMeeple="handlePlaceMeeple"
          :isLarge="false"
        />
      </div>
      <div v-if="fixBoard" class="h-[350px] md:h-[600px]">
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
      <div class="flex justify-between">
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
          <div v-if="!voted" class="flex flex-col justify-center">
            <button
              class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2 text-xs"
              v-if="canConfirm"
              @click.once="confirm"
            >
              {{ translate("confirm") }}
            </button>
            <button
              class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2 text-xs"
              v-if="canMeeple"
              @click.once="skip"
            >
              {{ translate("skip") }}
            </button>
            <button
              class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2 text-xs"
              v-if="canCancel"
              @click="cancel"
            >
              {{ translate("try_again") }}
            </button>
          </div>
        </div>
        <div @click="toggleFavorite">
          <HeartIcon
            :isRed="problem ? problem.favorited : false"
            :isLarge="true"
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
      <div v-if="!voted" class="mt-4">
        <textarea
          class="rounded-md p-2 w-full focus:outline-none focus:border-orange-200 border-2"
          rows="3"
          cols="30"
          :placeholder="translate('comment')"
          v-model="note"
        />
        <div class="flex flex-col items-center">
          <button
            class="bg-gray-600 hover:bg-gray-500 disabled:bg-gray-300 text-[#eeeeee] rounded px-4 py-2 mt-2"
            @click.once="createVote"
            :disabled="!canSubmit"
          >
            {{ translate("vote") }}
          </button>
        </div>
      </div>
      <div v-else>
        <p class="mt-4">
          {{ translate("vote_results") }}
        </p>
        <div class="p-2">
          <VoteItems
            :votes="votes"
            :currentVoteID="currentVoteID"
            @clickVote="handleClickVote"
          />
        </div>
        <div
          class="border rounded-full w-20 py-2 px-3 bg-gray-200 hover:bg-gray-300"
        >
          <a
            v-if="problem"
            target="_blank"
            rel="noopener"
            :href="tweetText"
            data-size="large"
          >
            <span class="flex gap-2">
              <div class="w-4 flex flex-col justify-center">
                <img src="../assets/img/x-logo.png" />
              </div>
              <div class="flex flex-col justify-center">Post</div>
            </span>
          </a>
        </div>
      </div>
    </div>
  </div>
</template>
