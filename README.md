# Agents

Some rendering code copied from https://github.com/rustwasm/wasm_game_of_life/.

## Examples

```
agent red_agent:
    set color = 0xff0000
    set x = 0
    set y = 0
    set direction = 0
    set a = 0
    loop:
        set a = a + 1
        move 10
        jump to set_zero if a is 1
        jump to set_one if a is 0
        goto exit
    set_one:
        set a 1
        face NW
        goto loop
    set_zero:
        set a 0
        turn -2
        goto loop
    exit:

agent blue_agent:
    set color = 0x00ff00
    set x = 0
    set y = 0
    set direction = 0
    set a = 0
    loop:
        set a = a + 1
        move 10
        jump to set_zero if a is 1
        jump to set_one if a is 0
        goto exit
    set_one:
        set a 1
        face NW
        goto loop
    set_zero:
        set a 0
        turn -2
        goto loop
    exit:
```