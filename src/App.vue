<template>
    <SetupScreen v-if="current ==='SETPG'"/>
    <SignInScreen v-if="current ==='SIGNPG'"/>
    <HomeScreen v-if="current ==='HOMEPG'"/>
</template>



<script>
import { ref} from 'vue';
import { invoke } from '@tauri-apps/api/tauri'
import SignInScreen from "@/components/SignInScreen.vue";
import SetupScreen from "@/components/SetupScreen.vue";
import HomeScreen from "@/components/HomeScreen.vue";
import {listen } from '@tauri-apps/api/event'

export default {
  name: 'App',
  components:{
    HomeScreen,
    SetupScreen,
    SignInScreen
  },
  setup(){
    const current = ref('')

    invoke('config_exists')
        .then((message) => {
          if (message){
            current.value ="SIGNPG";
          }
          else {
            current.value ="SETPG";
          }
        })
    listen('change_comp', event => {
      current.value = event.payload;
    })

    return {
      current
    }
  }
}

</script>

<style>
*{
  background: rgb(253, 253, 255);
}
</style>
