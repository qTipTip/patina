// Light as carried along rays: the radiance a sample contributes, accumulated by the film and
// scaled by BSDFs and lights. Used throughout the renderer, so it has its own module rather than
// living in any one of its users.
pub struct SampledSpectrum {}
