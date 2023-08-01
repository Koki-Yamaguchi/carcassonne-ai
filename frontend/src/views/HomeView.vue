<script setup lang="ts">
import { API } from "./../api";
import { useRouter } from "vue-router";
import { store } from "../store";

const router = useRouter();

const createGame = async () => {
  const api = new API();

  const player = await api.getPlayer(store.userID);

  const player0ID = player.id;
  const player1ID = 1;

  const player0Color = player.meepleColor;
  const player1Color = player0Color === "red" ? "yellow" : "red";
  const game = await api.createGame(
    player0ID,
    player1ID,
    player0Color,
    player1Color
  );
  router.push(`/games/${game.id}`);
};
</script>

<template>
  <div>
    <div class="flex flex-col items-center">
      <button
        class="mt-4 bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-4 py-2"
        @click="createGame"
      >
        Play
      </button>
    </div>
  </div>
</template>
