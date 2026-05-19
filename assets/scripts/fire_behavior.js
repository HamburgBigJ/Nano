function fireBehavior(x, y, surrounding) {
    if (World.isEmpty(x, y - 1)) {
        World.swap(x, y, x, y - 1);
    } else {
        let dx = Math.random() > 0.5 ? 1 : -1;
        if (World.isEmpty(x + dx, y)) {
            World.swap(x, y, x + dx, y);
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
