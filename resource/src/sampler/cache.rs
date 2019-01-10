//! A cache to store and retrieve samplers
use gfx_hal::device::Device;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use super::{Sampler, Info};

#[doc(hidden)]
#[derive(Debug)]
pub struct SamplerCache<B: gfx_hal::Backend> {
    samplers: HashMap<gfx_hal::image::Filter, HashMap<gfx_hal::image::WrapMode, Sampler<B>>>,
}

impl<B> SamplerCache<B>
where
    B: gfx_hal::Backend,
{
    #[doc(hidden)]
    pub fn get(
        &mut self,
        device: &B::Device,
        filter: gfx_hal::image::Filter,
        wrap_mode: gfx_hal::image::WrapMode
    ) -> Option<&Sampler<B>> {
        match self.samplers.entry(filter) {
            Entry::Occupied(e) => {
                let hashmap = &mut *e.into_mut();
                match hashmap.entry(wrap_mode) {
                    Entry::Occupied(e) => Some(&mut *e.into_mut()),
                    Entry::Vacant(e) => {
                        Some(&*e.insert(SamplerCache::create(device, filter, wrap_mode)))
                    }
                }
            },
            Entry::Vacant(_e) => None,
        }
    }

    fn create(
        device: &B::Device,
        filter: gfx_hal::image::Filter,
        wrap_mode: gfx_hal::image::WrapMode
    ) -> Sampler<B> {
        let sampler = unsafe {
            device.create_sampler(gfx_hal::image::SamplerInfo::new(filter, wrap_mode)).unwrap()
        };
        Sampler::new(
            Info {
                filter,
                wrap_mode,
            },
            sampler
        )
    }
}

impl<B> Default for SamplerCache<B>
where
    B: gfx_hal::Backend,
{
    fn default() -> Self {
        let mut samplers = HashMap::new();
        samplers.insert(gfx_hal::image::Filter::Linear, HashMap::new());
        samplers.insert(gfx_hal::image::Filter::Nearest, HashMap::new());
        SamplerCache {
            samplers,
        }
    }
}