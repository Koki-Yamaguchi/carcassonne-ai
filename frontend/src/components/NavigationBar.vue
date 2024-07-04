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
  <div class="bg-gray-200 w-full h-14 flex justify-between px-4 relative z-30">
    <div
      class="flex flex-col justify-center text-lg hover:cursor-pointer"
      @click="
        isOpen = false;
        router.push('/');
      "
    >
      <div>Top Carcassonner</div>
    </div>

    <div class="flex flex-col justify-center text-gray-700">
      <div class="md:hidden">
        <MenuIcon class="hover:cursor-pointer" @click="isOpen = !isOpen" />
      </div>
      <div class="flex flex-col justify-center text-gray-700">
        <div
          class="w-full absolute left-0 top-14 md:static md:flex bg-gray-200 [&>div]:py-3 [&>div]:md:py-0 [&>div]:px-4 [&>div]:md:px-0 md:gap-4 [&>div]:hover:cursor-pointer"
          :class="isOpen ? '' : 'hidden'"
        >
          <div
            @click="
              isOpen = false;
              router.push('/normal');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("normal_mode") }}
          </div>
          <div
            @click="
              isOpen = false;
              router.push('/competitive');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("competitive_mode") }}
          </div>
          <div
            @click="
              isOpen = false;
              router.push('/lobby');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("random_match") }}
          </div>
          <div
            @click="
              isOpen = false;
              router.push('/problems');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("problems") }}
          </div>
          <div
            @click="
              isOpen = false;
              router.push('/problems/propose');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("propose_problem") }}
          </div>
          <div
            @click="
              isOpen = false;
              router.push('/simulator');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("simulator") }}
          </div>
          <div
            v-if="store.authenticated"
            @click="
              isOpen = false;
              router.push('/settings');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("profile") }}
          </div>
          <div
            v-if="!store.authenticated && !store.authenticating"
            @click="
              isOpen = false;
              router.push('/signin');
            "
            class="hover:bg-gray-100 md:hover:bg-gray-200"
          >
            {{ translate("sign_in") }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
