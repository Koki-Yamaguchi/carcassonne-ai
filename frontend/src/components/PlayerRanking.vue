<script setup lang="ts">
import { translate } from "../locales/translate";
import { Player } from "../types";

defineProps<{
  players: Player[];
}>();
</script>
<template>
  <p class="mt-6 mb-4">{{ translate("ranking") }}</p>
  <div class="relative overflow-x-auto border rounded-lg">
    <table class="w-full text-sm text-left">
      <tbody>
        <tr
          v-for="(player, idx) in players"
          class="bg-white border dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
          :key="player.id"
        >
          <td class="px-6 py-4">
            {{ idx + 1 }}
          </td>
          <td class="px-6 py-4">
            {{ player.name }}
          </td>
          <td class="px-6 py-4">
            {{ player.rating }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
