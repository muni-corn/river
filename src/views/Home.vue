<template lang="pug">
div
    Main
</template>

<script lang="ts">
import Vue from "vue";
import Column from "@/components/Column.vue";

export default Vue.extend({
    name: "home",
    components: {
        Column
    }
});
</script>

<style scoped lang="stylus">
div
    width 100%
    height 100%
</style>
