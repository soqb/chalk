use crate::tls;
use chalk_ir::{
    interner::{HasInterner, Interner},
    TupleElem, TupleElemData, TyKind,
};
use chalk_ir::{
    AdtId, AliasTy, AssocTypeId, CanonicalVarKind, CanonicalVarKinds, ConstData, Constraint,
    Constraints, FnDefId, Goals, InEnvironment, Lifetime, OpaqueTy, OpaqueTyId,
    ProgramClauseImplication, ProgramClauses, ProjectionTy, QuantifiedWhereClauses,
    SeparatorTraitRef, Substitution, TraitId, Ty, TyData, VariableKind, VariableKinds, Variances,
};
use chalk_ir::{
    GenericArg, GenericArgData, Goal, GoalData, LifetimeData, ProgramClause, ProgramClauseData,
    QuantifiedWhereClause, Variance,
};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use string_cache::DefaultAtom;

pub type Identifier = DefaultAtom;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawId {
    pub index: u32,
}

impl Debug for RawId {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "#{}", self.index)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChalkFnAbi {
    Rust,
    C,
}

impl Debug for ChalkFnAbi {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                ChalkFnAbi::Rust => "\"rust\"",
                ChalkFnAbi::C => "\"c\"",
            },
        )
    }
}

/// The default "interner" and the only interner used by chalk
/// itself. In this interner, no interning actually occurs.
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ChalkIr;

impl Interner for ChalkIr {
    type InternedType = Arc<TyData<ChalkIr>>;
    type InternedLifetime = LifetimeData<ChalkIr>;
    type InternedConst = Arc<ConstData<ChalkIr>>;
    type InternedConcreteConst = u32;
    type InternedGenericArg = GenericArgData<ChalkIr>;
    type InternedTupleElem = TupleElemData<ChalkIr>;
    type InternedGoal = Arc<GoalData<ChalkIr>>;
    type InternedGoals = Vec<Goal<ChalkIr>>;
    type InternedSubstitution = Vec<GenericArg<ChalkIr>>;
    type InternedTupleContents = Vec<TupleElem<ChalkIr>>;
    type InternedProgramClause = ProgramClauseData<ChalkIr>;
    type InternedProgramClauses = Vec<ProgramClause<ChalkIr>>;
    type InternedQuantifiedWhereClauses = Vec<QuantifiedWhereClause<ChalkIr>>;
    type InternedVariableKinds = Vec<VariableKind<ChalkIr>>;
    type InternedCanonicalVarKinds = Vec<CanonicalVarKind<ChalkIr>>;
    type InternedConstraints = Vec<InEnvironment<Constraint<ChalkIr>>>;
    type InternedVariances = Vec<Variance>;
    type DefId = RawId;
    type InternedAdtId = RawId;
    type Identifier = Identifier;
    type FnAbi = ChalkFnAbi;

    fn debug_adt_id(
        type_kind_id: AdtId<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_adt_id(type_kind_id, fmt)))
    }

    fn debug_trait_id(
        type_kind_id: TraitId<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_trait_id(type_kind_id, fmt)))
    }

    fn debug_assoc_type_id(
        id: AssocTypeId<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_assoc_type_id(id, fmt)))
    }

    fn debug_opaque_ty_id(
        id: OpaqueTyId<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_opaque_ty_id(id, fmt)))
    }

    fn debug_fn_def_id(id: FnDefId<Self>, fmt: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_fn_def_id(id, fmt)))
    }

    fn debug_alias(alias: &AliasTy<ChalkIr>, fmt: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_alias(alias, fmt)))
    }

    fn debug_projection_ty(
        proj: &ProjectionTy<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_projection_ty(proj, fmt)))
    }

    fn debug_opaque_ty(
        opaque_ty: &OpaqueTy<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_opaque_ty(opaque_ty, fmt)))
    }

    fn debug_ty(ty: &Ty<ChalkIr>, fmt: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_ty(ty, fmt)))
    }

    fn debug_lifetime(
        lifetime: &Lifetime<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_lifetime(lifetime, fmt)))
            .or_else(|| Some(write!(fmt, "{:?}", lifetime.interned())))
    }

    fn debug_generic_arg(
        generic_arg: &GenericArg<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_generic_arg(generic_arg, fmt)))
    }

    fn debug_variable_kinds(
        variable_kinds: &VariableKinds<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_variable_kinds(variable_kinds, fmt)))
    }

    fn debug_variable_kinds_with_angles(
        variable_kinds: &VariableKinds<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| {
            Some(prog?.debug_variable_kinds_with_angles(variable_kinds, fmt))
        })
    }

    fn debug_canonical_var_kinds(
        canonical_var_kinds: &CanonicalVarKinds<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| {
            Some(prog?.debug_canonical_var_kinds(canonical_var_kinds, fmt))
        })
    }

    fn debug_goal(goal: &Goal<ChalkIr>, fmt: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_goal(goal, fmt)))
    }

    fn debug_goals(goals: &Goals<ChalkIr>, fmt: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_goals(goals, fmt)))
    }

    fn debug_program_clause_implication(
        pci: &ProgramClauseImplication<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_program_clause_implication(pci, fmt)))
    }

    fn debug_program_clause(
        clause: &ProgramClause<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_program_clause(clause, fmt)))
    }

    fn debug_program_clauses(
        clause: &ProgramClauses<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_program_clauses(clause, fmt)))
    }

    fn debug_substitution(
        substitution: &Substitution<ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_substitution(substitution, fmt)))
    }

    fn debug_tuple_contents(
        tuple_contents: &chalk_ir::TupleContents<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_tuple_contents(tuple_contents, fmt)))
    }

    fn debug_tuple_elem(
        tuple_elem: &TupleElem<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_tuple_elem(tuple_elem, fmt)))
    }

    fn debug_separator_trait_ref(
        separator_trait_ref: &SeparatorTraitRef<'_, ChalkIr>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| {
            Some(prog?.debug_separator_trait_ref(separator_trait_ref, fmt))
        })
    }

    fn debug_quantified_where_clauses(
        clauses: &QuantifiedWhereClauses<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_quantified_where_clauses(clauses, fmt)))
    }

    fn debug_constraints(
        constraints: &Constraints<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_constraints(constraints, fmt)))
    }

    fn debug_variances(
        variances: &Variances<Self>,
        fmt: &mut fmt::Formatter<'_>,
    ) -> Option<fmt::Result> {
        tls::with_current_program(|prog| Some(prog?.debug_variances(variances, fmt)))
    }

    fn intern_ty(self, kind: TyKind<ChalkIr>) -> Arc<TyData<ChalkIr>> {
        let flags = kind.compute_flags(self);
        Arc::new(TyData { kind, flags })
    }

    fn ty_data(self, ty: &Arc<TyData<ChalkIr>>) -> &TyData<Self> {
        ty
    }

    fn intern_lifetime(self, lifetime: LifetimeData<ChalkIr>) -> LifetimeData<ChalkIr> {
        lifetime
    }

    fn lifetime_data(self, lifetime: &LifetimeData<ChalkIr>) -> &LifetimeData<ChalkIr> {
        lifetime
    }

    fn intern_const(self, constant: ConstData<ChalkIr>) -> Arc<ConstData<ChalkIr>> {
        Arc::new(constant)
    }

    fn const_data(self, constant: &Arc<ConstData<ChalkIr>>) -> &ConstData<ChalkIr> {
        constant
    }

    fn const_eq(self, _ty: &Arc<TyData<ChalkIr>>, c1: &u32, c2: &u32) -> bool {
        c1 == c2
    }

    fn intern_generic_arg(self, generic_arg: GenericArgData<ChalkIr>) -> GenericArgData<ChalkIr> {
        generic_arg
    }

    fn generic_arg_data(self, generic_arg: &GenericArgData<ChalkIr>) -> &GenericArgData<ChalkIr> {
        generic_arg
    }

    fn intern_tuple_elem(self, tuple_elem: TupleElemData<ChalkIr>) -> TupleElemData<ChalkIr> {
        tuple_elem
    }

    fn tuple_elem_data(self, tuple_elem: &TupleElemData<ChalkIr>) -> &TupleElemData<ChalkIr> {
        tuple_elem
    }

    fn intern_goal(self, goal: GoalData<ChalkIr>) -> Arc<GoalData<ChalkIr>> {
        Arc::new(goal)
    }

    fn goal_data(self, goal: &Arc<GoalData<ChalkIr>>) -> &GoalData<ChalkIr> {
        goal
    }

    fn intern_goals<E>(
        self,
        data: impl IntoIterator<Item = Result<Goal<ChalkIr>, E>>,
    ) -> Result<Vec<Goal<ChalkIr>>, E> {
        data.into_iter().collect()
    }

    fn goals_data(self, goals: &Vec<Goal<ChalkIr>>) -> &[Goal<ChalkIr>] {
        goals
    }

    fn intern_substitution<E>(
        self,
        data: impl IntoIterator<Item = Result<GenericArg<ChalkIr>, E>>,
    ) -> Result<Vec<GenericArg<ChalkIr>>, E> {
        data.into_iter().collect()
    }

    fn substitution_data(self, substitution: &Vec<GenericArg<ChalkIr>>) -> &[GenericArg<ChalkIr>] {
        substitution
    }

    fn intern_tuple_contents<E>(
        self,
        data: impl IntoIterator<Item = Result<TupleElem<ChalkIr>, E>>,
    ) -> Result<Vec<TupleElem<ChalkIr>>, E> {
        data.into_iter().collect()
    }

    fn tuple_contents_data(self, substitution: &Vec<TupleElem<ChalkIr>>) -> &[TupleElem<ChalkIr>] {
        substitution
    }

    fn intern_program_clause(self, data: ProgramClauseData<Self>) -> ProgramClauseData<Self> {
        data
    }

    fn program_clause_data(self, clause: &ProgramClauseData<Self>) -> &ProgramClauseData<Self> {
        clause
    }

    fn intern_program_clauses<E>(
        self,
        data: impl IntoIterator<Item = Result<ProgramClause<Self>, E>>,
    ) -> Result<Vec<ProgramClause<Self>>, E> {
        data.into_iter().collect()
    }

    fn program_clauses_data(self, clauses: &Vec<ProgramClause<Self>>) -> &[ProgramClause<Self>] {
        clauses
    }

    fn intern_quantified_where_clauses<E>(
        self,
        data: impl IntoIterator<Item = Result<QuantifiedWhereClause<Self>, E>>,
    ) -> Result<Self::InternedQuantifiedWhereClauses, E> {
        data.into_iter().collect()
    }

    fn quantified_where_clauses_data(
        self,
        clauses: &Self::InternedQuantifiedWhereClauses,
    ) -> &[QuantifiedWhereClause<Self>] {
        clauses
    }
    fn intern_generic_arg_kinds<E>(
        self,
        data: impl IntoIterator<Item = Result<VariableKind<ChalkIr>, E>>,
    ) -> Result<Self::InternedVariableKinds, E> {
        data.into_iter().collect()
    }

    fn variable_kinds_data(
        self,
        variable_kinds: &Self::InternedVariableKinds,
    ) -> &[VariableKind<ChalkIr>] {
        variable_kinds
    }

    fn intern_canonical_var_kinds<E>(
        self,
        data: impl IntoIterator<Item = Result<CanonicalVarKind<ChalkIr>, E>>,
    ) -> Result<Self::InternedCanonicalVarKinds, E> {
        data.into_iter().collect()
    }

    fn canonical_var_kinds_data(
        self,
        canonical_var_kinds: &Self::InternedCanonicalVarKinds,
    ) -> &[CanonicalVarKind<ChalkIr>] {
        canonical_var_kinds
    }

    fn intern_constraints<E>(
        self,
        data: impl IntoIterator<Item = Result<InEnvironment<Constraint<Self>>, E>>,
    ) -> Result<Self::InternedConstraints, E> {
        data.into_iter().collect()
    }

    fn constraints_data(
        self,
        constraints: &Self::InternedConstraints,
    ) -> &[InEnvironment<Constraint<Self>>] {
        constraints
    }

    fn intern_variances<E>(
        self,
        data: impl IntoIterator<Item = Result<Variance, E>>,
    ) -> Result<Self::InternedVariances, E> {
        data.into_iter().collect()
    }

    fn variances_data(self, variances: &Self::InternedVariances) -> &[Variance] {
        variances
    }
}

impl HasInterner for ChalkIr {
    type Interner = ChalkIr;
}
