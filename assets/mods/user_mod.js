function waterBehavior(x, y, surrounding) {
    if (World.isEmpty(x, y + 1)) {
        World.swap(x, y, x, y + 1);
        return;
    }

    let dir = Math.random() > 0.5 ? 1 : -1;
    if (World.isEmpty(x + dir, y)) {
        World.swap(x, y, x + dir, y);
    } else if (World.isEmpty(x - dir, y)) {
        World.swap(x, y, x - dir, y);
    }

    for (let i = 0; i < surrounding.length; i++) {
        let cell = surrounding[i];
        if (cell && cell.name === "Fire") {
           World.swap(x, y, cell.x, cell.y);
           World.setCell(x, y, Nano.getElementIdByName("Empty"))
        }
    }
}

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

Nano.registerElement({
    name: "Water",
    color: [0.2, 0.4, 1.0, 1.0],
    kind: "Liquid",
    behavior: "waterBehavior"
});

Nano.registerElement({
    name: "Explosive",
    color: [1.0, 0.5, 0.0, 1.0],
    kind: "Powder",
    behavior: "explosiveBehavior"
});

print("mod loaded");
