---
title: Software Requirements Specification
subtitle: River
author: Harrison Thorne
...

Introduction
============================================================

River is a way for teams to keep track of their tasks and time. Get projects
done better than ever by knowing what everybody's working on.

River is designed for teams. Teams can keep track of their project's progress
by knowing how far along members are on their tasks, what they have left to do,
and how long they estimate their tasks will take. Users can supply a percentage
of completion for a task as well as a "time remaining" estimate. Other users
part of the same team can see these estimates, as well as the name (and
description, checklists, more) of tasks.

River keeps track of the users' past, present, and future for their team to
view. If a user goes out to lunch (or decides to take a power nap), they can
set their "present status" as such. If they have more than one task on their
plate, they can maintain a to-do list. If they need to remember what they
worked on a few days ago, River maintains a history of their past tasks. If
users are a little more concerned about their privacy, they have the option to
remove tasks from their history, to-do list, or to hide their current activity.

Requirements
============

## "Must have" requirements

-	River *shall*:

	-	display each user's data in a column, known as a
	"River".

		-	Rivers shall consist of three components (displayed from top to
		bottom): history list, status window (displays current status), and
		to-do list. The status window shall be displayed in the exact center of
		the River.

	-	keep and display a user's history of tasks and changes to their status.

		-	The history component shall be visible as the top portion of a
		user's River, above their status window.

	-	allow users to:

		-	update their current status by selecting an "edit" button within
		the status window and typing a new status title.

			-	River shall allow users to further specify whether this new
			status is a task or not.

		-	add tasks to and delete tasks from a to-do list.

		-	set a task as their current working status immediately by selecting
		an item in their to-do list.

		-	delete history items.

		-	update their progress, time spent, and estimated time remaining on
		a task while it is focused in the status window.

			-	This functionality is accessed using the same "edit" button as
			above.

		-	view their teammates' progress.

			-	Rivers shall be displayed side-by-side, with the user's on the
			very left (support for right-to-left locales may be supported later
			on, but will not be required now). River shall provide the ability
			to scroll to the right and view more Rivers, whether through use
			of the user's mouse (or touchpad or touchscreen), a horizontal
			scroll bar, or left- and right-arrow icons displayed on-screen that
			the user can select to scroll left or right, respectively.

	-	automatically keep time on a user's status or task.

## "Stretch" requirements

-	River *will*:

	- allow users to view their teammates' progress *in real-time*

## Main view design

River's main view will look something like this: 

![Main view design](./design.png)\

Where the white, left-most window is the user's current
status, the gray windows along the center are the teammates'
statuses, the black above the center is the users' history, and
the black below the center is users' to-do list.

The right-arrow button on the right allows users to bring
their teammates' Rivers into view (or view more if those
Rivers are already in view).
