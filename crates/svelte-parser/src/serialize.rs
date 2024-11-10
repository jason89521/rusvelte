use oxc_span::Span;
use serde::{ser::SerializeMap, Serialize};

use crate::ast::*;

impl Serialize for Root<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        let Span { start, end, .. } = self.span;
        map.serialize_entry("type", "Root")?;
        map.serialize_entry("start", &start)?;
        map.serialize_entry("end", &end)?;
        if self.css.is_none() {
            map.serialize_entry("css", "null")?;
        } else {
            panic!("Have'nt implement css serialize")
        }
        map.serialize_entry("fragment", &self.fragment)?;

        map.end()
    }
}

impl Serialize for FragmentNode<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FragmentNode::Text(x) => Serialize::serialize(x, serializer),
            FragmentNode::Element(x) => Serialize::serialize(x, serializer),
            FragmentNode::Tag(x) => Serialize::serialize(x, serializer),
        }
    }
}

impl Serialize for Text<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        let Span { start, end, .. } = self.span;
        map.serialize_entry("type", "Text")?;
        map.serialize_entry("start", &start)?;
        map.serialize_entry("end", &end)?;
        map.serialize_entry("raw", &self.raw)?;
        map.serialize_entry("data", &self.data)?;

        map.end()
    }
}

impl Serialize for Fragment<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", "Fragment")?;
        map.serialize_entry("nodes", &self.nodes)?;
        map.end()
    }
}

impl Serialize for RegularElement<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        let Span { start, end, .. } = self.span;
        map.serialize_entry("type", "RegularElement")?;
        map.serialize_entry("start", &start)?;
        map.serialize_entry("end", &end)?;
        map.serialize_entry("name", self.name)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.serialize_entry("fragment", &self.fragment)?;

        map.end()
    }
}

impl Serialize for Comment<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        let Span { start, end, .. } = self.span;
        map.serialize_entry("type", "Comment")?;
        map.serialize_entry("start", &start)?;
        map.serialize_entry("end", &end)?;
        map.serialize_entry("data", &self.data)?;

        map.end()
    }
}

impl Serialize for Element<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Element::RegularElement(x) => Serialize::serialize(x, serializer),
            Element::Comment(x) => Serialize::serialize(x, serializer),
        }
    }
}

impl Serialize for ExpressionTag<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        let Span { start, end, .. } = self.span;
        map.serialize_entry("type", "ExpressionTag")?;
        map.serialize_entry("start", &start)?;
        map.serialize_entry("end", &end)?;
        map.serialize_entry("expression", &self.expression)?;

        map.end()
    }
}

impl Serialize for Tag<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Tag::ExpressionTag(x) => Serialize::serialize(x, serializer),
        }
    }
}

impl Serialize for NormalAttribute<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        let Span { start, end, .. } = self.span;
        map.serialize_entry("type", "Attribute")?;
        map.serialize_entry("start", &start)?;
        map.serialize_entry("end", &end)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("value", &self.value)?;

        map.end()
    }
}

impl Serialize for AttributeValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AttributeValue::ExpressionTag(x) => Serialize::serialize(x, serializer),
            AttributeValue::Vec(x) => Serialize::serialize(x, serializer),
            AttributeValue::True => Serialize::serialize("true", serializer),
        }
    }
}

impl Serialize for QuotedAttributeValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            QuotedAttributeValue::ExpressionTag(x) => Serialize::serialize(x, serializer),
            QuotedAttributeValue::Text(x) => Serialize::serialize(x, serializer),
        }
    }
}

impl Serialize for SpreadAttribute<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        let Span { start, end, .. } = self.span;
        map.serialize_entry("type", "SpreadAttribute")?;
        map.serialize_entry("start", &start)?;
        map.serialize_entry("end", &end)?;
        map.serialize_entry("expression", &self.expression)?;

        map.end()
    }
}

impl Serialize for Attribute<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Attribute::NormalAttribute(x) => Serialize::serialize(x, serializer),
            Attribute::SpreadAttribute(x) => Serialize::serialize(x, serializer),
        }
    }
}
