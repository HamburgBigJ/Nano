const fire = Nano.getElementIdByName("Fire");

const testElement = Nano.registerElement({
    name: "test",
    color: [0.4, 0.3, 0.5, 1.0],
    kind: "Static",
    behavior: "test_behavior"
})

function test_behavior(x, y, surrounding) {

}


