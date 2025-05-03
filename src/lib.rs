//! Depend on [`Serialize`](serde::ser::Serialize) and [`Deserialize`](serde::de::Deserialize) only if the `serde` feature is activated.

/// If the serde feature is activated, this trait depends on [`Serialize`](serde::ser::Serialize) and [`Deserialize`](serde::de::Deserialize).
#[cfg(not(feature = "serde"))]
pub trait ConditionalSerde {}

/// If the serde feature is activated, this trait depends on [`Serialize`](serde::ser::Serialize) and [`Deserialize`](serde::de::Deserialize).
#[cfg(feature = "serde")]
pub trait ConditionalSerde: serde::ser::Serialize + for<'de> serde::de::Deserialize<'de> {}

#[cfg(all(feature = "autotrait", feature = "serde"))]
impl<T: serde::ser::Serialize + for<'de> serde::de::Deserialize<'de>> ConditionalSerde for T {}

#[cfg(all(feature = "autotrait", not(feature = "serde")))]
impl<T> ConditionalSerde for T {}

#[cfg(test)]
mod tests {
    use crate::ConditionalSerde;

    #[allow(dead_code)]
    trait Trait {
        type Type: ConditionalSerde;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    struct Struct;

    #[cfg(not(feature = "autotrait"))]
    impl ConditionalSerde for Struct {}

    impl Trait for i32 {
        type Type = Struct;
    }
}
