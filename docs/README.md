# Nano Modding Guide

Nano mods are JavaScript files that can register new elements and add custom
per-cell behavior to the powder simulation.

## Where Mods Go

Put external mod files in this directory:

```text
mods/
```

Every `*.js` file in this directory is loaded at runtime on native desktop
builds. Files are loaded in sorted filename order after the embedded scripts and
embedded mods.

External `mods/*.js` files are not loaded in the wasm/web build. The web build
only uses JavaScript files embedded by the Rust asset loader.

## Basic Mod

A mod usually has two parts:

1. A behavior function.
2. A call to `Nano.registerElement`.

```js
function oilBehavior(x, y, surrounding) {
    if (World.isEmpty(x, y + 1)) {
        World.swap(x, y, x, y + 1);
        return;
    }

    let dir = Math.random() > 0.5 ? 1 : -1;
    if (World.isEmpty(x + dir, y)) {
        World.swap(x, y, x + dir, y);
    }
}

const oilId = Nano.registerElement({
    name: "Oil",
    color: [0.12, 0.10, 0.04, 1.0],
    kind: "Liquid",
    behavior: "oilBehavior"
});

print("Oil element id is " + oilId);
```

## Registering Elements

Use `Nano.registerElement(config)` to add a new element:

```js
const elementId = Nano.registerElement({
    name: "Steam",
    color: [0.8, 0.8, 0.8, 0.6],
    kind: "Gas",
    behavior: "steamBehavior"
});
```

The config fields are:

| Field | Type | Description |
| --- | --- | --- |
| `name` | string | Display/internal element name. Names are case-sensitive. |
| `color` | array | RGBA color as four numbers from `0.0` to `1.0`. |
| `kind` | string | One of `"Static"`, `"Powder"`, `"Liquid"`, or `"Gas"`. Unknown values become `"Static"`. |
| `behavior` | string | Name of a global JavaScript function to run for this element. Use `""` or omit it for no custom behavior. |

`Nano.registerElement` returns the numeric element id. Element id `0` is
`Empty`.

## Getting Element Ids By Name

Use `Nano.getElementIdByName(name)` when a mod needs to place or compare an
element but does not know its numeric id:

```js
const fireId = Nano.getElementIdByName("Fire");
const waterId = Nano.getElementIdByName("Water");

if (fireId !== -1) {
    World.setCell(150, 50, fireId);
}
```

The lookup is case-sensitive. It returns `-1` when no element with that name is
registered yet. Because mods load in sorted filename order, put shared elements
in an earlier filename if another mod needs to look them up.

You can also inspect an id:

```js
let id = World.getCell(x, y);
let name = World.getElementName(id);
let kind = World.getElementKind(id);
```

## Behavior Functions

A behavior function is called once per simulation update for every cell that has
that behavior.

```js
function myBehavior(x, y, surrounding) {
    // x and y are the current cell coordinates.
    // surrounding contains the 3x3 area around the cell.
}
```

The simulation first applies the built-in movement for `"Powder"` and
`"Liquid"` elements, then calls the custom behavior. `"Static"` and `"Gas"` do
not get built-in movement, so their movement must be fully implemented in the
behavior function.

`surrounding` is an array with 9 entries for the 3x3 area centered on the cell.
Out-of-bounds entries are `null`. Cell entries have this shape:

```js
{
    x: 10,
    y: 20,
    id: 1,
    isEmpty: false,
    name: "Sand",
    kind: "Powder"
}
```

Example behavior that reacts to nearby fire:

```js
function explosiveBehavior(x, y, surrounding) {
    for (let i = 0; i < surrounding.length; i++) {
        let cell = surrounding[i];
        if (cell && cell.name === "Fire") {
            for (let dx = -2; dx <= 2; dx++) {
                for (let dy = -2; dy <= 2; dy++) {
                    World.setCell(x + dx, y + dy, 0);
                }
            }
            return;
        }
    }

    if (World.isEmpty(x, y + 1)) {
        World.swap(x, y, x, y + 1);
    }
}
```

## World API

The JavaScript API exposes these helpers:

| Function | Description |
| --- | --- |
| `World.getCell(x, y)` | Returns the element id at a cell. Out-of-bounds reads return `0`. |
| `World.setCell(x, y, id)` | Sets a cell to an element id. Out-of-bounds writes are ignored. |
| `World.isEmpty(x, y)` | Returns `true` when the cell contains id `0`. |
| `World.swap(x1, y1, x2, y2)` | Swaps two cells. Out-of-bounds swaps are ignored. |
| `World.getElementName(id)` | Returns the element name for an id, or `""` when missing. |
| `World.getElementKind(id)` | Returns `"Static"`, `"Powder"`, `"Liquid"`, `"Gas"`, or `""` when missing. |
| `Nano.registerElement(config)` | Registers an element and returns its numeric id. |
| `Nano.getElementIdByName(name)` | Returns an element id by name, or `-1` when missing. |
| `print(message)` | Prints a message with a `[JS]` prefix. |

## Complete Example

Save this as `mods/10_spark.js`:

```js
const fireId = Nano.getElementIdByName("Fire");

function sparkBehavior(x, y, surrounding) {
    for (let i = 0; i < surrounding.length; i++) {
        let cell = surrounding[i];
        if (cell && cell.name === "Sand" && fireId !== -1) {
            World.setCell(cell.x, cell.y, fireId);
        }
    }

    if (World.isEmpty(x, y - 1)) {
        World.swap(x, y, x, y - 1);
    } else {
        World.setCell(x, y, 0);
    }
}

const sparkId = Nano.registerElement({
    name: "Spark",
    color: [1.0, 1.0, 0.3, 1.0],
    kind: "Gas",
    behavior: "sparkBehavior"
});

for (let i = 0; i < 20; i++) {
    World.setCell(120 + i, 80, sparkId);
}
```

## Notes

- Coordinates are unsigned grid positions. Negative coordinates are ignored by
  write helpers and usually read as empty.
- Keep behavior functions small. They run for every matching cell on every
  simulation update.
- Element names and behavior function names are global. Use unique names to
  avoid collisions with other mods.
