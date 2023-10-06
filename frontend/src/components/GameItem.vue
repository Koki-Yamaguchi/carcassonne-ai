<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { Game } from "../types";
import ArrowIcon from "../components/ArrowIcon.vue";

const props = defineProps<{
  game: Game;
  pointOfViewPlayerID: number | null;
}>();

const router = useRouter();

const winnerName = ref<string>("");
const winnerPoint = ref<number>(0);
const loserName = ref<string>("");
const loserPoint = ref<number>(0);
const finished = ref<boolean>(false);
const afterRating = ref<number>(0);
const diff = ref<number>(0);

const onClick = () => {
  if (finished.value) {
    router.push(`/replays/${props.game.id}`);
  } else {
    router.push(`/games/${props.game.id}`);
  }
};

onMounted(() => {
  if (props.game.winnerPlayerID === props.game.player0ID) {
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

  let player_id = props.pointOfViewPlayerID ?? props.game.player0ID;
  if (props.game.player0ID === player_id) {
    afterRating.value = props.game.afterPlayer0Rating;
    diff.value = props.game.afterPlayer0Rating - props.game.beforePlayer0Rating;
  } else {
    afterRating.value = props.game.afterPlayer1Rating;
    diff.value = props.game.afterPlayer1Rating - props.game.beforePlayer1Rating;
  }
});
</script>
<template>
  <div
    class="border rounded-md py-2 bg-white hover:bg-gray-50 hover:cursor-pointer"
    @click="onClick"
  >
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
      <div v-if="finished" class="flex">
        <div class="text-xs flex flex-col justify-center">
          <div class="relative">
            <ArrowIcon
              :color="diff === 0 ? 'gray' : diff > 0 ? 'green' : 'red'"
            />
          </div>
        </div>
        <div class="text-sm flex flex-col justify-center">
          {{ afterRating ?? 1500 }}
        </div>
      </div>
    </div>
  </div>
</template>
