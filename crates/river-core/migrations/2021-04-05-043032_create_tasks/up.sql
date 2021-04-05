create table if not exists tasks (
	id					serial primary key not null,
	title 				text not null,
	date_added			timestamptz not null,
	started				boolean not null,
	percent_complete	real,
	date_completed		timestamp with time zone
);

-- vim: set ft=pgsql:
