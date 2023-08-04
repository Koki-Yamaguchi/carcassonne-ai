<script setup lang="ts">
import { API } from "./../api";
import { useRouter } from "vue-router";
import { store } from "../store";
import { Game, Player } from "../types";
import { onMounted, ref } from "vue";
import GameItems from "../components/GameItems.vue";

const router = useRouter();
const player = ref<Player | null>(null);
const games = ref<Game[]>([]);

const createGame = async () => {
  if (!player.value) {
    return;
  }
  const api = new API();
  const player0ID = player.value.id;
  const player1ID = 1;
  const player0Color = player.value.meepleColor;
  const player1Color = player0Color === "red" ? "yellow" : "red";
  const game = await api.createGame(
    player0ID,
    player1ID,
    player0Color,
    player1Color
  );
  router.push(`/games/${game.id}`);
};

onMounted(async () => {
  const api = new API();
  player.value = await api.getPlayer(store.userID);
  games.value = await api.getGames(player.value.id);
});
</script>

<template>
  <div>
    <div class="flex flex-col items-center">
      <button
        class="mt-4 bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-4 py-2"
        @click="createGame"
      >
        Play Now!
      </button>
    </div>
    <GameItems :games="games" />
  </div>
</template>
