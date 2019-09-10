use crate::common::position::Position;
use crate::parser::ast::{Node, AST};
use crate::type_checker::context::common::try_from_id;
use crate::type_checker::context::type_name::TypeName;
use crate::type_checker::context::ReturnType;
use crate::type_checker::type_result::TypeErr;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Field {
    pub name:     String,
    pub mutable:  bool,
    pub private:  bool,
    pub position: Position,
    ty:           Option<TypeName>
}

impl TryFrom<&AST> for Field {
    type Error = TypeErr;

    fn try_from(field: &AST) -> Result<Self, Self::Error> {
        match &field.node {
            Node::VariableDef { id_maybe_type, private, .. } => match &id_maybe_type.node {
                Node::IdType { id, mutable, _type } => Ok(Field {
                    name:     try_from_id(id)?,
                    mutable:  *mutable,
                    private:  *private,
                    position: field.pos.clone(),
                    ty:       match _type {
                        Some(ty) => Some(TypeName::try_from(ty.as_ref())?),
                        None => None
                    }
                }),
                _ => Err(TypeErr::new(&id_maybe_type.pos, "Expected identifier and type"))
            },
            _ => Err(TypeErr::new(&field.pos, "Expected field"))
        }
    }
}

impl ReturnType for Field {
    fn with_return_type_name(self, ty: TypeName) -> Result<Self, TypeErr> {
        if self.ty.is_some() && self.ty.unwrap() != ty {
            Err(TypeErr::new(&self.position, "Inferred type not equal to signature"))
        } else {
            Ok(Field {
                name:     self.name,
                private:  self.private,
                mutable:  self.mutable,
                position: self.position.clone(),
                ty:       Some(ty)
            })
        }
    }

    fn get_return_type_name(&self) -> Result<TypeName, TypeErr> {
        match &self.ty {
            Some(ty) => Ok(ty.clone()),
            None => Err(TypeErr::new(&self.position, "Type cannot be inferred"))
        }
    }
}
