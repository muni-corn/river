import { User } from "./User";

export interface Task {
    owner: User;
    name: string;
    percentComplete: number; // between 0 and 1
    minutesSpent: number;
    minutesRemaining: number;
    completionDate: Date;
    creationDate: Date;
}
