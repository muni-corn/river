import Vue from "vue";
import Vuex from "vuex";
import HttpService from "@/services/HttpService";
import { HistoryListItem } from "@/models/HistoryListItem";
import { RegistrationInfo } from "@/models/RegistrationInfo";
import { StoreActions, StoreMutations } from "@/enums/StoreTypes";
import {Task} from '@/models/Task';

Vue.use(Vuex);

interface State {
    history: HistoryListItem[];
    todo: Task[];
    userName: string;
}

export default new Vuex.Store({
    state: {
        history: [],
        todo: [],
        userName: ""
    } as State,
    mutations: {
        [StoreMutations.HistoryPush](state: any, newItem: HistoryListItem) {
            state.history.push(newItem);
        },

        [StoreMutations.SetUser](state: any, { firstName, lastName, displayName }) {
            state.userName = displayName || firstName + (lastName ? ` ${lastName}` : "");
        }
    },
    actions: {
        async [StoreActions.HistoryPush](
            { commit, state },
            newItem: HistoryListItem
        ) {
            const taskID = newItem.relatedTask ? newItem.relatedTask.id : null;
            await HttpService.pushHistory(
                state.userID,
                newItem.title,
                newItem.priv || false,
                taskID
            );
            commit(StoreActions.HistoryPush, newItem);
        },
    },
    modules: {}
});
