<template lang="pug">
.header
    h1 River
    p {{ getName() }}
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";

@Component
export default class Header extends Vue {
    getName(): string {
        return this.$store.state.userName;
    }
}
</script>

<style scoped lang="stylus">
.header
    max-width 75%
    margin 0 auto
    height 64px
    display flex
    align-items center
    justify-content space-between

h1
    float left

h2
    float right
</style>
