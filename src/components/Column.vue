<template lang="pug">
div.column
    HistoryList.mtop(:data="historyData")
    MainWindow.mtop
    TodoList.mtop.mbottom(:data="todoList")
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import MainWindow from "./MainWindow.vue";
import HistoryList from "./HistoryList.vue";
import TodoList from "./TodoList.vue";
import Header from "./Header.vue";
import { HistoryListItem } from "../models/HistoryListItem";
import { Task } from "../models/Task";

@Component({
    name: "column",
    components: {
        MainWindow,
        HistoryList,
        TodoList,
        Header
    }
})
export default class Column extends Vue {
    private historyData: HistoryListItem[] = [];
    private todoList: Task[] = [];
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="stylus">
div.column
    display flex
    flex-direction column
    align-items center
    width auto
    margin 0 auto

.mtop
    margin-top 32px

.mbottom
    margin-bottom 32px
</style>
