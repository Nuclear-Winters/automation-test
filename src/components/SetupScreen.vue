<template>
  <p id="centered"><b> Welcome to Automaton! </b></p>
  <p>Before we get started, please enter a password that will be used to encrypt the config data of this program</p>
  <div>
    <label for="input_box">Insert password, then press "encrypt"</label>
    <input id="input_box" v-model="password" type="password" name="pass_box">
    <button id="input_button" @click="create_config" >Encrypt</button>
  </div>
</template>

<script setup>
import {ref} from "vue";
import { invoke } from '@tauri-apps/api/tauri'

const password = ref("")

function create_config() {
  invoke("create_config",{ pass: password.value })
  password.value ="";
}
</script>

<script>
export default {
  name: "SetupScreen"
}
</script>

<style scoped>
#input_button{
  box-shadow: none;
  border: none;
  background: rgb(57, 61, 63);
  color: white;
}

#input_box{
  color: white;
  border-radius: 2px;
  align-self: center;
  background: rgb(57, 61, 63);
  box-shadow: none;

}
#centered{
  text-align: center;
  font-family: Inter,sans-serif;
  color: black;
}
</style>