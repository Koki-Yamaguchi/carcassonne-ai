<script setup lang="ts">
import { getAuth, signInWithEmailAndPassword } from "firebase/auth";
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

const email = ref<string>("");
const password = ref<string>("");
const error = ref<string>("");

const signin = async () => {
  const auth = getAuth();
  try {
    await signInWithEmailAndPassword(auth, email.value, password.value);
    router.push(`/`);
  } catch (e) {
    error.value = "ログインに失敗しました。再度お試しください。";
  }
};
</script>

<template>
  <div class="m-10">
    <form class="w-full max-w-sm">
      <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3">
          <label
            class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
            for="inline-password"
          >
            Email
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
            Password
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
      <div v-if="error" class="error text-sm text-red-500 mb-4">
        {{ error }}
      </div>
      <div class="md:flex md:items-center">
        <div class="md:w-1/3"></div>
        <div class="md:w-2/3">
          <button
            class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded"
            type="button"
            @click="signin"
          >
            Sign In
          </button>
        </div>
      </div>
    </form>
    <div class="mt-4 underline" @click="router.push('/signup')">
      Sign Up Here
    </div>
  </div>
</template>
