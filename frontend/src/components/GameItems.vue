<script setup lang="ts">
import { Game } from "../types";
import GameItem from "./GameItem.vue";
import { translate } from "../locales/translate";

defineProps<{
  games: Game[];
  pointOfViewPlayerID: number | null;
}>();
</script>
<template>
  <div class="">
    {{ translate("your_games") }}
    <ul>
      <li v-for="game in games" :key="game.id" class="mt-2">
        <GameItem :game="game" :pointOfViewPlayerID="pointOfViewPlayerID" />
      </li>
    </ul>
  </div>
</template>
