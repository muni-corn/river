<template lang="pug">
.container
    .card
        form(@submit.prevent="submit()")
            h1 Sign in
            input(required type="text", placeholder="Email", v-model.trim="email")
            input(required type="password", placeholder="Password", v-model="password")
            .actions
                button(type="button", @click="goToRegister()") Create an account
                button.primary Sign in
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import HttpService from "../services/HttpService";
import { StoreActions } from "../enums/StoreTypes";
import { RegistrationInfo } from "../models/RegistrationInfo";

@Component({
    name: "login"
})
export default class Login extends Vue {
    private email: string = "";
    private password: string = "";

    submit() {
        this.$store.dispatch(StoreActions.Login, {
            email: this.email,
            password: this.password
        });
    }

    goToRegister() {
        this.$router.push({ name: "register" })
    }
}
</script>

<style lang="stylus" scoped>
.container
    display flex
    align-items center

.card
    background white
    margin 0 auto
    max-width 100%
    width 384px
    height auto
</style>
