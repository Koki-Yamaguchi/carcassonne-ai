<script setup lang="ts">
import { Game } from "../types";
import GameItem from "./GameItem.vue";
import { translate } from "../locales/translate";

defineProps<{
  games: Game[];
}>();
</script>
<template>
  <div class="mx-4 my-4">
    {{ translate("your_games") }}
    <ul>
      <li v-for="game in games" :key="game.id" class="mt-2">
        <GameItem :game="game" />
      </li>
    </ul>
  </div>
</template>
