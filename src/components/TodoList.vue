<template lang="pug">
.card.list-card
    .header(@click="expanded = !expanded")
        ListIcon.icon
        span Todo
    transition(name="expand")
        p.nothing(v-if="expanded && !appIsBusy() && getTasks().length === 0") No tasks
        ul(v-else-if="expanded && !appIsBusy()")
            li(v-for="t of getTasks()", @click="start(t)")
                p {{ t.name }}

    transition(name="expand")
        p.add-task(v-if="expanded && !appIsBusy()", @click="openAddTaskModal()") + Add task

    modal(v-show="showAddModal", @close="closeAddTaskModal()")
        h1(slot="header") Add task
        div(slot="content")
            input(:disabled="appIsBusy()", placeholder="Title", v-model="newTaskName")
            input#private(:disabled="appIsBusy()", type="checkbox", v-model="priv")
            label(for="private") Make this task private
        div(slot="actions")
            button(:disabled="appIsBusy()", @click="closeAddTaskModal()") Cancel
            button.primary(:disabled="appIsBusy()", @click="addTask()") Confirm
</template>

<script lang="ts">
import "reflect-metadata";
import { Component, Prop, Vue } from "vue-property-decorator";
import { Task } from "../models/Task";
import { ListIcon } from "vue-feather-icons";
import { StoreActions, StoreMutations } from "@/enums/StoreTypes";
import Modal from "./Modal.vue";

@Component({
    components: {
        ListIcon,
        Modal
    }
})
export default class TodoList extends Vue {
    private expanded: boolean = true;

    private showAddModal: boolean = false;

    private newTaskName: string = "";
    private priv: boolean = false;

    openAddTaskModal() {
        this.newTaskName = "";
        this.showAddModal = true;
    }

    closeAddTaskModal() {
        this.showAddModal = false;
    }

    async addTask() {
        await this.$store.dispatch(StoreActions.NewTask, {
            name: this.newTaskName,
            priv: this.priv
        });
        this.closeAddTaskModal();
    }

    appIsBusy() {
        return this.$store.getters.isBusy;
    }

    getTasks() {
        return this.$store.state.todo.filter((task: Task) => !this.getCurrentTask() || task.id != this.getCurrentTask().id);
    }

    start(task: Task) {
        this.$store.dispatch(StoreActions.Start, task);
    }

    getCurrentTask() {
        return this.$store.state.currentTask;
    }
}
</script>

<style lang="stylus" scoped>
p.add-task
    text-align center
    color white
    margin 0
    cursor pointer

ul
    max-height 100%
    overflow-y auto

li:hover
    cursor pointer
</style>
