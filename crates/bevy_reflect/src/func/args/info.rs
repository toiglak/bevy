use alloc::borrow::Cow;

use crate::func::args::{GetOwnership, Ownership};
use crate::TypePath;

/// Type information for an [`Arg`] used in a [`DynamicFunction`] or [`DynamicClosure`].
///
/// [`Arg`]: crate::func::args::Arg
/// [`DynamicFunction`]: crate::func::function::DynamicFunction
/// [`DynamicClosure`]: crate::func::closures::DynamicClosure
#[derive(Debug, Clone)]
pub struct ArgInfo {
    /// The index of the argument within its function.
    index: usize,
    /// The name of the argument (if provided).
    name: Option<Cow<'static, str>>,
    /// The ownership of the argument.
    ownership: Ownership,
    /// The [type path] of the argument.
    ///
    /// [type path]: TypePath::type_path
    type_path: &'static str,
}

impl ArgInfo {
    /// Create a new [`ArgInfo`] with the given argument index and type `T`.
    ///
    /// To set the name of the argument, use [`Self::with_name`].
    pub fn new<T: TypePath + GetOwnership>(index: usize) -> Self {
        Self {
            index,
            name: None,
            ownership: T::ownership(),
            type_path: T::type_path(),
        }
    }

    /// Set the name of the argument.
    ///
    /// Reflected arguments are not required to have a name and by default are not given one,
    /// so this method must be called manually to set the name.
    pub fn with_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// The index of the argument within its function.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The name of the argument, if it was given one.
    ///
    /// Note that this may return `None` even if the argument has a name.
    /// This is because the name needs to be manually set using [`Self::with_name`]
    /// since the name can't be inferred from the function type alone.
    ///
    /// For [`DynamicFunctions`] created using [`IntoFunction`]
    /// or [`DynamicClosures`] created using [`IntoClosure`],
    /// the name will always be `None`.
    ///
    /// [`DynamicFunctions`]: crate::func::DynamicFunction
    /// [`IntoFunction`]: crate::func::IntoFunction
    /// [`DynamicClosures`]: crate::func::DynamicClosure
    /// [`IntoClosure`]: crate::func::IntoClosure
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// The ownership of the argument.
    pub fn ownership(&self) -> Ownership {
        self.ownership
    }

    pub fn type_path(&self) -> &'static str {
        self.type_path
    }

    /// Get an ID representing the argument.
    ///
    /// This will return `ArgId::Name` if the argument has a name,
    /// otherwise `ArgId::Index`.
    pub fn id(&self) -> ArgId {
        self.name
            .clone()
            .map(ArgId::Name)
            .unwrap_or_else(|| ArgId::Index(self.index))
    }
}

/// A representation of an argument.
///
/// This is primarily used for error reporting and debugging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgId {
    /// The index of the argument within its function.
    Index(usize),
    /// The name of the argument.
    Name(Cow<'static, str>),
}
