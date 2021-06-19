use syn::{parse, ForeignItemFn, ItemFn, Stmt};

use crate::{
    ast::{Local, SoftwareTask, SoftwareTaskArgs},
    parse::util,
    Map,
};

impl SoftwareTask {
    pub(crate) fn parse(args: SoftwareTaskArgs, item: ItemFn) -> parse::Result<Self> {
        let valid_signature =
            util::check_fn_signature(&item) && util::type_is_unit(&item.sig.output);

        let span = item.sig.ident.span();

        let name = item.sig.ident.to_string();

        if valid_signature {
            if let Some((context, Ok(inputs))) = util::parse_inputs(item.sig.inputs, &name) {
                let (locals, stmts) = util::extract_locals(item.block.stmts)?;
                let (cfgs, attrs) = util::extract_cfgs(item.attrs);
                let (rauk, attrs) = util::extract_rauk(attrs);

                return Ok(SoftwareTask {
                    args,
                    attrs,
                    cfgs,
                    context,
                    inputs,
                    locals: Local::parse(locals)?,
                    stmts,
                    is_extern: false,
                    rauk,
                });
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context, ..)`",
                name
            ),
        ))
    }
}

impl SoftwareTask {
    pub(crate) fn parse_foreign(
        args: SoftwareTaskArgs,
        item: ForeignItemFn,
    ) -> parse::Result<Self> {
        let valid_signature =
            util::check_foreign_fn_signature(&item) && util::type_is_unit(&item.sig.output);

        let span = item.sig.ident.span();

        let name = item.sig.ident.to_string();

        if valid_signature {
            if let Some((context, Ok(inputs))) = util::parse_inputs(item.sig.inputs, &name) {
                let (cfgs, attrs) = util::extract_cfgs(item.attrs);
                let (rauk, attrs) = util::extract_rauk(attrs);

                return Ok(SoftwareTask {
                    args,
                    attrs,
                    cfgs,
                    context,
                    inputs,
                    locals: Map::<Local>::new(),
                    stmts: Vec::<Stmt>::new(),
                    is_extern: true,
                    rauk,
                });
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context, ..)`",
                name
            ),
        ))
    }
}
