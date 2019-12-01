<template lang="pug">
div.home-content
    Header
    Column
</template>

<script lang="ts">
import Vue from "vue";
import Column from "@/components/Column.vue";
import Header from "@/components/Header.vue";

export default Vue.extend({
    name: "home",
    components: {
        Header,
        Column
    }
});
</script>

<style lang="stylus" scoped>
div.home-content
    background linear-gradient(to top, #bbddff, #ffffff)
    min-height 100vh
</style>
