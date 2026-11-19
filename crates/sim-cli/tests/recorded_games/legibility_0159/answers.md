# The readers' answers

Six readers, each a small model (Haiku), each given one plan order as a seat's `order` tool answers and nothing else, and the four questions of the key. Run on Nov 19, 2026, after the proof's run. The answers are as given.

## Order 1: scenario 80, morning of day 2

1. The plan has three phases, in order: Prepare, then Move (ends when the first man is ashore at hex (4,0)), then Land (ends when hex (4,0) is the side's). Its mission is to take the place at (4,0).
2. No phase is under way. All three are marked "to come", and the plan is still a draft (number 1, state "draft"), so Prepare is the next phase but has not started. It ends only when the plan is released.
3. The plan is waiting for men. It wants 1,200 and has 600 to hand, so the condition is not met. The file expects it to be met on day 4.
4. It expects to be ashore on day 8. The file gives that day explicitly as "expected_ashore": 8.

## Order 2: scenario 81, main game, morning of day 3

1. The plan has three phases, in order: Prepare, then Move, then Land. Prepare ends when the plan is released, Move ends when the first man is ashore at (4,0), and Land ends when (4,0) is the side's.
2. Prepare is under way now. Move and Land are to come.
3. The plan waits for a day: not before day 6, not yet met, and expected on day 6.
4. The plan expects to be ashore on day 9.

## Order 3: scenario 81, wait for a force, morning of day 3

1. The plan has three phases, in order: Prepare, then Move, then Land. Prepare ends when the plan is released. Move ends when the first man is ashore at (4,0). Land ends when (4,0) is the side's.
2. Prepare is under way now. Move and Land are to come. The plan is still in the preparing state, and its one task is to ready a landing at (4,0) by task_force:1.
3. The plan is waiting for a force: task_force:2. That condition is not met, and the file expects it on day 5. The plan's own waiting line and the force entry both give day 5 as the expected day, and no other day is named.
4. The file gives day 8 as when it expects to be ashore, in both the text and the expected_ashore field. The file does not give a day for the Move or Land phases on their own.

## Order 4: scenario 81, after another place, morning of day 3, intent 1

1. The plan has three phases, in order: Prepare, then Move, then Land. Prepare ends when the plan is released, Move ends when the first man is ashore at (4,0), and Land ends when (4,0) is the side's.
2. Prepare is under way now. Move and Land are still to come.
3. The plan is waiting for an intent: intent 3 has to come first, and it is not yet met. The file names no day for this (the waits_for day and expected_day are both empty).
4. The file gives no day. expected_ashore is null, and the plan says it cannot yet say when it will be ashore.

## Order 5: scenario 82, morning of day 3

1. The plan has three phases, in order: Prepare, then Move (ends when the first man is ashore at (4,0)), then Land (ends when (4,0) is the side's).
2. Prepare is under way now. It ends when the plan is released, and its one task is to ready a landing at (4,0) by task_force:1.
3. The plan is waiting for knowledge: a count of the enemy no more than 1 day old, which is not yet met. The file gives no day for this wait (the day field is null). The count it holds is 150 men, seen from the air on day 1, which is 2 days old, so it does not qualify.
4. The file gives no day for landing. The expected_ashore field is null, and the plan itself says it cannot yet say when it will be ashore.

## Order 6: scenario 79, morning of day 3 (a hold)

1. The plan has two phases, in order: Wait, then Hold. Wait ends when base:2 (Placeholder airfield B) is the side's. Hold does not end.
2. The Wait phase is under way now. Hold is still to come.
3. The plan waits for nothing: its waits-for list is empty, and the file says "It waits for nothing." It names no day. The only condition is that Wait ends when base:2 becomes the side's, which is how that phase ends, not something it is waiting for.
4. The file gives no day for arriving ashore. The expected-ashore field is null, and it names no men or landing either. It does say the plan is for day 3.
