<template lang="pug">
div
    Header
    div.container
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
import { Task } from '../models/Task';

@Component({
    name: "Main",
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
div.container
    background linear-gradient(to top, #bbddff, #ffffff)
    display flex
    flex-direction column
    align-items center
    width 100%
    height calc(100% - 64px)

.mtop
    margin-top 32px

.mbottom
    margin-bottom 32px
</style>
