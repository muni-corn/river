<template lang="pug">
div
    router-view
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";

@Component
export default class Auth extends Vue {}
</script>

<style lang="stylus" scoped>
div
    background linear-gradient(to top, #bbddff, #ffffff)
    min-height 100vh
</style>
