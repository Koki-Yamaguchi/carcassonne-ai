<script setup lang="ts">
import { getAuth, sendPasswordResetEmail } from "firebase/auth";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { translate } from "../locales/translate";

const router = useRouter();

const email = ref<string>("");
const error = ref<string>("");
const sent = ref<boolean>(false);

const reset = async () => {
  const auth = getAuth();
  try {
    await sendPasswordResetEmail(auth, email.value);
    sent.value = true;
  } catch (e) {
    error.value = translate("failed_to_send_reset_password_link");
  }
};
</script>

<template>
  <div class="m-10">
    <div v-if="!sent">
      <div class="text-gray-500 text-sm mb-4">
        {{ translate("forgot_password_text") }}
      </div>
      <form class="w-full max-w-sm">
        <div class="md:flex md:items-center mb-6">
          <div class="md:w-1/3">
            <label
              class="block text-gray-500 md:text-right mb-1 md:mb-0 pr-4"
              for="inline-email"
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
        <div v-if="error" class="error text-sm text-red-500 mb-4">
          {{ error }}
        </div>
        <div class="md:flex md:items-center">
          <div class="md:w-1/3"></div>
          <div class="md:w-2/3">
            <button
              class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded"
              type="button"
              @click="reset"
            >
              {{ translate("reset_password") }}
            </button>
          </div>
        </div>
      </form>
    </div>
    <div v-else>
      <div class="text-gray-500 mb-4">
        {{ translate("reset_password_sent") }}
      </div>
      <button
        class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white py-2 px-4 rounded"
        @click="router.push(`/signin`)"
      >
        {{ translate("back_to_sign_in") }}
      </button>
    </div>
  </div>
</template>
