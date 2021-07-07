use crate::{
    analyzer::{assign::AssignOpts, Analyzer},
    ValidationResult,
};
use stc_ts_errors::Error;
use stc_ts_types::TsExpr;
use swc_common::{Span, TypeEq};

impl Analyzer<'_, '_> {
    pub(crate) fn report_error_for_conflicting_parents(&mut self, span: Span, parent: &[TsExpr]) {
        if self.is_builtin {
            return;
        }

        for (i, p1) in parent.iter().enumerate() {
            let res: ValidationResult<()> = try {
                let p1_type =
                    self.type_of_ts_entity_name(span, self.ctx.module_id, &p1.expr, p1.type_args.as_deref())?;

                for (j, p2) in parent.iter().enumerate() {
                    if i <= j {
                        continue;
                    }
                    // Declaration merging
                    if p1.type_eq(p2) {
                        continue;
                    }

                    let p2 =
                        self.type_of_ts_entity_name(span, self.ctx.module_id, &p2.expr, p2.type_args.as_deref())?;

                    if let Err(err) = self.assign_with_opts(
                        &mut Default::default(),
                        AssignOpts {
                            span,
                            // required because interface can extend classes
                            use_missing_fields_for_class: true,
                            allow_unknown_rhs: true,
                            ..Default::default()
                        },
                        &p1_type,
                        &p2,
                    ) {
                        match err.actual() {
                            Error::MissingFields { .. } => {}

                            Error::Errors { errors, .. }
                                if errors.iter().all(|err| match err.actual() {
                                    Error::MissingFields { .. } => true,
                                    _ => false,
                                }) => {}

                            _ => self
                                .storage
                                .report(err.convert(|err| Error::InterfaceNotCompatible { span })),
                        }
                    }
                }
            };
        }
    }
}