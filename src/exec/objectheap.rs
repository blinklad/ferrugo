use super::super::class::{class::Class, classheap::ClassHeap};
use super::super::exec::vm::load_class;
use super::super::gc::{gc, gc::GcType};
use super::frame::{ObjectBody, Variable};
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct ObjectHeap {
    // TODO: Add fields for support of GC
}

impl ObjectHeap {
    pub fn new() -> ObjectHeap {
        ObjectHeap {}
    }

    pub fn create_object(&mut self, class: GcType<Class>) -> Variable {
        let class_field_count = unsafe { &*class }.get_object_field_count();
        let obj = gc::new(ObjectBody {
            class: Variable::Pointer(class as *mut u64),
            variables: {
                let mut vars = FxHashMap::default();
                vars.reserve(class_field_count);
                vars
            },
        });

        Variable::Object(obj)
    }

    pub fn create_string_object(
        &mut self,
        string: String,
        classheap: GcType<ClassHeap>,
    ) -> Variable {
        let class = load_class(classheap, self, "java/lang/String");
        let object = self.create_object(class);

        unsafe { &mut *object.get_object() }.variables.insert(
            "str".to_string(),
            Variable::Pointer(Box::into_raw(Box::new(string)) as GcType<u64>),
        );

        object
    }
}
