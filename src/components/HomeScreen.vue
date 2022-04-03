<template>
  <div id="side_bar_left">TestText</div>
  <div id="side_bar_right">TestText</div>
  <div id="terminal"><TerminalMiddle server="" ></TerminalMiddle></div>
</template>

<script>
import TerminalMiddle from "@/components/TerminalMiddle";
import {ref} from "vue";

export default {
  name: "HomeScreen",
  components:{
    TerminalMiddle
  },
  setup(){
    const selected_server = ref('');
    return{
      selected_server
    }
  }
}
</script>

<style scoped>
#terminal{
  padding-right: 1px;
  padding-left: 1px;

}
#side_bar_left{
  flex-basis: 100px;
  height: 100%;
  min-width: 150px;
  min-height: 579px;
  padding-top: 0px;
  margin-top: 0px;
  border-top: 0px;
  padding-left: 0px;
  padding-right: 0px;
  float: left;
  margin-bottom: fill;
  background: rgb(237, 237, 234);
}
#side_bar_right{
  height: 100%;
  min-width: 150px;
  min-height: 579px;
  padding-top: 0px;
  margin-top: 0px;
  border-top: 0px;
  padding-left: 0px;
  padding-right: 0px;
  float: right;
  margin-bottom: fill;
  background: rgb(237, 237, 234);
}
</style>