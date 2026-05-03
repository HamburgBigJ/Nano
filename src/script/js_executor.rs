// ai helped - Claude
use crate::elements::elements::{BehaviorScript, Element, ElementKind, ElementRegistry};
use crate::world::SandWorld;
use boa_engine::js_string;
use boa_engine::{
    Context, JsObject, JsResult, JsString, JsValue, NativeFunction, Source,
    object::ObjectInitializer,
};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static js_context: RefCell<Option<Context>> = RefCell::new(None);
    static current_registry: RefCell<Option<*mut ElementRegistry>> = RefCell::new(None);
    static BEHAVIOR_FUNCTIONS: RefCell<HashMap<String, JsObject>> = RefCell::new(HashMap::new());
}

pub struct js_executor;

impl js_executor {
    pub fn initialize() {
        js_context.with(|ctx| {
            let mut context = Context::default();
            Self::setup_api(&mut context);
            *ctx.borrow_mut() = Some(context);
        });
    }

    fn setup_api(context: &mut Context) {
        let world_obj = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::get_cell),
                js_string!("getCell"),
                2,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::set_cell),
                js_string!("setCell"),
                3,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::is_empty),
                js_string!("isEmpty"),
                2,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::swap),
                js_string!("swap"),
                4,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::get_element_name),
                js_string!("getElementName"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::get_element_kind),
                js_string!("getElementKind"),
                1,
            )
            .build();

        context
            .register_global_property(
                js_string!("World"),
                world_obj,
                boa_engine::property::Attribute::all(),
            )
            .expect("Failed to register World API");

        let nano_obj = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(Self::register_element),
                js_string!("registerElement"),
                1,
            )
            .function(
                NativeFunction::from_fn_ptr(Self::get_element_id_by_name),
                js_string!("getElementIdByName"),
                1,
            )
            .build();

        context
            .register_global_property(
                js_string!("Nano"),
                nano_obj,
                boa_engine::property::Attribute::all(),
            )
            .expect("Failed to register Nano API");

        context
            .register_global_builtin_callable(
                js_string!("print"),
                1,
                NativeFunction::from_fn_ptr(Self::print),
            )
            .expect("Failed to register print");
    }

    pub fn execute_behavior(
        function_name: &str,
        x: u32,
        y: u32,
        world: &mut SandWorld,
        registry: &ElementRegistry,
    ) {
        current_world.with(|w| *w.borrow_mut() = Some(world as *mut SandWorld));
        current_registry_const.with(|r| *r.borrow_mut() = Some(registry as *const ElementRegistry));

        js_context.with(|ctx| {
            if let Some(context) = ctx.borrow_mut().as_mut() {
                let surrounding = Self::get_surrounding_info(x, y, world, registry, context);

                match Self::get_behavior_function(function_name, context) {
                    Ok(Some(function)) => {
                        let args = [JsValue::from(x), JsValue::from(y), surrounding];
                        if let Err(e) = function.call(&JsValue::undefined(), &args, context) {
                            eprintln!("JS behavior error in {}: {}", function_name, e);
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        eprintln!("JS behavior error in {}: {}", function_name, e);
                    }
                }
            }
        });

        current_world.with(|w| *w.borrow_mut() = None);
        current_registry_const.with(|r| *r.borrow_mut() = None);
    }

    fn get_surrounding_info(
        x: u32,
        y: u32,
        world: &SandWorld,
        registry: &ElementRegistry,
        context: &mut Context,
    ) -> JsValue {
        let mut cells = Vec::with_capacity(9);
        
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < world.width as i32 && ny >= 0 && ny < world.height as i32 {
                    let cell_id = world.get_cell(nx as u32, ny as u32);
                    let element = registry.elements.get(cell_id as usize);
                    let name = element.map(|e| e.name.as_str()).unwrap_or_default();
                    let kind = element
                        .map(|e| Self::element_kind_name(e.kind))
                        .unwrap_or_default();

                    let cell_obj = ObjectInitializer::new(context)
                        .property(
                            js_string!("x"),
                            JsValue::from(nx),
                            boa_engine::property::Attribute::all(),
                        )
                        .property(
                            js_string!("y"),
                            JsValue::from(ny),
                            boa_engine::property::Attribute::all(),
                        )
                        .property(
                            js_string!("id"),
                            JsValue::from(cell_id),
                            boa_engine::property::Attribute::all(),
                        )
                        .property(
                            js_string!("isEmpty"),
                            JsValue::from(cell_id == 0),
                            boa_engine::property::Attribute::all(),
                        )
                        .property(
                            js_string!("name"),
                            JsValue::from(JsString::from(name)),
                            boa_engine::property::Attribute::all(),
                        )
                        .property(
                            js_string!("kind"),
                            JsValue::from(JsString::from(kind)),
                            boa_engine::property::Attribute::all(),
                        )
                        .build();

                    cells.push(cell_obj.into());
                } else {
                    cells.push(JsValue::null());
                }
            }
        }
        
        boa_engine::object::builtins::JsArray::from_iter(cells, context).into()
    }

    fn get_behavior_function(
        function_name: &str,
        context: &mut Context,
    ) -> JsResult<Option<JsObject>> {
        BEHAVIOR_FUNCTIONS.with(|cache| {
            if let Some(function) = cache.borrow().get(function_name).cloned() {
                return Ok(Some(function));
            }

            let value = context
                .global_object()
                .get(js_string!(function_name), context)?;
            let Some(function) = value.as_callable() else {
                return Ok(None);
            };

            cache
                .borrow_mut()
                .insert(function_name.to_string(), function.clone());
            Ok(Some(function))
        })
    }

    fn element_kind_name(kind: ElementKind) -> &'static str {
        match kind {
            ElementKind::Static => "Static",
            ElementKind::Powder => "Powder",
            ElementKind::Liquid => "Liquid",
            ElementKind::Gas => "Gas",
        }
    }

    pub fn register_function(code: &str, registry: &mut ElementRegistry) -> Result<String, String> {
        Self::register_function_with_world(code, registry, None)
    }

    pub fn register_function_with_world(
        code: &str,
        registry: &mut ElementRegistry,
        world: Option<&mut SandWorld>,
    ) -> Result<String, String> {
        BEHAVIOR_FUNCTIONS.with(|cache| cache.borrow_mut().clear());
        current_registry.with(|r| *r.borrow_mut() = Some(registry as *mut ElementRegistry));
        current_registry_const.with(|r| {
            *r.borrow_mut() = Some(registry as *const ElementRegistry);
        });

        if let Some(world) = world {
            current_world.with(|w| *w.borrow_mut() = Some(world as *mut SandWorld));
        }

        js_context.with(|ctx| {
            if let Some(context) = ctx.borrow_mut().as_mut() {
                context
                    .eval(Source::from_bytes(code))
                    .map_err(|e| format!("Failed to register JS function: {}", e))?;
                Ok("Function registered".to_string())
            } else {
                Err("JS context not initialized".to_string())
            }
        })?;

        current_registry.with(|r| *r.borrow_mut() = None);
        current_registry_const.with(|r| *r.borrow_mut() = None);
        current_world.with(|w| *w.borrow_mut() = None);
        Ok("OK".to_string())
    }

    fn register_element(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let config = args.get(0).ok_or_else(|| {
            boa_engine::JsError::from_native(
                boa_engine::JsNativeError::typ().with_message("Missing element config"),
            )
        })?;

        let config_obj = config.as_object().ok_or_else(|| {
            boa_engine::JsError::from_native(
                boa_engine::JsNativeError::typ().with_message("Config must be object"),
            )
        })?;

        let name_val = config_obj.get(js_string!("name"), context)?;
        let name = name_val
            .as_string()
            .ok_or_else(|| {
                boa_engine::JsError::from_native(
                    boa_engine::JsNativeError::typ().with_message("Missing name"),
                )
            })?
            .to_std_string_escaped();

        let color_val = config_obj.get(js_string!("color"), context)?;
        let mut color = [1.0, 1.0, 1.0, 1.0];
        if let Some(arr) = color_val.as_object() {
            for i in 0..4 {
                if let Ok(val) = arr.get(i, context) {
                    if let Some(num) = val.as_number() {
                        color[i] = num as f32;
                    }
                }
            }
        }

        let kind_val = config_obj.get(js_string!("kind"), context)?;
        let kind_str = kind_val
            .as_string()
            .map(|s| s.to_std_string_escaped())
            .unwrap_or_default();

        let kind = match kind_str.as_str() {
            "Powder" => ElementKind::Powder,
            "Liquid" => ElementKind::Liquid,
            "Gas" => ElementKind::Gas,
            _ => ElementKind::Static,
        };

        let behavior_val = config_obj.get(js_string!("behavior"), context)?;
        let behavior_fn = behavior_val
            .as_string()
            .map(|s| s.to_std_string_escaped())
            .unwrap_or_default();

        let element = Element {
            name: name.clone(),
            color,
            kind,
            behavior: if behavior_fn.is_empty() {
                BehaviorScript::None
            } else {
                BehaviorScript::JavaScript(behavior_fn)
            },
        };

        let mut registered_id = None;
        current_registry.with(|r| {
            if let Some(registry_ptr) = *r.borrow() {
                let registry = unsafe { &mut *registry_ptr };
                let id = registry.register(element);
                registered_id = Some(id);
                println!("Registered element '{}' with id {}", name, id);
            }
        });

        Ok(registered_id
            .map(JsValue::from)
            .unwrap_or_else(JsValue::undefined))
    }

    fn get_cell(_this: &JsValue, args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some((x, y)) = Self::read_xy(args) else {
            return Ok(JsValue::from(0));
        };

        current_world.with(|w| {
            if let Some(world_ptr) = *w.borrow() {
                let world = unsafe { &*world_ptr };
                Ok(JsValue::from(world.get_cell(x, y)))
            } else {
                Ok(JsValue::from(0))
            }
        })
    }

    fn set_cell(_this: &JsValue, args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some((x, y)) = Self::read_xy(args) else {
            return Ok(JsValue::undefined());
        };
        let id = args.get(2).and_then(|v| v.as_number()).unwrap_or(0.0) as u8;

        current_world.with(|w| {
            if let Some(world_ptr) = *w.borrow() {
                let world = unsafe { &mut *world_ptr };
                world.set_cell(x, y, id);
            }
        });

        Ok(JsValue::undefined())
    }

    fn is_empty(_this: &JsValue, args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some((x, y)) = Self::read_xy(args) else {
            return Ok(JsValue::from(false));
        };

        current_world.with(|w| {
            if let Some(world_ptr) = *w.borrow() {
                let world = unsafe { &*world_ptr };
                Ok(JsValue::from(world.is_empty(x, y)))
            } else {
                Ok(JsValue::from(true))
            }
        })
    }

    fn swap(_this: &JsValue, args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let Some((x1, y1)) = Self::read_xy(args) else {
            return Ok(JsValue::undefined());
        };
        let Some((x2, y2)) = Self::read_xy(&args[2..]) else {
            return Ok(JsValue::undefined());
        };

        current_world.with(|w| {
            if let Some(world_ptr) = *w.borrow() {
                let world = unsafe { &mut *world_ptr };
                world.swap_cells(x1, y1, x2, y2);
            }
        });

        Ok(JsValue::undefined())
    }

    fn read_xy(args: &[JsValue]) -> Option<(u32, u32)> {
        let x = Self::read_coord(args.get(0)?)?;
        let y = Self::read_coord(args.get(1)?)?;
        Some((x, y))
    }

    fn read_coord(value: &JsValue) -> Option<u32> {
        let coord = value.as_number()?;
        if coord.is_finite() && coord >= 0.0 {
            Some(coord as u32)
        } else {
            None
        }
    }

    fn get_element_name(
        _this: &JsValue,
        args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<JsValue> {
        let id = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0) as u8;

        current_registry_const.with(|r| {
            if let Some(registry_ptr) = *r.borrow() {
                let registry = unsafe { &*registry_ptr };
                let name = registry
                    .elements
                    .get(id as usize)
                    .map(|e| e.name.clone())
                    .unwrap_or_default();
                Ok(JsValue::from(JsString::from(name)))
            } else {
                Ok(JsValue::from(JsString::from("")))
            }
        })
    }

    fn get_element_kind(
        _this: &JsValue,
        args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<JsValue> {
        let id = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0) as u8;

        current_registry_const.with(|r| {
            if let Some(registry_ptr) = *r.borrow() {
                let registry = unsafe { &*registry_ptr };
                let kind = registry
                    .elements
                    .get(id as usize)
                    .map(|e| Self::element_kind_name(e.kind))
                    .unwrap_or_default();
                Ok(JsValue::from(JsString::from(kind)))
            } else {
                Ok(JsValue::from(JsString::from("")))
            }
        })
    }

    fn get_element_id_by_name(
        _this: &JsValue,
        args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<JsValue> {
        let name = args
            .get(0)
            .and_then(|v| v.as_string())
            .map(|s| s.to_std_string_escaped())
            .unwrap_or_default();

        current_registry_const.with(|r| {
            if let Some(registry_ptr) = *r.borrow() {
                let registry = unsafe { &*registry_ptr };
                let id = registry
                    .elements
                    .iter()
                    .position(|element| element.name == name)
                    .map(|id| id as i32)
                    .unwrap_or(-1);
                Ok(JsValue::from(id))
            } else {
                Ok(JsValue::from(-1))
            }
        })
    }

    fn print(_this: &JsValue, args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let msg = args
            .get(0)
            .and_then(|v| v.as_string())
            .map(|s| s.to_std_string_escaped())
            .unwrap_or_default();
        println!("[JS] {}", msg);
        Ok(JsValue::undefined())
    }
}

thread_local! {
    static current_world: RefCell<Option<*mut SandWorld>> = RefCell::new(None);
    static current_registry_const: RefCell<Option<*const ElementRegistry>> = RefCell::new(None);
}
