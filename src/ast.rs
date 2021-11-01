use crate::define::NType;
use std::fmt::{Display, Formatter};

pub(crate) trait E: Display {}
pub(crate) trait S: Display {}

pub(crate) struct VarStatements {
    pub(crate) m_type: NType,
    pub(crate) identifier: Option<Box<dyn E>>,
    pub(crate) init: Option<Box<dyn E>>,
}

impl Display for VarStatements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m_type:{:?},identifier:{},init:{{{}}}",
            self.m_type,
            self.identifier.as_ref().unwrap(),
            self.init.as_ref().unwrap()
        )
    }
}

impl S for VarStatements {}

#[derive(Debug)]
pub(crate) struct ID {
    pub(crate) name: String,
    pub(crate) xtype: NType,
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{xtype:{:?},name:{},struct:ID}}", self.xtype, self.name)
    }
}

impl E for ID {}

#[derive(Debug)]
pub struct Literal {
    pub(crate) m_type: NType,
    pub(crate) value: String,
}

impl Literal {
    pub(crate) fn new() -> Literal {
        Literal {
            m_type: NType::None,
            value: "".to_string(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{m_type:{:?},value:{},struct:Literal}}",
            self.m_type, self.value
        )
    }
}

impl E for Literal {}

pub(crate) struct OperatorExpress {
    pub(crate) left: Option<Box<dyn E>>,
    pub(crate) operator: String,
    pub(crate) right: Option<Box<dyn E>>,
}
impl E for OperatorExpress {}

impl Display for OperatorExpress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OperatorExpress{{left:{},Operator:{},right:{} }}",
            self.left.as_ref().unwrap(),
            self.operator,
            self.right.as_ref().unwrap()
        )
    }
}
