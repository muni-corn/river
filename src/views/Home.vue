<template lang="pug">
div
    Main
</template>

<script lang="ts">
import Vue from "vue";
import Main from "@/components/Main.vue";

export default Vue.extend({
    name: "home",
    components: {
        Main
    }
});
</script>

<style scoped lang="stylus">
div
    width 100%
    height 100%
</style>
