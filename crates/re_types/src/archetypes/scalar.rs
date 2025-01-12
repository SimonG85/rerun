// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/scalar.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Log a double-precision scalar.
///
/// The current timeline value will be used for the time/X-axis, hence scalars
/// cannot be timeless.
///
/// When used to produce a plot, this archetype is used to provide the data that
/// is referenced by the `SeriesLine` or `SeriesPoint` archetypes. You can do
/// this by logging both archetypes to the same path, or alternatively configuring
/// the plot-specific archetypes through the blueprint.
///
/// See also [`SeriesPoint`][crate::archetypes::SeriesPoint], [`SeriesLine`][crate::archetypes::SeriesLine].
///
/// ## Example
///
/// ### Simple line plot
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar").spawn()?;
///
///     for step in 0..64 {
///         rec.set_time_sequence("step", step);
///         rec.log(
///             "scalar",
///             &rerun::TimeSeriesScalar::new((step as f64 / 10.0).sin()),
///         )?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1200w.png">
///   <img src="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Scalar {
    /// The scalar value to log.
    pub scalar: crate::components::Scalar,

    /// An optional label for the scalar.
    ///
    /// TODO(#1289): This won't show up on points at the moment, as our plots don't yet
    /// support displaying labels for individual points.
    pub text: Option<crate::components::Text>,
}

impl ::re_types_core::SizeBytes for Scalar {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.scalar.heap_size_bytes() + self.text.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::Scalar>::is_pod() && <Option<crate::components::Text>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Scalar".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.ScalarIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.InstanceKey".into(),
            "rerun.components.Text".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Scalar".into(),
            "rerun.components.ScalarIndicator".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.Text".into(),
        ]
    });

impl Scalar {
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`Scalar`] [`::re_types_core::Archetype`]
pub type ScalarIndicator = ::re_types_core::GenericIndicatorComponent<Scalar>;

impl ::re_types_core::Archetype for Scalar {
    type Indicator = ScalarIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Scalar".into()
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: ScalarIndicator = ScalarIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let scalar = {
            let array = arrays_by_name
                .get("rerun.components.Scalar")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Scalar#scalar")?;
            <crate::components::Scalar>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Scalar#scalar")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Scalar#scalar")?
        };
        let text = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            <crate::components::Text>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Scalar#text")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { scalar, text })
    }
}

impl ::re_types_core::AsComponents for Scalar {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.scalar as &dyn ComponentBatch).into()),
            self.text
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        1
    }
}

impl Scalar {
    pub fn new(scalar: impl Into<crate::components::Scalar>) -> Self {
        Self {
            scalar: scalar.into(),
            text: None,
        }
    }

    #[inline]
    pub fn with_text(mut self, text: impl Into<crate::components::Text>) -> Self {
        self.text = Some(text.into());
        self
    }
}
