alter table tasks
add column creator serial
constraint fk_task_creator references users(id);

-- vim: set ft=pgsql:
