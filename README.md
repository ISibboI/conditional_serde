# Conditional Serde

Depend on [`Serialize`](serde::ser::Serialize) and [`Deserialize`](serde::de::Deserialize) only if the `serde` feature is activated.

Ever had to duplicate a full trait definition, just because you wanted some of its type members to conveniently implement [`Serialize`](serde::ser::Serialize) and [`Deserialize`](serde::de::Deserialize) depending on a feature?
Worry no more, this crate provides the solution!
Simply add a dependency to [`ConditionalSerde`] to your type member, and activate this crate's `serde` feature if yours is activated.
