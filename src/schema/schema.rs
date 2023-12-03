// Annotation is used to attach arbitrary metadata to the schema objects in codegen.
// The object must be serializable to JSON raw value (e.g. struct, map or slice).
//
// Template extensions can retrieve this metadata and use it inside their templates.
// Read more about it in ent website: https://entgo.io/docs/templates/#annotations.
pub trait Annotation: Sized {
    // Name defines the name of the annotation to be retrieved by the codegen.
    fn name(&self) -> String;
}

// Merger wraps the single Merge function allows custom annotation to provide
// an implementation for merging 2 or more annotations from the same type.
//
// A common use case is where the same Annotation type is defined both in
// mixin.Schema and ent.Schema.
pub trait Merger {
    fn merge(&self, other: Self) -> Self;
}

// CommentAnnotation is a builtin schema annotation for
// configuring the schema's Godoc comment.
#[derive(Debug, Default, Clone)]
pub struct CommentAnnotation {
    text: String, // Comment text.
}

impl Annotation for CommentAnnotation {
    fn name(&self) -> String {
        String::from("Comment")
    }
}

impl CommentAnnotation {
    // Text returns the comment text.
    pub fn new(text: String) -> CommentAnnotation {
        CommentAnnotation { text }
    }
}
