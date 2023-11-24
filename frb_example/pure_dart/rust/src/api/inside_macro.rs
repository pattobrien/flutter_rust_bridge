// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

macro_rules! generate_struct {
    ($name:ident) => {
        pub struct $name {
            pub data: i32,
        }

        pub fn func_macro_struct(arg: $name) -> $name {
            arg
        }
    };
}

generate_struct!(MacroStruct);

macro_rules! generate_another_struct {
    () => {
        #[frb]
        pub struct AnotherMacroStruct {
            pub data: i32,
            #[frb(non_final)]
            pub non_final_data: i32,
        }
        #[allow(unused)]
        pub fn another_macro_struct() -> AnotherMacroStruct {
            AnotherMacroStruct {
                data: 123,
                non_final_data: 0,
            }
        }
    };
}

generate_another_struct!();
