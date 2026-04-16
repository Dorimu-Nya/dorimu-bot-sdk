use qqbot_sdk::ContextStore;

pub struct TestContextA;

#[test]
fn test_context_store() {
    let store = ContextStore::new();
    store.insert(TestContextA);
    store.get::<TestContextA>();
}
