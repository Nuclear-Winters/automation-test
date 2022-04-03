<template>
  <div onclick="">{{serv_name}}</div>
</template>

<script>
import {ref} from "vue";

export default {
  name: "ServerComponent",
  props: ["name"],
  emits: ['server_click'],
  setup(props){
    const serv_name = ref(props.name);
    const activate_event = () => {
        this.$emit('server_click',serv_name)
    }

    return{
      activate_event,
      serv_name
    }
  },
  methods:{
    onClick: function (name) {
      this.$emit('server_click',name)
    }
  }
}
</script>

<style scoped>

</style>