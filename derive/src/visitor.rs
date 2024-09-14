use crate::err::VisitResult;
use crate::meta::DataMetaInfo;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{DataEnum, Field, Fields, FieldsNamed, FieldsUnnamed, Type, Variant};

#[derive(Debug)]
#[allow(dead_code)]
pub struct EnumVisitor<'meta> {
    meta: &'meta mut DataMetaInfo,
    res: VisitResult<()>,
}

impl<'meta> EnumVisitor<'meta> {
    pub fn new(meta: &'meta mut DataMetaInfo) -> Self {
        EnumVisitor { meta, res: Ok(()) }
    }
}

impl<'meta, 'ast> Visit<'ast> for EnumVisitor<'meta> {
    fn visit_variant(&mut self, node: &'ast Variant) {
        let mut handle = || {
            let mut type_collector = TypeCollector::new(self.meta);
            type_collector.visit_fields(&node.fields);
            type_collector.get_res()?;

            let mut fields_subst = FieldSubstitutor::new(self.meta);
            let mut fields = node.fields.clone();
            fields_subst.visit_fields_mut(&mut fields);
            fields_subst.get_res()?;

            self.meta.constructors.insert(
                node.ident.clone(),
                (fields, node.discriminant.as_ref().cloned().map(|d| d.1)),
            );

            Ok(())
        };

        if self.res.is_ok() {
            self.res = handle();
        }
    }

    fn visit_data_enum(&mut self, i: &'ast DataEnum) {
        if self.res.is_ok() {
            i.variants.iter().fold(self, |visitor, variant| {
                visitor.visit_variant(variant);
                visitor
            });
        }
    }
}

pub struct FieldSubstitutor<'meta> {
    meta: &'meta mut DataMetaInfo,
    res: VisitResult<()>,
}

pub struct TypeCollector<'meta> {
    meta: &'meta mut DataMetaInfo,
    res: VisitResult<()>,
}

impl<'meta> FieldSubstitutor<'meta> {
    pub fn new(meta: &'meta mut DataMetaInfo) -> Self {
        FieldSubstitutor { meta, res: Ok(()) }
    }

    pub fn get_res(self) -> VisitResult<()> {
        self.res
    }
}

impl<'meta> TypeCollector<'meta> {
    pub fn new(meta: &'meta mut DataMetaInfo) -> Self {
        TypeCollector { meta, res: Ok(()) }
    }

    pub fn get_res(self) -> VisitResult<()> {
        self.res
    }
}

impl<'meta> VisitMut for FieldSubstitutor<'meta> {
    fn visit_type_mut(&mut self, i: &mut Type) {
        let mut handle = || {
            let inner = self.meta.boxed_ty(i);
            *i = inner;
            Ok(())
        };

        if self.res.is_ok() {
            self.res = handle();
        }
    }

    fn visit_field_mut(&mut self, i: &mut Field) {
        self.visit_type_mut(&mut i.ty);
    }

    fn visit_fields_named_mut(&mut self, i: &mut FieldsNamed) {
        i.named.iter_mut().fold(self, |visitor, field| {
            if visitor.res.is_ok() {
                visitor.visit_field_mut(field);
            }

            visitor
        });
    }

    fn visit_fields_unnamed_mut(&mut self, i: &mut FieldsUnnamed) {
        i.unnamed.iter_mut().fold(self, |visitor, field| {
            if visitor.res.is_ok() {
                visitor.visit_field_mut(field);
            }

            visitor
        });
    }

    fn visit_fields_mut(&mut self, i: &mut Fields) {
        match i {
            Fields::Named(named) => self.visit_fields_named_mut(named),
            Fields::Unnamed(unnamed) => self.visit_fields_unnamed_mut(unnamed),
            _ => {}
        };
    }
}

impl<'meta, 'ast> Visit<'ast> for TypeCollector<'meta> {
    fn visit_type(&mut self, i: &Type) {
        self.meta.push_boxed_type(i)
    }

    fn visit_field(&mut self, i: &Field) {
        self.visit_type(&i.ty);
    }

    fn visit_fields_named(&mut self, i: &FieldsNamed) {
        i.named.iter().fold(self, |visitor, field| {
            visitor.visit_field(field);

            visitor
        });
    }

    fn visit_fields_unnamed(&mut self, i: &FieldsUnnamed) {
        i.unnamed.iter().fold(self, |visitor, field| {
            visitor.visit_field(field);

            visitor
        });
    }

    fn visit_fields(&mut self, i: &Fields) {
        match i {
            Fields::Named(named) => self.visit_fields_named(named),
            Fields::Unnamed(unnamed) => self.visit_fields_unnamed(unnamed),
            _ => {}
        };
    }
}
