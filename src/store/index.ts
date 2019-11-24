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
            HttpService.pushHistory(
                state.userID,
                newItem.title,
                newItem.private,
                newItem.relatedTask.id
            );
            commit(StoreActions.HistoryPush, newItem);
        }
    },
    modules: {}
});
