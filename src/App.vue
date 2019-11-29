<template lang="pug">
#app
    router-view
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({});
</script>

<style lang="stylus">
#app
    margin 0
    padding 0
    width 100%
    height 100%

    router-view
        width 100%
        height 100%

.card
    border-radius 24px
    box-shadow 0 16px 32px rgba(0, 0, 0, 0.25);
    padding 32px

.list-card
    background rgba(0, 0, 0, 0.5)
    color #ffffff80
    width 40%
    overflow hidden

    .header
        text-align center
        color white
        vertical-align top
        cursor pointer

        span
            vertical-align top

    ul
        list-style-type none
        padding 0
        margin 0
        height 256px

        li
            margin 0
            border-top 1px solid #ffffff80

    .icon
        display inline
        margin 0
        padding 0
        margin-right 8px

    .nothing
        margin 0
        margin-top 16px
        display block
        width 100%
        height 128px
        text-align center

    .expand-enter-active, .expand-leave-active
        transition all cubic-bezier(0.5, 0, 0, 1) 0.35s, opacity 0.1s 0.1s

    .expand-enter, .expand-leave-to
        height 0px
        opacity 0
        margin-top 0

</style>
