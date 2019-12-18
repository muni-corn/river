export enum StoreActions {
    HistoryPush = "historyPush",
    GetUserInformation = "getUserInformation",
    NewTask = "NewTask",
    Start = "start"
}

export enum StoreMutations {
    HistoryPush = "historyPush",
    SetUserName = "setUserName",
    SetTodo = "setTodo",
    SetHistory = "setHistory",
    SetCurrentTask = "setCurrentTask",
    PushBusy = "pushBusy",
    PopBusy = "popBusy",
    PushTask = "pushTask"
}
