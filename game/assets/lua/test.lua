local Test = {}

function Test.onStart()
    print("hello")
end


function Test.onStop()
    print("hello")
    info("hello")
end


function Test.onUpdate(dt)

end

Nano.registerModule("Test", Test)