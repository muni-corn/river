import axios from "axios";
import { HistoryListItem } from '@/models/HistoryListItem';

export default class HttpService {
    static async test(): Promise<string> {
        const res = await axios.get('/api/test');
        console.log(res.data);
        return res.data;
    }

    static async pushHistory(userID: number, title: string, priv: boolean, relatedTaskID: number): Promise<HistoryListItem> {
        const body = {
            userID,
            title,
            relatedTaskID,
            "private": priv
        };
        const res = await axios.post('/api/history', body)
        return res.data;
    }
}
