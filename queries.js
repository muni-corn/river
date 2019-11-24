function makeStartQuery(newTaskID, userID) {
    return {
        name: 'start',
        text: 'UPDATE "public.user" SET current_task = $1, away_reason = NULL WHERE id = $2',
        values: [newTaskID, userID]
    };
}

function makeStopQuery(reason, userID) {
    return {
        name: 'start',
        text: 'UPDATE "public.user" SET current_task = NULL, away_reason = $1 WHERE id = $2',
        values: [reason, userID]
    };
}

function makePushHistoryQuery(userID, actionName, private, relatedTaskID) {
    return {
        name: 'pushHistory',
        text: 'INSERT INTO "public.history" ' + 
            '("user", action, time, private, related_task) ' +
            'VALUES ' +
            '($1, $2, NOW(), $3, $4) RETURNING *',
        values: [userID, actionName, private, relatedTaskID]
    };
}

function makeNewTaskQuery(taskTitle, private) {
    return {
        name: 'newTask',
        text: 'INSERT INTO "public.task" (name, creation_date, private) VALUES ' +
            '($1, NOW(), $2) RETURNING *',
        values: [taskTitle, private]
    }
}

function makeLinkTaskOwnerQuery(userID, taskID) {
    return {
        name: "linkTaskOwner",
        text: 'INSERT INTO "public.task_owners" ("owner", "task") VALUES ' +
            '($1, $2)',
        values: [userID, taskID]
    }
}

function makeGetHistoryQuery(userID, offset, limit) {
    return {
        name: "getHistory",
        text: 'SELECT * FROM "public.history" WHERE "user" = $1 OFFSET $2 LIMIT $3',
        values: [userID, offset, limit]
    }
}

function makeUpdateTaskQuery(taskID, name, percentComplete, minutesSpent, wasCompletedAt, private) {
    return {
        name: "updateTask",
        text: 'UPDATE "public.task" SET ' +
            '"name" = $1,' +
            '"percent_complete" = $2,' +
            '"minutes_spent" = $3,' +
            '"was_completed_at" = $4,' +
            '"private" = $5 ' +
            'WHERE "id" = $6 RETURNING *',
        values: [name, percentComplete, minutesSpent, wasCompletedAt, private, taskID]
    }
}

function makeUpdateHistoryItemQuery(historyID, private) {
    return {
        name: "updateHistoryItem",
        text: 'UPDATE "public.history" SET "private" = $1 WHERE "id" = $2',
        values: [private, historyID]
    };
}

function makeGetTasksQuery(userID) {
    return {
        name: "getTasks",
        text: 'SELECT * FROM "public.task" WHERE "owner" = $1',
        values: [userID]
    }
}

function makeGetSingleTaskQuery(taskID) {
    return {
        name: "getTasks",
        text: 'SELECT * FROM "public.task" WHERE "id" = $1',
        values: [taskID]
    }
}

module.exports = {
    makeStartQuery,
    makeStopQuery,
    makePushHistoryQuery,
    makeNewTaskQuery,
    makeLinkTaskOwnerQuery,
    makeGetHistoryQuery,
    makeUpdateTaskQuery,
    makeUpdateHistoryItemQuery,
    makeGetTasksQuery,
    makeGetSingleTaskQuery
};