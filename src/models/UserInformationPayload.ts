import { Task } from "@/models/Task";
import { HistoryListItem } from "@/models/HistoryListItem";

export interface UserInformationPayload {
    userName: string;
    currentTaskID: number;
    awayReason: string;
    todo: Task[];
    history: HistoryListItem[];
}
