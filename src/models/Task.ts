import { User } from "./User";

export interface Task {
    id: number;
    name: string;
    percentComplete?: number; // between 0 and 1
    minutesSpent?: number;
    wasCompletedAt?: Date;
    creationDate: Date;
    priv: boolean;
}
