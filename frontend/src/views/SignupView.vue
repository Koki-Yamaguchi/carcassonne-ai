<script setup lang="ts">
import { getAuth, createUserWithEmailAndPassword } from "firebase/auth";
import { ref } from "vue";
import { API } from "../api";
import { useRouter } from "vue-router";
import { translate } from "../locales/translate";

const router = useRouter();

const email = ref<string>("");
const password = ref<string>("");
const name = ref<string>("");

const signup = async () => {
  const auth = getAuth();
  const res = await createUserWithEmailAndPassword(
    auth,
    email.value,
    password.value
  );
  const userID = res.user.uid;

  const api = new API();
  await api.createPlayer(name.value, email.value, userID);

  router.push(`/`);
};
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
          />
        </div>
      </div>
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-password"
          >
            {{ translate("email") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <input
            class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-green-300"
            id="inline-email"
            v-model="email"
          />
        </div>
      </div>
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-password"
          >
            {{ translate("password") }}
          </label>
        </div>
        <div class="md:w-2/3">
          <input
            class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-green-300"
            id="inline-password"
            type="password"
            v-model="password"
          />
        </div>
      </div>
      <div class="md:flex md:items-center">
        <div class="md:w-1/3"></div>
        <div class="md:w-2/3">
          <button
            class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded"
            type="button"
            @click="signup"
          >
            {{ translate("sign_up") }}
          </button>
        </div>
      </div>
    </form>
    <div class="mt-4 underline" @click="router.push('/signin')">
      {{ translate("sign_in_here") }}
    </div>
  </div>
</template>
