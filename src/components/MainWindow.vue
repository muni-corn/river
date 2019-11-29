<template lang="pug">
.card
    #edit(v-if="editing")
        CheckIcon.clickable(@click="stopEdit()", style="float: right;")
        select.status(v-model="status") {{ status.toString() }}
            option(:value="WorkingStatus.Working") Working
            option(:value="WorkingStatus.Away") Away
        input.subject(type="text" v-model="subject")
        #stats
            ClockIcon
    #current(v-else)
        EditIcon.clickable(@click="edit()", style="float: right;")
        p.status {{ status.toString() }}
        p.subject {{ subject }}
        #stats
            ClockIcon
</template>

<script lang="ts">
// eslint-disable-next-line
import { ClockIcon, EditIcon, CheckIcon } from "vue-feather-icons";
import { Component, Prop, Vue } from "vue-property-decorator";
import { WorkingStatus } from "@/enums/WorkingStatus";
import { Events } from "@/enums/Events";
import HttpService from "@/services/HttpService";
import HistoryList from "@/components/HistoryList.vue";

@Component({
    components: {
        ClockIcon,
        EditIcon,
        CheckIcon,
        HistoryList
    }
})
export default class MainWindow extends Vue {
    private WorkingStatus = WorkingStatus;
    private status: WorkingStatus = WorkingStatus.Working;
    private subject: string = "Web engineering project";
    private progress!: number;
    private minutesLeft!: number;
    private minutesSpent!: number;

    private editing: boolean = false;

    edit() {
        this.editing = true;
    }

    stopEdit() {
        this.editing = false;
    }

    changeStatus() {}

    pushHistory() {
        this.$emit(Events.HistoryPush, null);
    }
}
</script>

<style scoped lang="stylus">
.card
    background white
    display block
    width 40%
    height auto

.status
    display block
    font-size 16px
    margin 0
    margin-top 8px

.subject
    display block
    margin-top 16px
    font-size 32px
    margin-bottom 32px

input.subject
    border none
    border-bottom 2px solid gray
    outline none

    &:focus
        border-color #00aaff

select.status
    background none
    border none
    border-bottom 2px solid gray
    padding 0
    padding-bottom 2px
    outline none

.clickable
    cursor pointer
</style>
