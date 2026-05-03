function fireBehavior(x, y, surrounding) {
    if (World.isEmpty(x, y - 1)) {
        World.swap(x, y, x, y - 1);
    } else {
        let dx = Math.random() > 0.5 ? 1 : -1;
        if (World.isEmpty(x + dx, y)) {
            World.swap(x, y, x + dx, y);
        }
    }

    for (let i = 0; i < surrounding.length; i++) {
        let cell = surrounding[i];
        if (cell && cell.name === "Sand") {
            print("Fire found sand at " + cell.x + "," + cell.y);
        }
    }
}

function steamBehavior(x, y, surrounding) {
    if (World.isEmpty(x, y - 1)) {
        World.swap(x, y, x, y - 1);
    } else if (World.isEmpty(x, y - 2)) {
        World.swap(x, y, x, y - 2);
    }
}

print("build in loaded");
