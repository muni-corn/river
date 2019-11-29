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
</template>

<script lang="ts">
import "reflect-metadata";
import { Component, Prop, Vue } from "vue-property-decorator";
import { Task } from "../models/Task";
import { ListIcon } from "vue-feather-icons";

@Component({
    components: {
        ListIcon
    }
})
export default class TodoList extends Vue {
    @Prop() data!: Task[];

    private expanded: boolean = true;
}
</script>
