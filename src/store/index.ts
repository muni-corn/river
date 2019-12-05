import Vue from "vue";
import Vuex from "vuex";
import HttpService from "@/services/HttpService";
import { HistoryListItem } from "@/models/HistoryListItem";
import { RegistrationInfo } from "@/models/RegistrationInfo";
import { StoreActions, StoreMutations } from "@/enums/StoreTypes";

Vue.use(Vuex);

export default new Vuex.Store({
    state: {
        history: [] as HistoryListItem[],
        userToken: (null as unknown) as string
    },
    mutations: {
        [StoreMutations.HistoryPush](state: any, newItem: HistoryListItem) {
            state.history.push(newItem);
        },

        [StoreMutations.SetToken](state: any, token: string) {
            state.userToken = token;
        },

        [StoreMutations.ClearToken](state: any) {
            state.userToken = null;
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

        async [StoreActions.Login]({ commit }, { email, password }) {
            let token: string;
            try {
                token = await HttpService.login(email, password);
            } catch (e) {
                throw e;
            }
            commit(StoreMutations.SetToken, token);
        },

        async [StoreActions.Register]({ commit }, payload: RegistrationInfo) {
            let token: string;
            try {
                token = await HttpService.register(payload);
            } catch (e) {
                throw e;
            }
            commit(StoreMutations.SetToken, token);
        },

        async [StoreActions.Logout]({ commit }) {
            commit(StoreMutations.ClearToken);
        }
    },
    modules: {}
});
