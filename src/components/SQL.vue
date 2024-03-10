<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const sqlResult = ref("");
const sql = ref("");
const params = ref("");
const development = import.meta.env.DEV;

async function query(sql: string, params?: any) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  sqlResult.value = JSON.parse(await invoke("query", { sql, params: JSON.parse(params || '{}'), }));
  if (development) {
    console.log(sqlResult.value)
  }
}
</script>

<template>
  <form class="col" @submit.prevent="query(sql, params)">
    <input id="sql-input" v-model="sql" placeholder="Enter SurrelQL query..." />
    <input id="params-input" v-model="params" placeholder="Enter params as JSON..." />
    <button type="submit">SQL</button>
  </form>

  <p>{{ sqlResult }}</p>
</template>
