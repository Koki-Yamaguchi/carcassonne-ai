<script setup lang="ts">
import { useRouter } from "vue-router";
import { translate } from "../locales/translate";
import { ref } from "vue";
import MenuIcon from "../components/MenuIcon.vue";
import { store } from "../store";

const router = useRouter();

const isOpen = ref<boolean>(false);
</script>
<template>
  <div class="bg-gray-200 w-full h-14 flex justify-between px-4 relative z-10">
    <div
      class="flex flex-col justify-center text-lg"
      @click="
        isOpen = false;
        router.push('/');
      "
    >
      <div>Top Carcassonner</div>
    </div>

    <div class="flex flex-col justify-center text-gray-700">
      <div class="md:hidden">
        <MenuIcon @click="isOpen = !isOpen" />
      </div>
      <div class="flex flex-col justify-center text-gray-700">
        <div
          class="w-full absolute left-0 top-14 md:static md:flex md:gap-4 bg-gray-200 px-4"
          :class="isOpen ? '' : 'hidden'"
        >
          <div
            class="p-2 md:p-0"
            v-if="store.authenticated"
            @click="
              isOpen = false;
              router.push('/settings');
            "
          >
            {{ translate("profile") }}
          </div>
          <div
            class="p-2 md:p-0"
            v-if="!store.authenticated && !store.authenticating"
            @click="
              isOpen = false;
              router.push('/signin');
            "
          >
            {{ translate("sign_in") }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
