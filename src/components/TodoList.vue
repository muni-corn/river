<template lang="pug">
.card.list-card
    .header(@click="expanded = !expanded")
        ListIcon.icon
        span Todo
    transition(name="expand")
        p.nothing(v-if="expanded && data.length === 0") No tasks
        ul(v-else-if="expanded")
            li(v-for="d of data")
                p {{ d.name }}
    transition(name="expand")
        p.add-task(v-if="expanded", @click="addTask()") + Add task

    modal(v-show="showAddModal", @close="closeAddTask()")
        h1(slot="header") Add task
        div(slot="content")
            input
</template>

<script lang="ts">
import "reflect-metadata";
import { Component, Prop, Vue } from "vue-property-decorator";
import { Task } from "../models/Task";
import { ListIcon } from "vue-feather-icons";
import Modal from "./Modal.vue";

@Component({
    components: {
        ListIcon,
        Modal
    }
})
export default class TodoList extends Vue {
    @Prop() data!: Task[];

    private expanded: boolean = true;

    private showAddModal: boolean = false;

    addTask() {
        this.showAddModal = true;
    }

    closeAddTask() {
        this.showAddModal = false;
    }
}
</script>

<style lang="stylus" scoped>
p.add-task
    text-align center
    color white
    margin 0
    cursor pointer
</style>
