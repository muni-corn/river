import Vue from "vue";
import Vuex from "vuex";
import HTTPService from "@/services/HTTPService";
import { HistoryListItem } from "@/models/HistoryListItem";
import { StoreActions, StoreMutations } from "@/enums/StoreTypes";
import { Task } from "@/models/Task";

Vue.use(Vuex);

interface State {
    history: HistoryListItem[];
    todo: Task[];
    userName: string;
    currentTask: Task | null;
    awayReason: string | null;
    busyTasks: number;
}

export default new Vuex.Store({
    state: {
        history: [],
        todo: [],
        userName: "",
        currentTask: null,
        awayReason: null,
        busyTasks: 0
    } as State,

    mutations: {
        [StoreMutations.HistoryPush](state: State, newItem: HistoryListItem) {
            state.history.push(newItem);
        },

        [StoreMutations.SetUserName](state: State, name: string) {
            state.userName = name;
        },

        [StoreMutations.SetTodo](state: State, todo: Task[]) {
            state.todo = todo;
        },

        [StoreMutations.SetHistory](state: State, history: HistoryListItem[]) {
            state.history = history;
        },

        [StoreMutations.SetCurrentTask](state: State, task: Task) {
            state.currentTask = task;
        },

        [StoreMutations.PushBusy](state: State) {
            state.busyTasks++;
        },

        [StoreMutations.PopBusy](state: State) {
            state.busyTasks--;
        },

        [StoreMutations.PushTask](state: State, task: Task) {
            state.todo.push(task);
        },
    },

    actions: {
        async [StoreActions.HistoryPush]({ commit }, newItem: HistoryListItem) {
            const taskID = newItem.relatedTaskID || null;
            await HTTPService.pushHistory(
                newItem.title,
                newItem.priv || false,
                taskID
            );
            commit(StoreActions.HistoryPush, newItem);
        },

        async [StoreActions.GetUserInformation]({ commit }) {
            const info = await HTTPService.getUserInformation();

            const tasksMapped = info.todo.map((task: any) => {
                return {
                    id: task.id,
                    name: task.name,
                    percentComplete: task.percent_complete,
                    minutesSpent: task.minutes_spent,
                    wasCompletedAt: task.was_completed_at,
                    creationDate: task.creation_date
                };
            });
            const historyMapped = info.history.map((item: any) => {
                const date = new Date(item.time);
                return {
                    at: date,
                    title: item.action,
                    priv: item.private,
                    relatedTaskID: item.related_task
                };
            });

            const user = info.user;
            const userName =
                user.display_name ||
                user.first_name + (user.last_name ? " " + user.last_name : "");

            commit(StoreMutations.SetUserName, userName);
            commit(StoreMutations.SetCurrentTask, info.currentTaskID);
            commit(StoreMutations.SetHistory, historyMapped);
            commit(StoreMutations.SetTodo, tasksMapped);
        },

        async [StoreActions.NewTask]({ commit, dispatch }, { name, priv }) {
            const rawTask = await HTTPService.newTask(name, priv);
            const task: Task = {
                id: rawTask.id,
                name: rawTask.name,
                minutesSpent: rawTask.minutes_spent,
                wasCompletedAt: rawTask.was_completed_at,
                priv: rawTask.private,
                creationDate: rawTask.creation_date,
                percentComplete: rawTask.percent_complete
            };
            commit(StoreMutations.PushTask, task);
            dispatch(StoreActions.HistoryPush, {
                priv: false,
                title: `Added task "${name}"`,
                at: new Date()
            } as HistoryListItem);
        },

        async [StoreActions.Start]({ state, commit, dispatch }, task: Task) {
            // don't switch if starting the already-current task
            if (state.currentTask && state.currentTask.id == task.id) {
                return;
            }

            dispatch(StoreActions.HistoryPush, {
                priv: false,
                title: `Switched to task "${task.name}"`,
                at: new Date()
            } as HistoryListItem);

            commit(StoreMutations.SetCurrentTask, task);
        }
    },

    getters: {
        isBusy(state: State) {
            return state.busyTasks > 0;
        }
    },

    modules: {}
});
