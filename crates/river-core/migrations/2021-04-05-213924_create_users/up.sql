create table if not exists users (
	id						serial primary key not null,

	-- if `present` is true:
	-- *	when current_away_reason isn't null, the corresponding enum is
	--		UserStatus::Away(<current_away_reason>)
	-- *	when current_away_reason is null but current_task is not, the
	--		corresponding enum is UserStatus::Working(<current_task>)
	-- *	when both current_task and current_away_reason are null, the
	--		corresponding enum is UserStatus::Working(None)
	-- and, of course, if `present` is false, the enum is UserStatus::Out
	present					boolean not null,
	current_task			integer, -- foreign key
	current_away_reason		text,

	constraint fk_current_task foreign key (current_task) references tasks(id)
);

-- vim: set ft=pgsql:
