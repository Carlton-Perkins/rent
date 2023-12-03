use std::any::type_name;

use crate::schema::edge;

#[derive(Debug, Default, Clone)]
struct DescriptorThrough {
    name: String,
    type_: String,
}

// A Descriptor for edge configuration.
#[derive(Debug, Default, Clone)]
struct Descriptor {
    tag: String,                        // struct tag.
    type_: String,                      // edge type.
    name: String,                       // edge name.
    field: String,                      // edge field name (e.g. foreign-key).
    ref_name: String,                   // ref name; inverse only.
    ref_: Option<Box<Descriptor>>,      // edge reference; to/from of the same type.
    through: Option<DescriptorThrough>, // through type and name.
    unique: bool,                       // unique edge.
    inverse: bool,                      // inverse edge.
    required: bool,                     // required on creation.
    immutable: bool,                    // create only edge.
    storage_key: Option<StorageKey>,    // optional storage-key configuration.
    annotations: Vec<edge::Annotation>, // edge annotations.
    comment: String,                    // edge comment.
}

// To defines an association edge between two vertices.
pub fn to<T>(name: String) -> AssocBuilder {
    return AssocBuilder {
        desc: Descriptor {
            name: name,
            type_: typ::<T>(),
            ..Default::default()
        },
    };
}

// From represents a reversed-edge between two vertices that has a back-reference to its source edge.
pub fn from<T>(name: String) -> InverseBuilder {
    return InverseBuilder {
        desc: Descriptor {
            name: name,
            type_: typ::<T>(),
            inverse: true,
            ..Default::default()
        },
    };
}

fn typ<T>() -> String {
    type_name::<T>().rsplit("::").next().unwrap().to_string()
}

// assocBuilder is the builder for assoc edges.
#[derive(Debug, Default, Clone)]
pub struct AssocBuilder {
    desc: Descriptor,
}

impl AssocBuilder {
    // unique sets the edge type to be unique. Basically, it limits the edge to be one of the two:
    // one2one or one2many. one2one applied if the inverse-edge is also unique.
    fn unique(mut self) -> Self {
        self.desc.unique = true;
        self
    }

    // required indicates that this edge is a required field on creation.
    // Unlike fields, edges are optional by default.
    fn required(mut self) -> Self {
        self.desc.required = true;
        self
    }

    // immutable indicates that this edge cannot be updated.
    fn immutable(mut self) -> Self {
        self.desc.immutable = true;
        self
    }

    // StructTag sets the struct tag of the assoc edge.
    fn struct_tag(mut self, s: String) -> Self {
        self.desc.tag = s;
        self
    }

    // From creates an inverse-edge with the same type.
    fn from(self, name: String) -> InverseBuilder {
        InverseBuilder {
            desc: Descriptor {
                name: name,
                type_: self.desc.type_.clone(),
                inverse: true,
                ref_: Some(Box::new(self.desc)),
                ..Default::default()
            },
        }
    }

    // Field is used to bind an edge (with a foreign-key) to a field in the schema.
    //
    //	field.Int("owner_id").
    //		Optional()
    //
    //	edge.To("owner", User.Type).
    //		Field("owner_id").
    //		Unique(),
    fn field(mut self, f: String) -> Self {
        self.desc.field = f;
        self
    }

    // Through allows setting an "edge schema" to interact explicitly with M2M edges.
    //
    //	edge.To("friends", User.Type).
    //		Through("friendships", Friendship.Type)
    fn through<T>(mut self, name: String) -> Self {
        self.desc.through = Some(DescriptorThrough {
            name,
            type_: typ::<T>(),
        });
        self
    }

    // Comment used to put annotations on the schema.
    fn comment(mut self, c: String) -> Self {
        self.desc.comment = c;
        self
    }

    // StorageKey sets the storage key of the edge.
    //
    //	edge.To("groups", Group.Type).
    //		StorageKey(edge.Table("user_groups"), edge.Columns("user_id", "group_id"))
    fn storage_key(mut self, opts: &[StorageOption]) -> Self {
        let mut storage_key = match self.desc.storage_key {
            Some(storage_key) => storage_key,
            None => StorageKey::default(),
        };

        for option in opts {
            option(&mut storage_key);
        }

        self.desc.storage_key = Some(storage_key);
        return self;
    }

    // Annotations adds a list of annotations to the edge object to be used by
    // codegen extensions.
    //
    //	edge.To("pets", Pet.Type).
    //		Annotations(entgql.Bind())
    fn annotations(mut self, annotations: Vec<edge::Annotation>) -> Self {
        self.desc.annotations.extend(annotations);
        self
    }

    // Descriptor implements the ent.Descriptor interface.
    fn descriptor(self) -> Descriptor {
        return self.desc;
    }
}

// inverseBuilder is the builder for inverse edges.
#[derive(Debug, Default, Clone)]
pub struct InverseBuilder {
    desc: Descriptor,
}

impl InverseBuilder {
    // Ref sets the referenced-edge of this inverse edge.
    fn ref_(mut self, ref_: String) -> Self {
        self.desc.ref_name = ref_;
        self
    }

    // Unique sets the edge type to be unique. Basically, it limits the edge to be one of the two:
    // one-2-one or one-2-many. one-2-one applied if the inverse-edge is also unique.
    fn unique(mut self) -> Self {
        self.desc.unique = true;
        self
    }

    // Required indicates that this edge is a required field on creation.
    // Unlike fields, edges are optional by default.
    fn required(mut self) -> Self {
        self.desc.required = true;
        self
    }

    // Immutable indicates that this edge cannot be updated.
    fn immutable(mut self) -> Self {
        self.desc.immutable = true;
        self
    }

    // StructTag sets the struct tag of the inverse edge.
    fn struct_tag(mut self, s: String) -> Self {
        self.desc.tag = s;
        self
    }

    // Comment used to put annotations on the schema.
    fn comment(mut self, c: String) -> Self {
        self.desc.comment = c;
        self
    }

    // Field is used to bind an edge (with a foreign-key) to a field in the schema.
    //
    //	field.Int("owner_id").
    //		Optional()
    //
    //	edge.From("owner", User.Type).
    //		Ref("pets").
    //		Field("owner_id").
    //		Unique(),
    fn field(mut self, f: String) -> Self {
        self.desc.field = f;
        self
    }

    // Through allows setting an "edge schema" to interact explicitly with M2M edges.
    //
    //	edge.From("liked_users", User.Type).
    //		Ref("liked_tweets").
    //		Through("likes", TweetLike.Type)
    fn through<T>(mut self, name: String) -> Self {
        self.desc.through = Some(DescriptorThrough {
            name,
            type_: typ::<T>(),
        });
        self
    }

    // Annotations adds a list of annotations to the edge object to be used by
    // codegen extensions.
    //
    //	edge.From("owner", User.Type).
    //		Ref("pets").
    //		Unique().
    //		Annotations(entgql.Bind())
    fn annotations(mut self, annotations: Vec<edge::Annotation>) -> Self {
        self.desc.annotations.extend(annotations);
        self
    }

    // Descriptor implements the ent.Descriptor interface.
    fn descriptor(self) -> Descriptor {
        self.desc
    }
}

// StorageKey holds the configuration for edge storage-key.
#[derive(Debug, Default, Clone)]
pub struct StorageKey {
    table: String,        // Table or label.
    symbols: Vec<String>, // Symbols/names of the foreign-key constraints.
    columns: Vec<String>, // Foreign-key columns.
}

// StorageOption allows for setting the storage configuration using functional options.
pub type StorageOption = Box<dyn Fn(&mut StorageKey)>;

// Table sets the table name option for M2M edges.
pub fn table(name: String) -> StorageOption {
    Box::new(move |key: &mut StorageKey| {
        key.table = name.clone();
    })
}

// Symbol sets the symbol/name of the foreign-key constraint for O2O, O2M and M2O edges.
// Note that, for M2M edges (2 columns and 2 constraints), use the edge.Symbols option.
pub fn symbol(symbol: String) -> StorageOption {
    Box::new(move |key: &mut StorageKey| {
        key.symbols = vec![symbol.clone()];
    })
}

// Symbols sets the symbol/name of the foreign-key constraints for M2M edges.
// The 1st column defines the name of the "To" edge, and the 2nd defines
// the name of the "From" edge (inverse edge).
// Note that, for O2O, O2M and M2O edges, use the edge.Symbol option.
pub fn symbols(to: String, from: String) -> StorageOption {
    Box::new(move |key: &mut StorageKey| {
        key.symbols = vec![to.clone(), from.clone()];
    })
}

// Column sets the foreign-key column name option for O2O, O2M and M2O edges.
// Note that, for M2M edges (2 columns), use the edge.Columns option.
pub fn column(name: String) -> StorageOption {
    Box::new(move |key: &mut StorageKey| {
        key.columns = vec![name.clone()];
    })
}

// Columns sets the foreign-key column names option for M2M edges.
// The 1st column defines the name of the "To" edge, and the 2nd defines
// the name of the "From" edge (inverse edge).
// Note that, for O2O, O2M and M2O edges, use the edge.Column option.
pub fn columns(to: String, from: String) -> StorageOption {
    Box::new(move |key: &mut StorageKey| {
        key.columns = vec![to.clone(), from.clone()];
    })
}

#[cfg(test)]
mod unit_tests {
    use crate::schema::edge::edge;
    struct User {}
    struct Node {}

    #[test]
    fn basic_edge() {
        let user_edge = edge::to::<User>(String::from("friends"))
            .required()
            .comment(String::from("comment"))
            .descriptor();
        assert_eq!(user_edge.inverse, false);
        assert_eq!(user_edge.comment, String::from("comment"));
        assert_eq!(user_edge.type_, String::from("User"));
        assert_eq!(user_edge.name, String::from("friends"));
        assert_eq!(user_edge.required, true);
    }

    #[test]
    fn edge_with_children() {
        let node_edge = edge::to::<Node>(String::from("parent"))
            .unique()
            .immutable()
            .descriptor();

        assert_eq!(node_edge.inverse, false);
        assert_eq!(node_edge.unique, true);
        assert_eq!(node_edge.type_, String::from("Node"));
        assert_eq!(node_edge.name, String::from("parent"));
        assert_eq!(node_edge.required, false);
        assert_eq!(node_edge.immutable, true);

        let children_edge = edge::to::<Node>(String::from("children"))
            .from(String::from("parent"))
            .unique()
            .comment(String::from("comment"))
            .field(String::from("parent_id"))
            .descriptor();

        assert_eq!(children_edge.field, String::from("parent_id"));
        assert_eq!(children_edge.comment, String::from("comment"));
        assert_eq!(children_edge.ref_.unwrap().field, String::from(""));
    }

    #[test]
    fn m2m_relation_of_same_type() {
        let m2m_edge = edge::to::<User>(String::from("following"))
            .from(String::from("followers"))
            .descriptor();

        assert_eq!(m2m_edge.inverse, true);
        assert_eq!(m2m_edge.unique, false);
        assert_eq!(m2m_edge.name, "followers");
        assert_eq!(m2m_edge.ref_.is_none(), false);
        assert_eq!(m2m_edge.ref_.unwrap().name, "following");
        assert_eq!(m2m_edge.unique, false);
    }

    #[test]
    fn o2m_relation_of_same_type() {
        let m2o_edge = edge::to::<User>(String::from("following"))
            .unique()
            .from(String::from("followers"))
            .descriptor();

        assert_eq!(m2o_edge.unique, false);
        assert_eq!(m2o_edge.ref_.unwrap().unique, true);

        let o2m_edge = edge::to::<User>(String::from("following"))
            .from(String::from("followers"))
            .unique()
            .descriptor();

        assert_eq!(o2m_edge.unique, true);
        assert_eq!(o2m_edge.ref_.unwrap().unique, false);
    }

    #[test]
    fn o2o_relation_of_same_type() {
        let o2o_edge = edge::to::<User>(String::from("following"))
            .unique()
            .from(String::from("followers"))
            .unique()
            .descriptor();

        assert_eq!(o2o_edge.unique, true);
        assert_eq!(o2o_edge.ref_.unwrap().unique, true);
    }

    #[test]
    fn edge_with_struct_tag() {
        let edge = edge::to::<User>(String::from("friends"))
            .struct_tag(String::from("json:\"user_name,omitempty\""))
            .descriptor();

        assert_eq!(edge.tag, String::from("json:\"user_name,omitempty\""));
    }

    #[test]
    fn edge_with_storage_key() {
        let edge = edge::to::<User>(String::from("following"))
            .struct_tag(String::from("following"))
            .storage_key(&[
                edge::table(String::from("user_followers")),
                edge::columns(String::from("following_id"), String::from("followers_id")),
                edge::symbols(
                    String::from("users_followers"),
                    String::from("users_followers"),
                ),
            ])
            .from(String::from("followers"))
            .struct_tag(String::from("followers"))
            .descriptor();

        assert_eq!(edge.tag, String::from("followers"));
        assert_eq!(edge.ref_.clone().unwrap().tag, String::from("following"));
        assert_eq!(edge.ref_.clone().unwrap().storage_key.is_some(), true);
        assert_eq!(
            edge.ref_
                .clone()
                .unwrap()
                .storage_key
                .clone()
                .unwrap()
                .table,
            String::from("user_followers")
        );
        assert_eq!(
            edge.ref_
                .clone()
                .unwrap()
                .storage_key
                .clone()
                .unwrap()
                .columns,
            vec![String::from("following_id"), String::from("followers_id")]
        );
        assert_eq!(
            edge.ref_
                .clone()
                .unwrap()
                .storage_key
                .clone()
                .unwrap()
                .symbols,
            vec![
                String::from("users_followers"),
                String::from("users_followers")
            ]
        );
    }
}
