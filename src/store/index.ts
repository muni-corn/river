import Vue from "vue";
import Vuex from "vuex";
import HTTPService from "@/services/HTTPService";
import { HistoryListItem } from "@/models/HistoryListItem";
import { StoreActions, StoreMutations } from "@/enums/StoreTypes";
import { Task } from '@/models/Task';

Vue.use(Vuex);

interface State {
    history: HistoryListItem[];
    todo: Task[];
    userName: string
    currentTaskID: number,
    awayReason: string,
}

export default new Vuex.Store({
    state: {
        history: [],
        todo: [],
        userName: "",
        currentTaskID: 0,
        awayReason: ""
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

        [StoreMutations.SetCurrentTask](state: State, id: number) {
            state.currentTaskID = id;
        }
    },

    actions: {
        async [StoreActions.HistoryPush](
            { commit },
            newItem: HistoryListItem
        ) {
            const taskID = newItem.relatedTaskID || null;
            await HTTPService.pushHistory(
                newItem.title,
                newItem.priv || false,
                taskID
            );
            commit(StoreActions.HistoryPush, newItem);
        },

        async [StoreActions.GetUserInformation](
            { commit }
        ) {
            const info = await HTTPService.getUserInformation();
            commit(StoreMutations.SetUserName, info.userName);
            commit(StoreMutations.SetCurrentTask, info.currentTaskID);
            commit(StoreMutations.SetHistory, info.history);
            commit(StoreMutations.SetTodo, info.todo);
        }
    },
    modules: {}
});
