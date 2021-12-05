# Agents

Some rendering code copied from https://github.com/rustwasm/wasm_game_of_life/.

## Examples

```
agent red_agent:
    set color = 255
    set x = 0
    set y = 0
    set direction = 0
    set a = 0
    loop:
        face NW
        move 10
        turn -4
        goto loop
        set a = 5
        jump to exit if a is 1
    exit:
agent blue_agent:
    set color = 255
    set x = 0
    set y = 0
    set direction = 0
    set a = 0
    loop:
        face NE
        move 20
        turn -30
        goto loop
        set b = 5
    exit:
```