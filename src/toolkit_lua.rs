use iced_core::Theme;
use iced_wgpu::{wgpu, Renderer};

macro_rules! lua_wrapper_min {
    ($wrapper: ident, $wrapped: ty) => {
        struct $wrapper($wrapped);
        unsafe impl Send for $wrapper {}
        impl From<$wrapper> for $wrapped {
            fn from(value: $wrapper) -> Self {
                value.0.into()
            }
        }
    };
}

macro_rules! lua_wrapper {
    (clone $wrapper: ident, $wrapped: ty) => {
        lua_wrapper_min!($wrapper, $wrapped);
        impl mlua::FromLua for $wrapper {
            fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
                match value {
                    mlua::Value::UserData(ud) => Ok(Self(ud.borrow::<Self>()?.0.clone())),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: String::from(std::any::type_name::<$wrapper>()),
                        message: None,
                    }),
                }
            }
        }
    };
    ($wrapper: ident, $wrapped: ty) => {
        lua_wrapper_min!($wrapper, $wrapped);
        impl mlua::FromLua for $wrapper {
            fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
                match value {
                    mlua::Value::UserData(ud) => Ok(ud.take::<Self>()?),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: String::from(std::any::type_name::<$wrapper>()),
                        message: None,
                    }),
                }
            }
        }
    };
}

macro_rules! impl_element_for {
    ($($typename:ty),*) => {$(
        impl From<$typename> for iced::Element<'static, Message, Theme, Renderer> {
            fn from(value: $typename) -> Self {
                value.0.into()
            }
        }
    )*}
}

macro_rules! impl_fromlua_for {
  ($($typename:ty),*) => {$(
    impl mlua::FromLua for $typename {
      fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
          mlua::Value::UserData(ud) => Ok(ud.take::<Self>()?),
          _ => unreachable!()
        }
      }
   }
 )*}
}

#[derive(Debug, Clone)]
pub struct Message(mlua::Value);
impl mlua::UserData for Message {}
impl_fromlua_for!(Message);

// Wrapper for Length
lua_wrapper_min!(LuaLength, iced::Length);
impl mlua::UserData for LuaLength {}
impl mlua::FromLua for LuaLength {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Integer(n) => Ok(LuaLength(iced::Length::Fixed(n as f32))),
            mlua::Value::Number(n) => Ok(LuaLength(iced::Length::Fixed(n as f32))),
            mlua::Value::UserData(ud) => Ok(Self(ud.borrow::<Self>()?.0.clone())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaLength"),
                message: None,
            }),
        }
    }
}

// Wrapper for Padding
lua_wrapper_min!(LuaPadding, iced::Padding);
impl mlua::UserData for LuaPadding {}
impl mlua::FromLua for LuaPadding {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Integer(n) => Ok(LuaPadding(iced::Padding::new(n as f32))),
            mlua::Value::Number(n) => Ok(LuaPadding(iced::Padding::new(n as f32))),
            mlua::Value::UserData(ud) => Ok(Self(ud.borrow::<Self>()?.0.clone())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaPadding"),
                message: None,
            }),
        }
    }
}

// Wrapper for Alignment
lua_wrapper!(clone LuaAlignment, iced::alignment::Alignment);
impl mlua::UserData for LuaAlignment {}

// Wrapper for Horizontal
lua_wrapper!(clone LuaHorizontal, iced::alignment::Horizontal);
impl mlua::UserData for LuaHorizontal {}

// Wrapper for Vertical
lua_wrapper!(clone LuaVertical, iced::alignment::Vertical);
impl mlua::UserData for LuaVertical {}

// Wrapper for Pixels
lua_wrapper_min!(LuaPixels, iced::Pixels);
impl mlua::UserData for LuaPixels {}
impl mlua::FromLua for LuaPixels {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Integer(n) => Ok(LuaPixels(iced::Pixels(n as f32))),
            mlua::Value::Number(n) => Ok(LuaPixels(iced::Pixels(n as f32))),
            mlua::Value::UserData(ud) => Ok(Self(ud.borrow::<Self>()?.0.clone())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaPixels"),
                message: None,
            }),
        }
    }
}

// Wrapper for Color
lua_wrapper!(clone LuaColor, iced::Color);
impl mlua::UserData for LuaColor {}

// Wrapper for Border
lua_wrapper!(clone LuaBorder, iced::Border);
impl mlua::UserData for LuaBorder {}

// Wrapper for Radius
lua_wrapper_min!(LuaRadius, iced::border::Radius);
impl mlua::UserData for LuaRadius {}
impl mlua::FromLua for LuaRadius {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Integer(n) => Ok(LuaRadius((n as f32).into())),
            mlua::Value::Number(n) => Ok(LuaRadius((n as f32).into())),
            mlua::Value::UserData(ud) => Ok(Self(ud.borrow::<Self>()?.0.clone())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaRadius"),
                message: None,
            }),
        }
    }
}

// Wrapper for Background
lua_wrapper_min!(LuaBackground, iced::Background);
impl mlua::UserData for LuaBackground {}
impl mlua::FromLua for LuaBackground {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                if ud.is::<LuaColor>() {
                    let col = ud.borrow::<LuaColor>()?;
                    Ok(LuaBackground(iced::Background::Color(col.0)))
                } else {
                    Err(mlua::Error::UserDataTypeMismatch)
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaBackground"),
                message: None,
            }),
        }
    }
}

// Wrapper for Shadow
lua_wrapper!(clone LuaShadow, iced::Shadow);
impl mlua::UserData for LuaShadow {}

// Wrapper for Palette
lua_wrapper!(clone LuaPalette, iced::theme::Palette);
impl mlua::UserData for LuaPalette {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("background", |_, this| Ok(LuaColor(this.0.background)));
        fields.add_field_method_get("text", |_, this| Ok(LuaColor(this.0.text)));
        fields.add_field_method_get("primary", |_, this| Ok(LuaColor(this.0.primary)));
        fields.add_field_method_get("success", |_, this| Ok(LuaColor(this.0.success)));
        fields.add_field_method_get("danger", |_, this| Ok(LuaColor(this.0.danger)));
    }
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("generate", |_lua, this: Self| {
            Ok(LuaExtended(iced::theme::palette::Extended::generate(
                this.0.clone(),
            )))
        });
    }
}

lua_wrapper!(clone LuaPalettePair, iced::theme::palette::Pair);
impl mlua::UserData for LuaPalettePair {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("color", |_, this| Ok(LuaColor(this.0.color)));
        fields.add_field_method_get("text", |_, this| Ok(LuaColor(this.0.text)));
        fields.add_field_method_set("color", |_, this, value: LuaColor| {
            this.0.color = value.0;
            Ok(())
        });
        fields.add_field_method_set("text", |_, this, value: LuaColor| {
            this.0.text = value.0;
            Ok(())
        });
    }
}

// The Extended triplet pair stuff is the same, so macro it
macro_rules! lua_extended {
    ($wrapper: ident, $wrapped: ty) => {
        lua_wrapper!(clone $wrapper, $wrapped);
        impl mlua::UserData for $wrapper {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                fields.add_field_method_get("base", |_, this| Ok(LuaPalettePair(this.0.base)));
                fields.add_field_method_get("weak", |_, this| Ok(LuaPalettePair(this.0.weak)));
                fields.add_field_method_get("strong", |_, this| Ok(LuaPalettePair(this.0.strong)));
            }
        }
    };
}
lua_extended!(LuaExtendedBackground, iced::theme::palette::Background);
lua_extended!(LuaExtendedPrimary, iced::theme::palette::Primary);
lua_extended!(LuaExtendedSecondary, iced::theme::palette::Secondary);
lua_extended!(LuaExtendedSuccess, iced::theme::palette::Success);
lua_extended!(LuaExtendedDanger, iced::theme::palette::Danger);

// Wrapper for Extended
lua_wrapper!(clone LuaExtended, iced::theme::palette::Extended);
impl mlua::UserData for LuaExtended {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("background", |_, this| {
            Ok(LuaExtendedBackground(this.0.background))
        });
        fields.add_field_method_get("primary", |_, this| Ok(LuaExtendedPrimary(this.0.primary)));
        fields.add_field_method_get("secondary", |_, this| {
            Ok(LuaExtendedSecondary(this.0.secondary))
        });
        fields.add_field_method_get("success", |_, this| Ok(LuaExtendedSuccess(this.0.success)));
        fields.add_field_method_get("danger", |_, this| Ok(LuaExtendedDanger(this.0.danger)));
        fields.add_field_method_get("is_dark", |_, this| Ok(this.0.is_dark));
    }
}

// Wrapper for Theme
lua_wrapper!(clone LuaTheme, Theme);
impl mlua::UserData for LuaTheme {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("palette", |_lua, this: Self| {
            Ok(LuaPalette(this.0.palette()))
        });
        methods.add_function_mut("extended_palette", |_lua, this: Self| {
            Ok(LuaExtended(this.0.extended_palette().clone()))
        });
    }
}

// Wrapper for Container Style
lua_wrapper!(LuaContainerStyle, iced::widget::container::Style);
impl mlua::UserData for LuaContainerStyle {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("color", |_lua, (this, val): (Self, LuaColor)| {
            Ok(LuaContainerStyle(this.0.color(val)))
        });
        methods.add_function_mut("border", |_lua, (this, val): (Self, LuaBorder)| {
            Ok(LuaContainerStyle(this.0.border(val)))
        });
        methods.add_function_mut("background", |_lua, (this, val): (Self, LuaBackground)| {
            Ok(LuaContainerStyle(this.0.background(val)))
        });
        methods.add_function_mut("shadow", |_lua, (this, val): (Self, LuaShadow)| {
            Ok(LuaContainerStyle(this.0.shadow(val)))
        });
    }
}

// Element Wrapper
lua_wrapper!(LuaElement, iced::Element<'static, Message, Theme, Renderer>);
impl mlua::UserData for LuaElement {}
impl_element_for!(LuaButton, LuaContainer, LuaColumn);

// Button Wrapper
lua_wrapper!(
    LuaButton,
    iced_widget::Button<'static, Message, Theme, Renderer>
);
impl mlua::UserData for LuaButton {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("on_press", |_lua, (this, val): (Self, mlua::Value)| {
            Ok(LuaButton(this.0.on_press(Message(val))))
        });
        methods.add_function_mut("width", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaButton(this.0.width(val)))
        });
        methods.add_function_mut("height", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaButton(this.0.height(val)))
        });
        methods.add_function_mut("padding", |_lua, (this, val): (Self, LuaPadding)| {
            Ok(LuaButton(this.0.padding(val)))
        });
        methods.add_function_mut("clip", |_lua, (this, val): (Self, mlua::Value)| {
            Ok(LuaButton(this.0.clip(val.as_boolean().unwrap_or(false))))
        });
    }
}

// Column Wrapper
lua_wrapper!(
    LuaColumn,
    iced_widget::Column<'static, Message, Theme, Renderer>
);
impl mlua::UserData for LuaColumn {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("spacing", |_lua, (this, spacing): (Self, LuaPixels)| {
            Ok(LuaColumn(this.0.spacing(spacing)))
        });
        methods.add_function_mut("padding", |_lua, (this, padding): (Self, LuaPadding)| {
            Ok(LuaColumn(this.0.padding(padding)))
        });
        methods.add_function_mut("width", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaColumn(this.0.width(val)))
        });
        methods.add_function_mut("height", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaColumn(this.0.height(val)))
        });
        methods.add_function_mut("max_width", |_lua, (this, val): (Self, LuaPixels)| {
            Ok(LuaColumn(this.0.max_width(val)))
        });
        methods.add_function_mut("align_x", |_lua, (this, val): (Self, LuaAlignment)| {
            Ok(LuaColumn(
                this.0.align_x(iced::alignment::Horizontal::from(val.0)),
            ))
        });
        methods.add_function_mut("clip", |_lua, (this, val): (Self, mlua::Value)| {
            Ok(LuaColumn(this.0.clip(val.as_boolean().unwrap_or(false))))
        });
        methods.add_function_mut("push", |_lua, (this, val): (Self, mlua::Value)| {
            Ok(LuaColumn(this.0.push(value_to_element(val)?)))
        });
    }
}

// Container Wrapper
lua_wrapper!(
    LuaContainer,
    iced_widget::Container<'static, Message, Theme, Renderer>
);
impl mlua::UserData for LuaContainer {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("padding", |_lua, (this, padding): (Self, f32)| {
            Ok(LuaContainer(this.0.padding(padding)))
        });
        methods.add_function_mut("width", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.width(val)))
        });
        methods.add_function_mut("height", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.height(val)))
        });
        methods.add_function_mut("max_width", |_lua, (this, val): (Self, LuaPixels)| {
            Ok(LuaContainer(this.0.max_width(val)))
        });
        methods.add_function_mut("max_height", |_lua, (this, val): (Self, LuaPixels)| {
            Ok(LuaContainer(this.0.max_height(val)))
        });
        methods.add_function_mut("center_x", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.center_x(val)))
        });
        methods.add_function_mut("center_y", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.center_y(val)))
        });
        methods.add_function_mut("center", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.center(val)))
        });
        methods.add_function_mut("align_left", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.align_left(val)))
        });
        methods.add_function_mut("align_right", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.align_right(val)))
        });
        methods.add_function_mut("align_top", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.align_top(val)))
        });
        methods.add_function_mut("align_bottom", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaContainer(this.0.align_bottom(val)))
        });
        methods.add_function_mut("align_x", |_lua, (this, val): (Self, LuaAlignment)| {
            Ok(LuaContainer(
                this.0.align_x(iced::alignment::Horizontal::from(val.0)),
            ))
        });
        methods.add_function_mut("align_y", |_lua, (this, val): (Self, LuaAlignment)| {
            Ok(LuaContainer(
                this.0.align_y(iced::alignment::Vertical::from(val.0)),
            ))
        });
        methods.add_function_mut("clip", |_lua, (this, val): (Self, mlua::Value)| {
            Ok(LuaContainer(this.0.clip(val.as_boolean().unwrap_or(false))))
        });
        methods.add_function_mut("style", |_lua, (this, func): (Self, mlua::Function)| {
            Ok(LuaContainer(this.0.style(move |theme: &Theme| {
                func.call::<LuaContainerStyle>(LuaTheme(theme.clone()))
                    .unwrap()
                    .0
            })))
        });
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ToolkitProgramLua {
    lua: mlua::Lua,
    update: mlua::Function,
    view: mlua::Function,
}

impl ToolkitProgramLua {
    pub fn new() -> mlua::Result<ToolkitProgramLua> {
        let lua = mlua::Lua::new();
        open_iced(&lua)?;

        lua.load(
            "
local PALETTE = iced.palette(
    iced.color( 0.2, 0.2, 0.2, 1 ), -- background
    iced.color( 0.95, 0.95, 0.95, 1 ), -- text
    iced.color( 0.25, 0.25, 0.25, 1 ), -- primary
    iced.color( 58.0 / 255.0, 170.0 / 255.0, 153.0 / 255.0, 1 ), -- success
    iced.color( 204.0 / 255.0, 68.0 / 255.0, 153.0 / 255.0, 1 ) -- danger
)

local function generate( palette )
    local palext = palette:generate()
    return palext
end

function update( msg )
    print( msg )
end

local function window( theme )
    local palette = theme:palette()
    local palext = theme:extended_palette()

    return iced.Container.style()
    :border( iced.border( palext.background.strong.color, 1, 10 ) )
    :background( palette.background )
end

function view()
    return iced.container(
        iced.container(
            iced.column{
                iced.button('Load Game'),
                iced.button('New Game'):on_press('New Game'),
                iced.button('Editors'):on_press('Editors'),
                iced.button('Options'):on_press('Options'),
                iced.button('Credits'):on_press('Credits'),
                iced.button('Exit Game'):on_press('Exit Game'),
            }
            :spacing(10)
            :padding(20)
            :align_x( iced.Center() )
        )
        :style( window )
        :align_x( iced.Center() )
        :width( 150 )
    )
    :center( iced.Fill() )
end
        ",
        )
        .exec()?;

        let globals = lua.globals();
        Ok(ToolkitProgramLua {
            lua,
            update: globals.get("update")?,
            view: globals.get("view")?,
        })
    }
}

fn value_to_element(
    val: mlua::Value,
) -> mlua::Result<iced::Element<'static, Message, Theme, Renderer>> {
    //dbg!(&val);
    match val {
        mlua::Value::String(s) => Ok(iced_widget::text(s.to_string_lossy()).into()),
        mlua::Value::UserData(ud) => {
            if ud.is::<LuaButton>() {
                Ok(ud.take::<LuaButton>()?.into())
            } else if ud.is::<LuaColumn>() {
                Ok(ud.take::<LuaColumn>()?.into())
            } else if ud.is::<LuaContainer>() {
                Ok(ud.take::<LuaContainer>()?.into())
            } else {
                Err(mlua::Error::UserDataTypeMismatch)
            }
        }
        _ => Err(mlua::Error::UserDataTypeMismatch),
    }
}

pub fn open_iced(lua: &mlua::Lua) -> mlua::Result<()> {
    let iced = lua.create_table()?;
    let globals = lua.globals();
    // Lengths
    iced.set(
        "Fill",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::Fill))
        })?,
    )?;
    iced.set(
        "FillPortion",
        lua.create_function(|_lua, val: u16| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::FillPortion(val)))
        })?,
    )?;
    iced.set(
        "Shrink",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::Shrink))
        })?,
    )?;
    iced.set(
        "Fixed",
        lua.create_function(|_lua, val: f32| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::Fixed(val)))
        })?,
    )?;
    // Padding
    iced.set(
        "padding",
        lua.create_function(|_lua, val: f32| -> mlua::Result<LuaPadding> {
            Ok(LuaPadding(iced::Padding::new(val)))
        })?,
    )?;
    // Alignment
    iced.set(
        "Start",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaAlignment> {
            Ok(LuaAlignment(iced::Alignment::Start))
        })?,
    )?;
    iced.set(
        "Center",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaAlignment> {
            Ok(LuaAlignment(iced::Alignment::Center))
        })?,
    )?;
    iced.set(
        "End",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaAlignment> {
            Ok(LuaAlignment(iced::Alignment::End))
        })?,
    )?;
    // Color
    iced.set(
        "color",
        lua.create_function(
            |_lua, (r, g, b, a): (f32, f32, f32, f32)| -> mlua::Result<LuaColor> {
                Ok(LuaColor(iced::Color::new(r, g, b, a)))
            },
        )?,
    )?;
    // Border
    iced.set(
        "border",
        lua.create_function(
            |_lua, (color, width, radius): (LuaColor, f32, LuaRadius)| -> mlua::Result<LuaBorder> {
                Ok(LuaBorder(iced::Border {
                    color: color.0,
                    width,
                    radius: radius.0,
                }))
            },
        )?,
    )?;
    // Palette
    iced.set(
        "palette",
        lua.create_function(
            |_lua,
             (background, text, primary, success, danger): (
                LuaColor,
                LuaColor,
                LuaColor,
                LuaColor,
                LuaColor,
            )|
             -> mlua::Result<LuaPalette> {
                Ok(LuaPalette(iced::theme::Palette {
                    background: background.into(),
                    text: text.into(),
                    primary: primary.into(),
                    success: success.into(),
                    danger: danger.into(),
                }))
            },
        )?,
    )?;
    // Widgets
    iced.set(
        "container",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaContainer> {
            Ok(LuaContainer(
                iced_widget::container(value_to_element(val)?).into(),
            ))
        })?,
    )?;
    let container = lua.create_table()?;
    container.set(
        "style",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaContainerStyle> {
            Ok(LuaContainerStyle(iced_widget::container::Style::default()))
        })?,
    )?;
    iced.set("Container", container)?;
    iced.set(
        "column",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaColumn> {
            match val {
                mlua::Value::Table(t) => {
                    let list: Vec<iced_core::Element<'static, Message, Theme, Renderer>> = t
                        .sequence_values::<mlua::Value>()
                        .map(|v| value_to_element(v.unwrap()).unwrap())
                        .collect();
                    Ok(LuaColumn(iced_widget::Column::from_vec(list)))
                }
                mlua::Value::Nil => Ok(LuaColumn(iced_widget::Column::new())),
                _ => Err(mlua::Error::BadArgument {
                    to: Some(String::from("iced.column")),
                    pos: 1,
                    name: Some(String::from("tbl")),
                    cause: std::sync::Arc::new(mlua::Error::UserDataTypeMismatch),
                }),
            }
        })?,
    )?;
    iced.set(
        "button",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaButton> {
            Ok(LuaButton(
                iced_widget::button(value_to_element(val)?).into(),
            ))
        })?,
    )?;
    globals.set("iced", iced)?;
    Ok(())
}

impl iced_runtime::Program for ToolkitProgramLua {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> iced_runtime::Task<Message> {
        self.update.call::<()>(message.0).unwrap_or_else(|err| {
            panic!("{}", err);
        });
        iced_runtime::Task::none()
    }

    fn view(&self) -> iced_core::Element<Message, Theme, Renderer> {
        let ele = value_to_element(self.view.call::<mlua::Value>(()).unwrap_or_else(|err| {
            panic!("{}", err);
        }))
        .unwrap_or_else(|err| {
            panic!("{}", err);
        });
        ele.into()
    }
}

pub struct Toolkit<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    renderer: iced_wgpu::Renderer,
    viewport: iced_wgpu::graphics::Viewport,
    debug: iced_runtime::Debug,
    cursor_position: iced_core::mouse::Cursor,
    //pub state: Vec<Box<iced_runtime::program::State<dyn iced_runtime::Program>>>,
    pub state: Vec<iced_runtime::program::State<ToolkitProgramLua>>,
}

impl<'a> Toolkit<'a> {
    pub fn new(
        engine: &mut iced_wgpu::Engine,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        scale_factor: f64,
        width: u32,
        height: u32,
    ) -> Toolkit<'a> {
        let mut renderer = iced_wgpu::Renderer::new(
            device,
            engine,
            iced::Font::default(),
            iced::Pixels::from(16),
        );
        let viewport = iced_wgpu::graphics::Viewport::with_physical_size(
            iced::Size::new(width, height),
            scale_factor,
        );
        let mut debug = iced_runtime::Debug::new();
        let mut state = iced_runtime::program::State::new(
            ToolkitProgramLua::new().unwrap_or_else(|err| {
                panic!("{}", err);
            }),
            viewport.logical_size(),
            &mut renderer,
            &mut debug,
        );
        state.queue_event(iced::Event::Window(iced::window::Event::RedrawRequested(
            std::time::Instant::now(),
        )));
        Toolkit {
            device,
            queue,
            renderer,
            viewport,
            debug,
            cursor_position: iced_core::mouse::Cursor::Unavailable,
            state: Vec::new(),
        }
    }

    pub fn update_cursor_position(&mut self, x: f32, y: f32) -> () {
        let s = 1.0 / self.viewport.scale_factor() as f32;
        self.cursor_position =
            iced_core::mouse::Cursor::Available(iced_core::Point::new(x * s, y * s));
    }

    pub fn queue_event(&mut self, event: iced_core::Event) -> () {
        match self.state.last_mut() {
            Some(state) => {
                state.queue_event(event);
            }
            _ => {
                return;
            }
        };
    }

    pub fn update(&mut self) -> () {
        let state = match self.state.last_mut() {
            Some(state) => state,
            _ => {
                return;
            }
        };

        if state.is_queue_empty() {
            return;
        }
        let theme = crate::toolkit::theme();

        // We update iced
        let _ = state.update(
            self.viewport.logical_size(),
            self.cursor_position,
            &mut self.renderer,
            &theme,
            &iced_core::renderer::Style::default(),
            &mut iced_core::clipboard::Null,
            &mut self.debug,
        );

        // Handle events from the app
        //let program = state.program();
        // match program.state {
        //     menu_main::Message::ExitGame => {
        //         break 'running;
        //     }
        //     _ => (),
        // };

        // and request a redraw
        //window.request_redraw();
    }

    pub fn draw(
        &mut self,
        engine: &mut iced_wgpu::Engine,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SurfaceTexture,
    ) -> () {
        self.renderer.present(
            engine,
            self.device,
            self.queue,
            encoder,
            None,
            frame.texture.format(),
            view,
            &self.viewport,
            &self.debug.overlay(),
        );
    }

    //pub fn open ( &mut self, program: impl iced_runtime::Program<Renderer = iced_wgpu::Renderer> + 'static ) -> () {
    pub fn open(&mut self, program: ToolkitProgramLua) -> () {
        let state = iced_runtime::program::State::new(
            program,
            self.viewport.logical_size(),
            &mut self.renderer,
            &mut self.debug,
        );
        self.state.push(state);
    }
}
