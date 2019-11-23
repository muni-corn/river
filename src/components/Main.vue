<template lang="pug">
div
    MainWindow
</template>

<script lang="ts">
import Vue from "vue";
import MainWindow from "./MainWindow.vue";

export default Vue.extend({
    name: "Main",
    components: {
        MainWindow
    }
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="stylus">
div
    background-color #ddeeff
    display flex
    align-items center
    width 100%
    height 100%
</style>
