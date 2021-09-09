use rand::seq::SliceRandom;
use rand::Rng;

/// A type implementing the `Random` trait should be able to generate random instances of itself
pub trait Random {
    type Config;
    fn random<R: Rng + ?Sized>(rng: &mut R, config: &Self::Config) -> Self;
}

#[derive(Debug)]
pub struct Tree<V> {
    pub value: V,
    pub children: Vec<Tree<V>>,
}

impl<V> Tree<V>
where
    V: Clone + Default,
{
    pub fn default() -> Self {
        Tree {
            value: V::default(),
            children: Vec::default(),
        }
    }

    pub fn map_nodes(&self, func: &dyn Fn(&V) -> V) -> Self {
        match self {
            Tree { value, children } => Tree {
                value: func(value),
                children: children
                    .into_iter()
                    .map(|child| child.map_nodes(func))
                    .collect(),
            },
        }
    }
}

/// Cofiguration struct for generating random trees
#[derive(Clone, Copy)]
pub struct RandomTreeConfig {
    pub depth: i32,
    pub min_children: i32,
    pub max_children: i32,
    pub max_offset: f32,
}

impl RandomTreeConfig {
    /// Decrements the depth of the config, called once per recursive call to random(..)
    fn decrement_depth(self) -> Self {
        if self.depth == 0 {
            self
        } else {
            RandomTreeConfig {
                depth: self.depth - 1,
                ..self
            }
        }
    }
}

impl<V> Random for Tree<V>
where
    V: Random<Config = RandomTreeConfig>,
{
    type Config = RandomTreeConfig;

    fn random<R: Rng + ?Sized>(rng: &mut R, config: &Self::Config) -> Self {
        if config.depth > 0 {
            let num_children = rng.gen_range(config.min_children..=config.max_children);
            // let num_children = (config.depth)
            //     .max(config.min_children)
            //     .min(config.max_children);
            Tree {
                value: V::random(rng, config),
                children: (0..num_children)
                    .map(|_| Self::random(rng, &config.decrement_depth()))
                    .collect(),
            }
        } else {
            Tree {
                value: V::random(rng, config),
                children: Vec::default(),
            }
        }
    }
}

impl Random for f32 {
    type Config = RandomTreeConfig;

    fn random<R: Rng + ?Sized>(rng: &mut R, config: &Self::Config) -> Self {
        let values = [-3., -2., -1., 1., 2., 3.];
        *values.choose(rng).unwrap()
        // rng.gen_range(-config.max_offset..=config.max_offset)
    }
}
