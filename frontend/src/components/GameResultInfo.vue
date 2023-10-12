<script setup lang="ts">
import ProfileImage from "./ProfileImage.vue";
import ArrowIcon from "./ArrowIcon.vue";

defineProps<{
  profileImage: string;
  name: string;
  point: number;
  beforeRating: number | null;
  afterRating: number | null;
  isWinner: boolean;
}>();
</script>

<template>
  <div class="flex gap-4 w-full border rounded-xl hover:bg-gray-50 p-4">
    <div class="flex flex-col justify-center">
      <ProfileImage :src="profileImage" />
    </div>
    <div class="w-full">
      <div class="flex justify-between">
        <div class="flex">
          <img v-if="isWinner" class="w-5 mr-1" src="../assets/img/crown.png" />
          <div>{{ name }}</div>
        </div>
        <div>{{ point }} pt</div>
      </div>
      <div class="flex mt-2">
        <div>{{ beforeRating ?? 1500 }}</div>
        <ArrowIcon
          :color="
            !beforeRating || !afterRating || beforeRating == afterRating
              ? 'gray'
              : afterRating > beforeRating
              ? 'green'
              : 'red'
          "
        />
        <div>{{ afterRating ?? 1500 }}</div>
      </div>
    </div>
  </div>
</template>
