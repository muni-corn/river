<template lang="pug">
.card
    div.idle(v-if="appIsBusy()") 
        p.status Busy...
        p.subject One moment...
    div(v-else-if="getAwayReason() || getCurrentTask()")
        #edit(v-if="editing")
            CheckIcon.clickable(@click="stopEdit()", style="float: right;")
            select.status(v-model="status") {{ status.toString() }}
                option(:value="WorkingStatus.Working") Working
                option(:value="WorkingStatus.Away") Away
            input.subject(v-if="getCurrentTask()", type="text", v-model="getCurrentTask().name")
            #stats
                ClockIcon
        #current(v-else)
            EditIcon.clickable(@click="edit()", style="float: right;")
            p.status {{ getStatus() }}
            p.subject(v-if="getCurrentTask()") {{ getCurrentTask().name }}
            #stats(v-if="getCurrentTask()")
                ClockIcon
    div.idle(v-else)
        p.status Idle
        p.subject Choose a task below or add a new one.
</template>

<script lang="ts">
// eslint-disable-next-line
import { ClockIcon, EditIcon, CheckIcon } from "vue-feather-icons";
import { Component, Prop, Vue } from "vue-property-decorator";
import { WorkingStatus } from "@/enums/WorkingStatus";
import { Events } from "@/enums/Events";
import { Task } from "@/models/Task";
import { HistoryListItem } from "@/models/HistoryListItem";
import { StoreActions } from "@/enums/StoreTypes";
import HTTPService from "@/services/HTTPService";
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
    private status: WorkingStatus = WorkingStatus.Away;
    private lastState: {
        title?: string;
        status?: WorkingStatus;
    } = {};

    private editing: boolean = false;

    edit() {
        this.editing = true;
        this.lastState = {
            title: this.getCurrentTask()!.name,
            status: this.status
        };
    }

    stopEdit() {
        this.editing = false;
        const differentTitle =
            this.lastState.title != this.getCurrentTask()!.name;
        const differentStatus = this.lastState.status != this.status;

        if (differentTitle) {
            this.$store.dispatch(StoreActions.HistoryPush, {
                at: new Date(),
                title: `Renamed "${this.lastState.title}" to "${
                    this.getCurrentTask()!.name
                }"`
            } as HistoryListItem);
        }
    }

    getStatus() {
        if (this.$store.state.currentTask) {
            return "Working";
        } else if (this.$store.state.awayReason && !this.$store.state.currentTask) {
            return "Away";
        } else {
            return "Idle";
        }
    }

    changeStatus() {}

    pushHistory() {
        this.$emit(Events.HistoryPush, null);
    }

    getCurrentTask() {
        return this.$store.state.currentTask;
    }

    getAwayReason() {
        return this.$store.state.awayReason;
    }

    appIsBusy() {
        return this.$store.getters.isBusy;
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

.idle
    opacity 0.25
</style>
