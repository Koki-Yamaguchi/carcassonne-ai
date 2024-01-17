<script setup lang="ts">
import { getAuth, signOut } from "firebase/auth";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { API } from "../api";
import { store } from "../store";
import { colorToColorID, TileEdition } from "../tiles";
import { Player } from "../types";
import { translate } from "../locales/translate";

const router = useRouter();

const player = ref<Player | null>(null);
const name = ref<string>("");
const lang = ref<string>("");
const color = ref<number>(-1);
const tileEdition = ref<TileEdition | null>(null);
const rating = ref<number | undefined>();
const profileImageURL = ref<string>("");
const file = ref<File | null>(null);
const image = ref<string>("");
const input = ref<HTMLInputElement>();

const upload = () => {
  if (input.value) {
    input.value.click();
  }
};
const setImage = (event: any) => {
  file.value = event.target.files[0];
  const reader = new FileReader();
  reader.onloadend = () => {
    image.value = reader.result as string;
  };
  if (file.value) {
    reader.readAsDataURL(file.value);
  }
};

const update = async () => {
  if (!tileEdition.value) {
    return;
  }

  const api = new API();

  if (file.value && player.value) {
    let data = new FormData();
    data.append("profile_image", file.value);

    api.uploadProfileImage(player.value.id, data);
  }

  await api.updatePlayer(
    player.value ? player.value.id : -1,
    name.value,
    color.value,
    tileEdition.value
  );

  store.setLanguage(lang.value);
  localStorage.setItem("language", lang.value);

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
  player.value = await api.getPlayerByUserID(store.userID);
  name.value = player.value.name;
  color.value = colorToColorID(player.value.meepleColor);
  rating.value = player.value.rating;
  tileEdition.value = player.value.tileEdition;
  profileImageURL.value = player.value.profileImageURL;

  if (store.language !== "") {
    if (store.language === "ja") {
      lang.value = "ja";
    } else {
      lang.value = "en";
    }
  } else {
    if (window.navigator.language === "ja") {
      lang.value = "ja";
    } else {
      lang.value = "en";
    }
  }
});
</script>

<template>
  <div class="m-10">
    <form class="w-full max-w-sm">
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-profile-image"
          >
            {{ translate("image") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <img class="w-28 rounded" :src="image ? image : profileImageURL" />
          <input
            class="hidden"
            ref="input"
            type="file"
            accept="image/*"
            @change="setImage"
          />
          <button
            class="bg-gray-500 hover:bg-gray-400 text-[#eeeeee] rounded px-4 py-2 mt-2"
            type="button"
            @click="upload"
          >
            Upload
          </button>
        </div>
      </div>
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
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-full-name"
          >
            {{ translate("rating") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <input
            class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-green-300"
            id="inline-full-name"
            type="text"
            v-model="rating"
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
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4">
            {{ translate("tile_edition") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <select
            class="bg-gray-200 border-2 rounded w-full py-2 px-4 text-gray-700 focus:outline-none focus:bg-white focus:border-green-300"
            v-model="tileEdition"
          >
            <option value="first">{{ translate("first_edition") }}</option>
            <option value="second">{{ translate("second_edition") }}</option>
          </select>
        </div>
      </div>
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4">
            {{ translate("language") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <select
            class="bg-gray-200 border-2 rounded w-full py-2 px-4 text-gray-700 focus:outline-none focus:bg-white focus:border-green-300"
            v-model="lang"
          >
            <option value="ja">日本語</option>
            <option value="en">English</option>
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
