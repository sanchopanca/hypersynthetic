use crate::HtmlFragment;

pub trait Component<P> {
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

pub trait Props {
    type Builder;

    fn builder() -> Self::Builder;
}

pub trait PropsOrNoPropsBuilder {
    type Builder;

    fn builder_or_not() -> Self::Builder;
}

impl<P: Props> PropsOrNoPropsBuilder for P {
    type Builder = P::Builder;

    fn builder_or_not() -> Self::Builder {
        P::builder()
    }
}

pub fn component_props_builder<P: PropsOrNoPropsBuilder>(
    _f: &impl Component<P>,
) -> <P as PropsOrNoPropsBuilder>::Builder {
    <P as PropsOrNoPropsBuilder>::builder_or_not()
}

pub fn component_view<P: Props>(component: &impl Component<P>, props: P) -> HtmlFragment {
    component.call(props)
}
