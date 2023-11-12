<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  page: number;
  limit: number;
  totalCount: number;
}>();

const emit = defineEmits<{
  (e: "pageClicked", page: number): void;
}>();

const maxPage = computed(() => {
  return Math.ceil(props.totalCount / props.limit) - 1;
});

const pages = computed(() => {
  const ps = [];
  if (props.page - 2 >= 0) ps.push(props.page - 2);
  if (props.page - 1 >= 0) ps.push(props.page - 1);
  ps.push(props.page);
  if (props.page + 1 <= maxPage.value) ps.push(props.page + 1);
  if (props.page + 2 <= maxPage.value) ps.push(props.page + 2);
  return ps;
});
</script>

<template>
  <div class="flex justify-center mt-6 text-gray-700">
    <div class="flex gap-4">
      <div v-for="p in pages" :key="p" class="w-8 h-8">
        <div
          @click="emit('pageClicked', p)"
          class="hover:cursor-pointer w-full h-full rounded-full flex justify-center"
          :class="p === page ? 'bg-green-400 text-white' : ''"
        >
          <div class="flex flex-col justify-center">
            <div>
              {{ p + 1 }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
