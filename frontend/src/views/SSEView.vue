<script setup lang="ts">
import { ref } from "vue";
import { API } from "../api";

const ord = ref<number>(0);
const msgs = ref<number[]>([]);
const gid = ref<number>(0);

const send = async () => {
  const api = new API();
  api.sendEvent(gid.value, ord.value);
  api.send(gid.value, ord.value);
};

const join = () => {
  const api = new API();

  const f = (event: any) => {
    const json = JSON.parse(event.data);
    msgs.value.push(json.ord);
  };
  api.events(gid.value, f);
};
</script>
<template>
  <input type="text" v-model="gid" />
  <button @click="join">join</button>
  <input type="text" v-model="ord" />
  <button @click="send">send</button>
  <div>{{ msgs }}</div>
</template>
