<template>
  <div><p>{{data}}</p></div>
</template>


<script>
import {ref} from "vue";
import { invoke } from '@tauri-apps/api/tauri'

export default {
  props:['server'],
  name: "TerminalMiddle",
  setup(props){
    const data = ref('~> ');
    setInterval(function (){
      invoke('get_task',{serverName: props.server})
      .then((message) => {
          data.value += message;
      })

    },2000)
    return{
      data
    }
  }
}
</script>

<style scoped>
p{;
  color: whitesmoke;
}
*{
  background: #1a1a1a;
  min-height: 565px;
  height: 100%;
}
</style>