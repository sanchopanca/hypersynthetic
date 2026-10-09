//! Traits and functions used by the code that [html!](crate::html) and
//! [#\[component\]](macro@crate::component) generate. They aren't meant to be used directly.

use crate::HtmlFragment;

/// A component function: takes its props and returns HTML.
pub trait Component<P> {
    /// Renders the component.
    fn call(&self, props: P) -> HtmlFragment;
}

impl<P, F> Component<P> for F
where
    F: Fn(P) -> HtmlFragment,
    P: Props,
{
    fn call(&self, props: P) -> HtmlFragment {
        self(props)
    }
}

/// The props struct generated for a component.
pub trait Props {
    /// The builder that `html!` uses to set the props.
    type Builder;

    /// A builder with no props set.
    fn builder() -> Self::Builder;
}

/// Same as [Props]; implemented for every [Props] type.
pub trait PropsOrNoPropsBuilder {
    /// The props builder.
    type Builder;

    /// A builder with no props set.
    fn builder_or_not() -> Self::Builder;
}

impl<P: Props> PropsOrNoPropsBuilder for P {
    type Builder = P::Builder;

    fn builder_or_not() -> Self::Builder {
        P::builder()
    }
}

/// The props builder of a component, found through the component function's type.
pub fn component_props_builder<P: PropsOrNoPropsBuilder>(
    _f: &impl Component<P>,
) -> <P as PropsOrNoPropsBuilder>::Builder {
    <P as PropsOrNoPropsBuilder>::builder_or_not()
}

/// Calls a component with its props.
pub fn component_view<P: Props>(component: &impl Component<P>, props: P) -> HtmlFragment {
    component.call(props)
}
