<template>
  <p id="centered"><b> Welcome to Automaton! </b></p>
  <p>Please enter in your config password</p>
  <input id="input_box" v-model="password" type="password">
  <button id="input_button" @click="load_config" >Decrypt</button>
</template>

<script setup>
import {invoke} from "@tauri-apps/api/tauri";
import {ref} from "vue";

const password = ref("")

function load_config() {
  invoke("load_config",{ pass: password.value })
  password.value ="";
}
</script>

<script>
export default {
  name: "SignInScreen"
}
</script>

<style scoped>
#centered{
  text-align: center;
  font-family: Inter,sans-serif;
  color: black;
}
</style>