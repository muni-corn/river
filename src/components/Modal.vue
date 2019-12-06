<template lang="pug">
transition(name="modal-fade")
    div.backdrop
        .card.modal
            slot(name="header")
            div
                slot(name="content")
            .actions
                slot(name="actions")
                    button.primary(@click="close()") Close
</template>

<script lang="ts">
import "reflect-metadata";
import { Vue, Prop, Component } from "vue-property-decorator";
import { ModalListener } from "../models/ModalListener";

@Component({
    name: "modal"
})
export default class Modal extends Vue {
    @Prop() show!: boolean;

    close(data?: any) {
        this.$emit("close", data);
    }
}
</script>

<style lang="stylus">
div.backdrop
    position fixed
    background rgba(0, 0, 0, 0.5);
    top 0
    left 0
    right 0
    bottom 0
    display flex
    align-items center

div.modal
    background white
    border-radius 24px
    margin 0 auto

    h1
        color black

.modal-fade-enter, .modal-fade-leave-to
    opacity 0

    .modal
        transform translateY(30px) scale(0.95)

.modal-fade-enter-active, .modal-fade-leave-active
    transition opacity 0.25s

    .modal
        transition transform 0.25s cubic-bezier(0.5, 0, 0, 1)
</style>
