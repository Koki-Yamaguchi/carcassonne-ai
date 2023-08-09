<script setup lang="ts">
import { getAuth, signOut } from "firebase/auth";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { API } from "../api";
import { store } from "../store";
import { colorToColorID } from "../tiles";
import { Player } from "../types";
import { translate } from "../locales/translate";

const router = useRouter();

const player = ref<Player | null>(null);
const name = ref<string>("");
const color = ref<number>(-1);

const update = async () => {
  const api = new API();
  await api.updatePlayer(
    player.value ? player.value.id : -1,
    name.value,
    color.value
  );

  router.push("/");
};

const signout = () => {
  const auth = getAuth();
  signOut(auth);
  store.setAuthenticated(false);
  router.push("/signin");
};

onMounted(async () => {
  const api = new API();
  player.value = await api.getPlayer(store.userID);
  name.value = player.value.name;
  color.value = colorToColorID(player.value.meepleColor);
});
</script>

<template>
  <div class="m-10">
    <form class="w-full max-w-sm">
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-full-name"
          >
            {{ translate("name") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <input
            class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-green-300"
            id="inline-full-name"
            type="text"
            v-model="name"
            disabled
          />
        </div>
      </div>
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4">
            {{ translate("meeple_color") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <select
            class="bg-gray-200 border-2 rounded w-full py-2 px-4 text-gray-700 focus:outline-none focus:bg-white focus:border-green-300"
            v-model="color"
          >
            <option value="0">{{ translate("red") }}</option>
            <option value="1">{{ translate("yellow") }}</option>
            <option value="2">{{ translate("green") }}</option>
            <option value="3">{{ translate("black") }}</option>
            <option value="4">{{ translate("blue") }}</option>
          </select>
        </div>
      </div>
      <div class="md:flex md:items-center">
        <div class="md:w-1/3"></div>
        <div class="md:w-2/3">
          <button
            class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded"
            type="button"
            @click="update"
          >
            {{ translate("update") }}
          </button>
        </div>
      </div>
    </form>
    <div class="mt-4 underline" @click="signout">
      {{ translate("sign_out") }}
    </div>
  </div>
</template>
