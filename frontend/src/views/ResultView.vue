<script setup lang="ts">
import { ref, onMounted } from "vue";
import { Game } from "../types";
import { API } from "../api";
import { useRoute, useRouter } from "vue-router";
import GameResultInfo from "../components/GameResultInfo.vue";
import { translate } from "../locales/translate";

const game = ref<Game | null>(null);
const player0ProfileImage = ref<string>("");
const player1ProfileImage = ref<string>("");
import { store } from "../store";

const router = useRouter();

const playAgain = async () => {
  if (!game.value) {
    return;
  }

  const api = new API();

  const player = await api.getPlayerByUserID(store.userID);

  const player0ID = player.id;
  const player1ID = 1;
  const player0Color = player.meepleColor;
  const player1Color = player0Color === "red" ? "yellow" : "red";
  const newGame = await api.createGame(
    player0ID,
    player1ID,
    player0Color,
    player1Color,
    game.value.isRated
  );

  router.push(`/games/${newGame.id}`);
};

onMounted(async () => {
  const api = new API();
  const route = useRoute();

  const gameID: number = parseInt(route.params.id as string, 10);
  game.value = await api.getGame(gameID);

  const player0 = await api.getPlayer(game.value.player0ID);
  player0ProfileImage.value = player0.profileImageURL;
  const player1 = await api.getPlayer(game.value.player1ID);
  player1ProfileImage.value = player1.profileImageURL;
});
</script>

<template>
  <div class="p-6">
    <p class="">{{ translate("game_result") }}</p>
    <div class="mt-2">
      <GameResultInfo
        v-if="game"
        :profileImage="player0ProfileImage"
        :name="game.player0Name"
        :point="game.player0Point"
        :beforeRating="game.beforePlayer0Rating"
        :afterRating="game.afterPlayer0Rating"
        :isWinner="game.winnerPlayerID === game.player0ID"
      />
      <GameResultInfo
        v-if="game"
        class="mt-2"
        :profileImage="player1ProfileImage"
        :name="game.player1Name"
        :point="game.player1Point"
        :beforeRating="game.beforePlayer1Rating"
        :afterRating="game.afterPlayer1Rating"
        :isWinner="game.winnerPlayerID === game.player1ID"
      />
    </div>
    <div class="flex gap-4 mt-4">
      <button
        class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
        @click.once="router.push(`/games/${game ? game.id : ''}/replay`)"
      >
        {{ translate("view_replay") }}
      </button>
      <button
        class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2"
        @click.once="playAgain"
      >
        {{ translate("play_again") }}
      </button>
    </div>
  </div>
</template>
