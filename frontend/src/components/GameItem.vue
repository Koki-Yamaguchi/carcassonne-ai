<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { Game } from "../types";
const props = defineProps<{
  game: Game;
}>();

const router = useRouter();

const winnerName = ref<string>("");
const winnerPoint = ref<number>(0);
const loserName = ref<string>("");
const loserPoint = ref<number>(0);
const finished = ref<boolean>(false);

onMounted(() => {
  if (props.game.player0Point > props.game.player1Point) {
    winnerName.value = props.game.player0Name;
    winnerPoint.value = props.game.player0Point;
    loserName.value = props.game.player1Name;
    loserPoint.value = props.game.player1Point;
  } else {
    winnerName.value = props.game.player1Name;
    winnerPoint.value = props.game.player1Point;
    loserName.value = props.game.player0Name;
    loserPoint.value = props.game.player0Point;
  }
  finished.value = props.game.nextTileID === -1;
});
</script>
<template>
  <div class="border rounded-md py-2">
    <div class="flex px-4 justify-between">
      <div class="w-2/3">
        <div class="text-sm flex justify-between mb-2">
          <div class="flex">
            <div class="w-4 mr-1">
              <img v-if="finished" class="w-4" src="../assets/img/crown.png" />
            </div>
            <div>
              {{ winnerName }}
            </div>
          </div>
          <div>
            {{ winnerPoint }}
          </div>
        </div>
        <div class="text-sm flex justify-between">
          <div class="flex">
            <div class="w-4 mr-1"></div>
            {{ loserName }}
          </div>
          <div>
            {{ loserPoint }}
          </div>
        </div>
      </div>
      <button
        class="shadow bg-green-200 hover:bg-green-400 focus:shadow-outline focus:outline-none text-gray-700 w-20 rounded text-sm"
        @click="router.push(`/games/${game.id}`)"
      >
        {{ finished ? "Replay" : "Continue" }}
      </button>
    </div>
  </div>
</template>
