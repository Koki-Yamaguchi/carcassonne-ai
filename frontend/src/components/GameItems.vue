<script setup lang="ts">
import { Game } from "../types";
import GameItem from "./GameItem.vue";
defineProps<{
  games: Game[];
}>();
</script>
<template>
  <div class="mx-4 my-4">
    Your Games
    <ul>
      <li v-for="game in games" :key="game.id" class="mt-2">
        <GameItem :game="game" />
      </li>
    </ul>
  </div>
</template>
