use syn::{parse, ForeignItemFn, ItemFn, Stmt};

use crate::{
    ast::{HardwareTask, HardwareTaskArgs, Local},
    parse::util,
    Map,
};

impl HardwareTask {
    pub(crate) fn parse(args: HardwareTaskArgs, item: ItemFn) -> parse::Result<Self> {
        let span = item.sig.ident.span();
        let valid_signature = util::check_fn_signature(&item)
            && item.sig.inputs.len() == 1
            && util::type_is_unit(&item.sig.output);

        let name = item.sig.ident.to_string();

        if name == "init" || name == "idle" {
            return Err(parse::Error::new(
                span,
                "tasks cannot be named `init` or `idle`",
            ));
        }

        if valid_signature {
            if let Some((context, Ok(rest))) = util::parse_inputs(item.sig.inputs, &name) {
                if rest.is_empty() {
                    let (locals, stmts) = util::extract_locals(item.block.stmts)?;
                    let (cfgs, attrs) = util::extract_cfgs(item.attrs);
                    let (rauk, attrs) = util::extract_rauk(attrs);

                    return Ok(HardwareTask {
                        args,
                        cfgs,
                        attrs,
                        context,
                        locals: Local::parse(locals)?,
                        stmts,
                        is_extern: false,
                        rauk,
                    });
                }
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context)`",
                name
            ),
        ))
    }
}

impl HardwareTask {
    pub(crate) fn parse_foreign(
        args: HardwareTaskArgs,
        item: ForeignItemFn,
    ) -> parse::Result<Self> {
        let span = item.sig.ident.span();
        let valid_signature = util::check_foreign_fn_signature(&item)
            && item.sig.inputs.len() == 1
            && util::type_is_unit(&item.sig.output);

        let name = item.sig.ident.to_string();

        if name == "init" || name == "idle" {
            return Err(parse::Error::new(
                span,
                "tasks cannot be named `init` or `idle`",
            ));
        }

        if valid_signature {
            if let Some((context, Ok(rest))) = util::parse_inputs(item.sig.inputs, &name) {
                if rest.is_empty() {
                    let (cfgs, attrs) = util::extract_cfgs(item.attrs);
                    let (rauk, attrs) = util::extract_rauk(attrs);

                    return Ok(HardwareTask {
                        args,
                        cfgs,
                        attrs,
                        context,
                        locals: Map::<Local>::new(),
                        stmts: Vec::<Stmt>::new(),
                        is_extern: true,
                        rauk,
                    });
                }
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context)`",
                name
            ),
        ))
    }
}
