<script setup lang="ts">
import { translate } from "../locales/translate";
import { Player } from "../types";
import ProfileImage from "./ProfileImage.vue";

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
          class="bg-white border dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 [&>td]:py-4 [&>td]:px-6"
          :key="player.id"
        >
          <td>{{ idx + 1 }}</td>
          <ProfileImage class="py-4" :src="player.profileImageURL" />
          <td>{{ player.name }}</td>
          <td>{{ player.rating }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
