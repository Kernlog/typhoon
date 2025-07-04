use {
    super::GeneratorResult,
    crate::{context::Context, StagedGenerator},
    quote::{format_ident, quote},
    syn::parse_quote,
    typhoon_syn::arguments::{Argument, Arguments},
};

pub struct ArgumentsGenerator<'a>(&'a Context);

impl<'a> ArgumentsGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        ArgumentsGenerator(context)
    }
}

impl StagedGenerator for ArgumentsGenerator<'_> {
    fn append(&mut self, context: &mut GeneratorResult) -> Result<(), syn::Error> {
        let Some(ref args) = self.0.args else {
            return Ok(());
        };

        let context_name = &self.0.item_struct.ident;
        let (name, args_struct) = match args {
            Arguments::Struct(name) => (name.clone(), None),
            Arguments::Values(args) => {
                let struct_name = format_ident!("{context_name}Args");
                let fields = args
                    .iter()
                    .map(|Argument { name, ty }: &Argument| quote!(pub #name: #ty));

                let generated_struct = quote! {
                    #[derive(Debug, PartialEq, bytemuck::AnyBitPattern, bytemuck::NoUninit, Copy, Clone)]
                    #[repr(C)]
                    pub struct #struct_name {
                        #(#fields),*
                    }
                };

                (struct_name, Some(generated_struct))
            }
        };

        context
            .new_fields
            .push(parse_quote!(pub args: Arg<'info, #name>));
        context
            .inside
            .extend(quote!(let args = Arg::<#name>::from_entrypoint(program_id, accounts, instruction_data)?;));

        if let Some(args_struct) = args_struct {
            context.outside.extend(args_struct);
        }

        Ok(())
    }
}
