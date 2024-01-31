// class AstVisitor:

//     def __init__(self, traversal='post', log=False):
//         self.traversal = traversal
//         self.log = log
use crate::zkay_ast::ast::{ASTChildren, AST};
pub trait AstVisitor {
    type Return;
    fn visit(&self, ast: AST) -> Self::Return {
        self._visit_internal(ast)
    }
    fn log(&self) -> bool;
    fn traversal(&self) -> &'static str;
    fn has_attr(&self, name: &String) -> bool;
    fn get_attr(&self, name: &String) -> Option<String>;
    fn temper_result(&self) -> Self::Return;
    fn _visit_internal(&self, ast: AST) -> Option<Self::Return >{
        if self.log() {
            // std::any::type_name::<Option<String>>(),
            print!("Visiting {:?}", ast);
        }
        let mut ret = None;
        let mut ret_children = None;

        if self.traversal() == "post" {
            ret_children = Some(self.visit_children(&ast));
        }
        let f = Some(self.try_call_visit_function("SourceUnit", &ast));
        if f.is_some() {
            ret = Some(f);
        } else if self.traversal() == "node-or-children" {
            ret_children = self.visit_children(&ast);
        }
        if self.traversal() == "pre" {
            ret_children = Some(self.visit_children(&ast));
        }
        if ret.is_some() {
            // Some(ret)
            None
        } else if ret_children.is_some() {
            ret_children
        } else {
            None
        }
    }

    fn try_call_visit_function(&self, c: &str, ast: &AST) -> Option<Self::Return>
// std::any::type_name::<Option<String>>(),
    {
        let visitor_function = c.to_owned(); // String::from("visit") +
        let ret = Some(self.call_visit_function(ast));
        if ret.is_some() {
            return ret;
        } else {
            let f = self.try_call_visit_function(AST::bases(c), ast);
            if f.is_some() {
                return f;
            }
        }
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Self::Return;
    fn visit_children(&self, ast: &AST) -> Self::Return {
        let mut ast = ast.clone();
        for c in ast.children() {
            self.visit(c);
        }
        self.temper_result()
    }
}

use std::{convert::Infallible, error, fmt};

use serde::{
    de,
    ser::{self, Impossible},
    Deserialize, Deserializer, Serialize, Serializer,
};

fn main() {
    #[derive(Serialize, Deserialize)]
    struct MyStruct {
        field: usize,
    }

    let my_struct = MyStruct { field: 1 };

    let name_of_struct: String = my_struct.get_struct_name_ser().unwrap().to_owned();
    println!("{}", name_of_struct); // prints "MyStruct"
    println!("or: {}", MyStruct::get_struct_name_de().unwrap()); // prints "MyStruct", too
}

#[derive(Debug)]
struct Error(Option<&'static str>);
impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error(None)
    }
}
impl de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error(None)
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StructName implementation detail, not meant to be displayed")
    }
}
impl<T: ?Sized> StructName for T {}
pub trait StructName {
    fn get_struct_name_ser(&self) -> Option<&'static str>
    where
        Self: Serialize,
    {
        struct GetStructName;
        impl Serializer for GetStructName {
            type Ok = Infallible;

            type Error = Error;

            type SerializeSeq = Impossible<Self::Ok, Self::Error>;

            type SerializeTuple = Impossible<Self::Ok, Self::Error>;

            type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

            type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

            type SerializeMap = Impossible<Self::Ok, Self::Error>;

            type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

            type SerializeStruct = Impossible<Self::Ok, Self::Error>;

            fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize,
            {
                Err(Error(None))
            }

            fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_unit_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
            ) -> Result<Self::Ok, Self::Error> {
                Err(Error(None))
            }

            fn serialize_newtype_variant<T: ?Sized>(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
                _value: &T,
            ) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize,
            {
                Err(Error(None))
            }

            fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
                Err(Error(None))
            }

            fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
                Err(Error(None))
            }

            fn serialize_tuple_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeTupleVariant, Self::Error> {
                Err(Error(None))
            }

            fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
                Err(Error(None))
            }

            fn serialize_struct_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeStructVariant, Self::Error> {
                Err(Error(None))
            }

            fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
                Err(Error(Some(name)))
            }

            fn serialize_newtype_struct<T: ?Sized>(
                self,
                name: &'static str,
                _value: &T,
            ) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize,
            {
                Err(Error(Some(name)))
            }

            fn serialize_tuple_struct(
                self,
                name: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeTupleStruct, Self::Error> {
                Err(Error(Some(name)))
            }

            fn serialize_struct(
                self,
                name: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeStruct, Self::Error> {
                Err(Error(Some(name)))
            }
        }
        self.serialize(GetStructName).unwrap_err().0
    }

    fn get_struct_name_de<'de>() -> Option<&'static str>
    where
        Self: Deserialize<'de>,
    {
        struct GetStructName;
        impl<'de> Deserializer<'de> for GetStructName {
            type Error = Error;

            fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_enum<V>(
                self,
                _name: &'static str,
                _variants: &'static [&'static str],
                _visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(None))
            }

            fn deserialize_unit_struct<V>(
                self,
                name: &'static str,
                _visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(Some(name)))
            }

            fn deserialize_newtype_struct<V>(
                self,
                name: &'static str,
                _visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(Some(name)))
            }

            fn deserialize_tuple_struct<V>(
                self,
                name: &'static str,
                _len: usize,
                _visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(Some(name)))
            }

            fn deserialize_struct<V>(
                self,
                name: &'static str,
                _fields: &'static [&'static str],
                _visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(Error(Some(name)))
            }
        }

        Self::deserialize(GetStructName).err()?.0
    }
}
