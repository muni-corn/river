import { Task } from "./Task";

export interface HistoryListItem {
    at: Date;
    title: string;
    priv?: boolean;
    relatedTask?: Task;
}
