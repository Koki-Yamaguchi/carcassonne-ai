<script setup lang="ts">
import { ProblemProposal } from "../types";
import { translate } from "../locales/translate";

defineProps<{
  proposals: ProblemProposal[];
}>();
</script>
<template>
  <div v-if="proposals.length === 0">
    <p class="text-sm text-gray-600">{{ translate("no_proposed_problems") }}</p>
  </div>
  <div v-else class="flex">
    <table class="border rounded-xl border-separate p-2">
      <tr class="[&>*]:px-2">
        <td class="text-sm">{{ translate("table_id") }}</td>
        <td class="text-sm">{{ translate("remaining_tile_count") }}</td>
      </tr>
      <tr
        v-for="proposal in proposals"
        :key="proposal.id"
        class="[&>*]:p-2 text-gray-600 text-sm"
      >
        <td>{{ proposal.tableID }}</td>
        <td>{{ proposal.remainingTileCount }}</td>
      </tr>
    </table>
  </div>
</template>
