<template lang="pug">
    
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class ProgressBar extends Vue {
    @Prop() private progress: number = 0; // between 0 and 1
}
</script>
