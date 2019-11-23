import { Task } from "./Task";

export interface HistoryListItem {
    at: Date;
    title: string;
    private: boolean;
    relatedTask: Task;
}
