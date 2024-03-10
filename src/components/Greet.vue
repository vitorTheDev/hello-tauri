<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const sqlResult = ref("");
const sql = ref("create person set name = $name;");
const params = ref("{\"name\":\"vitor\", \"test\":{\"name\":\"vitor\"}}");

async function query(sql: string, params?: any) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  sqlResult.value = JSON.parse(await invoke("query", {
    sql, params: Object.entries(JSON.parse(params ?? "{}")).map(([key, val]) => [key, JSON.stringify(val)])
      .reduce((prev, [key, val]) => {
        // @ts-ignore
        prev[key] = val;
        return prev;
      }, {})
  }));
  console.log(sqlResult.value);
}
async function query2(sql: string, params?: any) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  sqlResult.value = JSON.parse(await invoke("query2", { sql, params: JSON.parse(params), }));
  console.log(sqlResult.value);
}
</script>

<template>
  <form class="col" @submit.prevent="query2(sql, params)">
    <input id="sql-input" v-model="sql" placeholder="Enter a sql..." />
    <input id="params-input" v-model="params" placeholder="Enter a params JSON..." />
    <button type="submit">SQL</button>
  </form>

  <p>{{ sqlResult }}</p>
</template>
