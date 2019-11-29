import Vue from "vue";
import Vuex from "vuex";
import { StoreActions } from "@/enums/StoreActions";
import { HistoryListItem } from "@/models/HistoryListItem";
import HttpService from "@/services/HttpService";

Vue.use(Vuex);

export default new Vuex.Store({
    state: {
        history: [] as HistoryListItem[],
        userID: (null as unknown) as number
    },
    mutations: {
        [StoreActions.HistoryPush](state, newItem: HistoryListItem) {
            state.history.push(newItem);
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
        }
    },
    modules: {}
});
