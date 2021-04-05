create table if not exists users (
	id						serial primary key not null,

	-- if both current_task and away_reason are null, the user isn't present
	current_task			integer, -- foreign key
	current_away_reason		text,

	constraint fk_current_task foreign key (current_task) references tasks(id)
);

-- vim: set ft=pgsql:
