<script setup lang="ts">
import { API } from "./../api";
import { useRouter } from "vue-router";

const router = useRouter();

const createGame = async () => {
  const api = new API();

  const player0ID = 0;
  const player1ID = 1;
  const game = await api.createGame(player0ID, player1ID);
  router.push(`/games/${game.id}`);
};
</script>

<template>
  <div>
    <button @click="createGame()">Play</button>
  </div>
</template>
