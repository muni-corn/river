import axios, { AxiosResponse } from "axios";
import { HistoryListItem } from "@/models/HistoryListItem";
import { RegistrationInfo } from "@/models/RegistrationInfo";

export default class HttpService {
    static async test(): Promise<string> {
        const res = await axios.get("/api/test");
        // eslint-disable-next-line
        console.log(res.data);
        return res.data;
    }

    static async pushHistory(
        userID: number,
        title: string,
        priv: boolean,
        relatedTaskID: number | null
    ): Promise<HistoryListItem> {
        const body = {
            userID,
            title,
            relatedTaskID,
            private: priv
        };

        let res: AxiosResponse;
        try {
            res = await axios.post("/api/history", body);
        } catch (e) {
            throw e;
        }
        return res.data;
    }

    static async login(email: string, password: string) {
        const body = {
            email,
            password
        };

        let res: AxiosResponse;
        try {
            res = await axios.post("/auth/login", body);
        } catch (e) {
            throw e;
        }
        return res.data;
    }

    static async logout() {
        try {
            await axios.post("/auth/logout");
        } catch (e) {
            throw e;
        }
    }

    static async register(info: RegistrationInfo): Promise<string> {
        let res: AxiosResponse;
        try {
            res = await axios.post("/auth/register", info);
        } catch (e) {
            throw e;
        }

        return res.data;
    }

    static async isAuthenticated(): Promise<boolean> {
        let res: AxiosResponse;
        try {
            res = await axios.post("/auth/verify");
        } catch (e) {
            return false;
        }

        return res.data || false;
    }
}
