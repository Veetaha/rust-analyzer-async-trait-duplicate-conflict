use async_trait::async_trait;
use duplicate::duplicate_item;

#[async_trait]
pub trait Foo {
    #[duplicate_item(
        var; [foo];
    )]
    async fn foo(&self);
}

pub fn foo(val: &dyn Foo) {


    let _ = val.foo();
}
