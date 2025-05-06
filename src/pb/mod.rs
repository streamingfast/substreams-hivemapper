// @generated
pub mod hivemapper {
    pub mod types {
        // @@protoc_insertion_point(attribute:hivemapper.types.v2)
        pub mod v2 {
            include!("hivemapper.types.v2.rs");
            // @@protoc_insertion_point(hivemapper.types.v2)
        }
    }
}
// @@protoc_insertion_point(attribute:schema)
pub mod schema {
    include!("schema.rs");
    // @@protoc_insertion_point(schema)
}
pub mod sf {
    pub mod solana {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.solana.type.v1)
            pub mod v1 {
                include!("sf.solana.type.v1.rs");
                // @@protoc_insertion_point(sf.solana.type.v1)
            }
        }
    }
    pub mod substreams {
        pub mod sink {
            pub mod sql {
                pub mod schema {
                    // @@protoc_insertion_point(attribute:sf.substreams.sink.sql.schema.v1)
                    pub mod v1 {
                        include!("sf.substreams.sink.sql.schema.v1.rs");
                        // @@protoc_insertion_point(sf.substreams.sink.sql.schema.v1)
                    }
                }
            }
        }
    }
}
pub mod sol {
    pub mod instructions {
        // @@protoc_insertion_point(attribute:sol.instructions.v1)
        pub mod v1 {
            include!("sol.instructions.v1.rs");
            // @@protoc_insertion_point(sol.instructions.v1)
        }
    }
    pub mod transactions {
        // @@protoc_insertion_point(attribute:sol.transactions.v1)
        pub mod v1 {
            include!("sol.transactions.v1.rs");
            // @@protoc_insertion_point(sol.transactions.v1)
        }
    }
}
pub mod test {
    // @@protoc_insertion_point(attribute:test.relations)
    pub mod relations {
        include!("test.relations.rs");
        // @@protoc_insertion_point(test.relations)
    }
}
