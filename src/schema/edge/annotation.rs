use crate::schema::schema;

#[derive(Debug, Default, Clone)]
pub struct Annotation {
    // The `struct_tag` option allows overriding the struct-tag
    // of the `Edges` field in the generated entity. For example:
    //
    //	edge.Annotation{
    //		StructTag: `json:"pet_edges"`
    //	}
    //
    struct_tag: String,
}

impl schema::Annotation for Annotation {
    // Name defines the name of the annotation to be retrieved by the codegen.
    fn name(&self) -> String {
        String::from("Edges")
    }
}

impl schema::Merger for Annotation {
    fn merge(&self, other: Self) -> Self {
        Annotation {
            struct_tag: other.struct_tag,
        }
    }
}
