# Agents

## Examples

```
# A program is composed of one or more independent agents.
# Agents move around the board in the direction they are oriented in on cycle
# of ticks, leaving behind a colored wake in their path.
#
# Agents are defined by by using the "agent" keyword followed by a name and
# semicolon.
agent red_agent:
    # Everything in a agent block is a statement.
    #
    # Statements can be labels, variables or commands.
    # 
    # Some variables are special and impact the board.
    # These include color, x and y for cell color or coordinates.
    set color = 16711680
    set x = 40
    set y = 40
    # Other variables can be general purpose.
    set acc = 1
    # Commands control either agent behavior or control flow.
    #
    # face changes orientation of an agent and takes a cardinal direction, like
    # S, E or NW.
    face NW
    # Labels allow defining locations in code for a program to jump to
    loop:
        # Move takes a positive 32 bit integer and controls the number of steps
        # to move in a tick in this case our agent will move one space per tick
        # in the direction they are facing.
        move 1
        # Agent scripts support branching through the "jump to" command.
        # These commands look like:
        # "jump to <label> if <expression> is <expresion>".
        jump to spin if acc is 4
        # Variables also support basic arithmetic expressions.
        set acc = acc + 1
        # The goto command allows for the basic jumping to a label without a
        # condition.
        goto loop
    spin:
        # The turn command allows spinning an agent based on their current 
        # orientation. With a positive number being a clockwise rotation and a
        # negative being counter-clockwise.
        turn 1
        set acc = 1
        goto loop
agent blue_agent:
    set color = 255
    set x = 40
    set y = 40
    face NE
    loop:
        move 2
        goto loop
```

## Referenced Links and Credits

- Frontend and rendering code heavily referenced examples from https://github.com/rustwasm/wasm_game_of_life/
