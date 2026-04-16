Nano = Nano or {}

Nano.Modules = {}

function Nano.registerModule(name, module)
    Nano.Modules[name] = module
    print("[Lua] Modul registriert: " .. name)
end