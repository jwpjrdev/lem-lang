pub struct VarContainer<T: AllowedVarType> {
    var_type: VarType,
    name: String,
    value: T,
}

pub enum VarType {
    String,
    Integer,
    Boolean,
}

pub trait AllowedVarType {
    fn get_type() -> VarType;
}
impl AllowedVarType for String {
    fn get_type() -> VarType {
        VarType::String
    }
}
impl AllowedVarType for f64 {
    fn get_type() -> VarType {
        VarType::Integer
    }
}
impl AllowedVarType for bool {
    fn get_type() -> VarType {
        VarType::Boolean
    }
}
