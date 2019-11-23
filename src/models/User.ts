import { Task } from "./Task";

export interface User {
    email: string;
    firstName: string;
    lastName: string;
    displayName: string;
    currentTaskID: number;
    awayReason: string;
}
