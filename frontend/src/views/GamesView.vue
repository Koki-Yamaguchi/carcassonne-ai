<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { API } from "../api";
import GameItems from "../components/GameItems.vue";
import { Game } from "../types";
const route = useRoute();

const games = ref<Game[]>([]);
const playerID = ref<number | null>(null);

onMounted(async () => {
  playerID.value = route.query.player
    ? parseInt(route.query.player as string, 10)
    : null;
  const isRated = (route.query.is_rated as string) === "true";
  const limit = 100;

  const api = new API();
  games.value = await api.getGames(playerID.value, isRated, limit);
});
</script>
<template>
  <div class="mx-4 my-4">
    <GameItems :games="games" :pointOfViewPlayerID="playerID" />
  </div>
</template>
