<template lang="pug">
.container
    .card
        form(@submit.prevent="submit()")
            h1 Welcome to River!
            input(:disabled="busy", required type="text", placeholder="First name", v-model.trim="firstName")
            input(:disabled="busy", type="text", placeholder="Last name", v-model.trim="lastName")
            input(:disabled="busy", type="text", placeholder="Display name", v-model.trim="displayName")
            input(:disabled="busy", required type="text", placeholder="Email", v-model.trim="email")
            input(:disabled="busy", required type="password", placeholder="Password", v-model="password")
            .actions
                button(:disabled="busy", type="button", @click="goToSignIn()") Sign in instead
                button.primary(:disabled="busy") Submit
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import HTTPService from "../services/HTTPService";
import { StoreActions } from "../enums/StoreTypes";
import { RegistrationInfo } from "../models/RegistrationInfo";

@Component({
    name: "register"
})
export default class Register extends Vue {
    private firstName: string = "";
    private lastName: string = "";
    private displayName: string = "";
    private email: string = "";
    private password: string = "";
    private busy: boolean = false;
    private error: string = "";

    async submit() {
        this.busy = true;
        try {
            await HTTPService.register({
                firstName: this.firstName,
                lastName: this.lastName,
                displayName: this.displayName,
                email: this.email,
                password: this.password
            });
        } catch (e) {
            this.error = e;
            this.busy = false;
        }
        this.$router.push({ name: "home" });
    }

    goToSignIn() {
        this.$router.push({ name: "login" });
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
    height initial
</style>
