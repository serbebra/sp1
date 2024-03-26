use crate::prelude::*;
use sp1_recursion_derive::DslVariable;

use crate::{
    ir::{Config, Felt, Usize},
    prelude::{Builder, Var},
};
use p3_field::{AbstractField, TwoAdicField};

/// Reference: https://github.com/Plonky3/Plonky3/blob/main/commit/src/domain.rs#L55
#[derive(DslVariable, Clone)]
pub struct TwoAdicMultiplicativeCosetVariable<C: Config> {
    pub log_n: Var<C::N>,
    pub size: Var<C::N>,
    pub shift: Felt<C::F>,
    pub g: Felt<C::F>,
}

impl<C: Config> TwoAdicMultiplicativeCosetVariable<C> {
    /// Reference: https://github.com/Plonky3/Plonky3/blob/main/commit/src/domain.rs#L74
    pub fn first_point(&self) -> Felt<C::F> {
        self.shift
    }

    pub fn size(&self) -> Var<C::N> {
        self.size
    }

    pub fn gen(&self) -> Felt<C::F> {
        self.g
    }
}

impl<C: Config> Builder<C> {
    pub fn const_domain(
        &mut self,
        domain: &p3_commit::TwoAdicMultiplicativeCoset<C::F>,
    ) -> TwoAdicMultiplicativeCosetVariable<C>
    where
        C::F: TwoAdicField,
    {
        let log_d_val = domain.log_n as u32;
        let g_val = C::F::two_adic_generator(domain.log_n);
        // Initialize a domain.
        TwoAdicMultiplicativeCosetVariable::<C> {
            log_n: self.eval::<Var<_>, _>(C::N::from_canonical_u32(log_d_val)),
            size: self.eval::<Var<_>, _>(C::N::from_canonical_u32(1 << (log_d_val))),
            shift: self.eval(domain.shift),
            g: self.eval(g_val),
        }
    }
    // /// Reference: https://github.com/Plonky3/Plonky3/blob/main/commit/src/domain.rs#L77
    // pub fn next_point(
    //     &mut self,
    //     domain: &TwoAdicMultiplicativeCosetVariable<C>,
    //     point: Ext<C::F, C::EF>,
    // ) -> Ext<C::F, C::EF> {
    //     self.eval(point * domain.gen())
    // }

    // /// Reference: https://github.com/Plonky3/Plonky3/blob/main/commit/src/domain.rs#L112
    // pub fn selectors_at_point(
    //     &mut self,
    //     domain: &TwoAdicMultiplicativeCosetVariable<C>,
    //     point: Ext<C::F, C::EF>,
    // ) -> LagrangeSelectors<Ext<C::F, C::EF>> {
    //     let unshifted_point: Ext<_, _> = self.eval(point * domain.shift.inverse());
    //     let z_h_expr = self
    //         .exp_power_of_2_v::<Ext<_, _>>(unshifted_point, Usize::Var(domain.log_n))
    //         - C::EF::one();
    //     let z_h: Ext<_, _> = self.eval(z_h_expr);

    //     LagrangeSelectors {
    //         is_first_row: self.eval(z_h / (unshifted_point - C::EF::one())),
    //         is_last_row: self.eval(z_h / (unshifted_point - domain.gen().inverse())),
    //         is_transition: self.eval(unshifted_point - domain.gen().inverse()),
    //         inv_zeroifier: self.eval(z_h.inverse()),
    //     }
    // }

    // /// Reference: https://github.com/Plonky3/Plonky3/blob/main/commit/src/domain.rs#L87
    // pub fn zp_at_point(
    //     &mut self,
    //     domain: &TwoAdicMultiplicativeCosetVariable<C>,
    //     point: Ext<C::F, C::EF>,
    // ) -> Ext<C::F, C::EF> {
    //     // Compute (point * domain.shift.inverse()).exp_power_of_2(domain.log_n) - Ext::one()
    //     let unshifted_power = self.exp_power_of_2_v::<Ext<_, _>>(
    //         point * domain.shift.inverse(),
    //         Usize::Var(domain.log_n),
    //     );
    //     self.eval(unshifted_power - C::EF::one())
    // }

    // pub fn split_domains(
    //     &mut self,
    //     domain: &TwoAdicMultiplicativeCosetVariable<C>,
    //     log_num_chunks: usize,
    // ) -> Vec<TwoAdicMultiplicativeCosetVariable<C>> {
    //     let num_chunks = 1 << log_num_chunks;
    //     let log_n: Var<_> = self.eval(domain.log_n - C::N::from_canonical_usize(log_num_chunks));
    //     let size = self.power_of_two_usize(Usize::Var(log_n));
    //     let size = size.materialize(self);

    //     let g_dom = domain.gen();

    //     // let mut domain_powers = vec![];
    //     // domain_powers.push(domain_power);
    //     // for _ in 0..
    //     // let domain_power = |i| {
    //     //     let mut result = SymbolicFelt::from(g_dom);
    //     //     for _ in 0..i {
    //     //         result *= g_dom;
    //     //     }
    //     //     result
    //     // };

    //     // We can compute a generator for the domain by computing g_dom^{log_num_chunks}
    //     let g = self.exp_power_of_2_v::<Felt<C::F>>(g_dom, log_num_chunks.into());

    //     let domain_power = g_dom;
    //     let mut domains = vec![];
    //     for _ in 0..num_chunks {
    //         let shift: Felt<_> = self.eval(domain.shift * domain_power);
    //         domains.push(TwoAdicMultiplicativeCosetVariable {
    //             log_n,
    //             size,
    //             shift,
    //             g,
    //         });
    //         self.assign(domain_power, domain_power * g_dom);
    //     }
    //     // (0..num_chunks)
    //     //     .map(|i| TwoAdicMultiplicativeCosetVariable {
    //     //         log_n,
    //     //         size,
    //     //         shift: self.eval(domain.shift * domain_power(i)),
    //     //         g,
    //     //     })
    //     //     .collect()

    //     domains
    // }
}
