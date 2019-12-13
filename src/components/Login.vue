<template lang="pug">
.container
    .card
        form(@submit.prevent="submit()")
            h1 Sign in
            input(:disabled="busy" required type="text", placeholder="Email", v-model.trim="email")
            input(:disabled="busy" required type="password", placeholder="Password", v-model="password")
            .actions
                button(:disabled="busy" type="button", @click="goToRegister()") Create an account
                button.primary(:disabled="busy") Sign in
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import HTTPService from "../services/HTTPService";
import { StoreActions, StoreMutations } from "../enums/StoreTypes";
import { RegistrationInfo } from "../models/RegistrationInfo";

@Component({
    name: "login"
})
export default class Login extends Vue {
    private email: string = "";
    private password: string = "";
    private busy: boolean = false;
    private error: string = "";

    async submit() {
        let user;

        this.busy = true;
        try {
            user = await HTTPService.login(this.email, this.password);
        } catch (e) {
            this.error = e;
            this.busy = false;
        }
        this.$router.push({ name: "home" });
    }

    goToRegister() {
        this.$router.push({ name: "register" });
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
