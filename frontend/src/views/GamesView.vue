<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { API } from "../api";
import GameItems from "../components/GameItems.vue";
import { Game } from "../types";
const route = useRoute();

const games = ref<Game[]>([]);

onMounted(async () => {
  const playerID = parseInt(route.query.player as string, 10);
  const isRated = (route.query.is_rated as string) === "true";
  const limit = 500;

  const api = new API();
  games.value = await api.getGames(playerID, isRated, limit);
});
</script>
<template>
  <div class="mx-4 my-4">
    <GameItems :games="games" />
  </div>
</template>
