<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";
import { API } from "../api";
import { store } from "../store";
import { Player } from "../types";

const player = ref<Player>();
const evtSource = ref<any>(null);
const searching = ref<boolean>(false);
const router = useRouter();

const search = async () => {
  if (!player.value) {
    return;
  }
  searching.value = true;

  const api = new API();
  const waitingGames = await api.getWaitingGames();
  if (
    waitingGames.length === 0 ||
    waitingGames[0].playerID === player.value.id
  ) {
    await api.deleteWaitingGame(player.value.id);

    const waitingGame = await api.createWaitingGame(player.value.id);
    evtSource.value = await api.subscribe(
      "wait_game",
      waitingGame.id,
      waitGameHandler
    );
  } else {
    const waitingGame = waitingGames[0];

    const player0ID = player.value.id;
    const player0Color = player.value.meepleColor;

    const player1ID = waitingGame.playerID;
    const player1 = await api.getPlayer(player1ID);
    const player1Color =
      player1.meepleColor !== player0Color
        ? player1.meepleColor
        : player0Color !== "red"
        ? "red"
        : "yellow";

    const game = await api.createGame(
      player0ID,
      player1ID,
      player0Color,
      player1Color,
      false
    );

    await api.updateWaitingGame(waitingGame.id, game.id);

    await disconnect();

    evtSource.value = await api.subscribe(
      "join_game",
      game.id,
      joinGameHandler
    );

    await api.sendEvent("wait_game", waitingGame.id);

    await sleep(5000);

    await api.deleteWaitingGame(waitingGame.playerID);

    await search();
  }
};

const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

const joinGameHandler = async (event: any) => {
  const gameID: number = JSON.parse(event.data).id;

  await disconnect();

  router.push(`/games/${gameID}`);
};

const waitGameHandler = async (event: any) => {
  const api = new API();
  const waitingGameID: number = JSON.parse(event.data).id;

  const allWaitingGames = await api.getWaitingGames();
  const waitingGames = allWaitingGames.filter((wg) => wg.id === waitingGameID);
  if (waitingGames.length === 0) {
    await disconnect();
    return;
  }
  const waitingGame = waitingGames[0];

  if (!waitingGame.game_id) {
    await disconnect();
    return;
  }

  await disconnect();

  await api.sendEvent("join_game", waitingGame.game_id);

  router.push(`/games/${waitingGame.game_id}`);
};

const disconnect = async () => {
  if (!player.value) {
    return;
  }
  const api = new API();
  await api.deleteWaitingGame(player.value.id);
  if (evtSource.value) {
    evtSource.value.close();
  }
};

const cancel = async () => {
  searching.value = false;
  await disconnect();
};

onMounted(async () => {
  const api = new API();
  player.value = await api.getPlayerByUserID(store.userID);
});

onUnmounted(async () => {
  await disconnect();
});
</script>

<template>
  <button v-if="!searching" @click="search">Play random match!</button>
  <button v-else @click="cancel">Cancel</button>
</template>
